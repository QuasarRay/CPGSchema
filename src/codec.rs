//! Optional CPGSchemaGraph v1 codec. Independent of the JSON Schema dialect package.
use crate::{
    graph::{ContainedValue, Edge, EdgeId, Graph, Node, NodeId},
    model::{Error, Result},
    value::{JavaString, PropertyValue, Scalar},
};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
fn object<'a>(v: &'a Value, path: &str) -> Result<&'a Map<String, Value>> {
    v.as_object()
        .ok_or_else(|| Error::new(path, "expected object"))
}
fn get<'a>(o: &'a Map<String, Value>, name: &str, path: &str) -> Result<&'a Value> {
    o.get(name)
        .ok_or_else(|| Error::new(path, format!("missing {name}")))
}
fn text<'a>(v: &'a Value, path: &str) -> Result<&'a str> {
    v.as_str()
        .ok_or_else(|| Error::new(path, "expected string"))
}
fn fields(o: &Map<String, Value>, allowed: &[&str], path: &str) -> Result<()> {
    for k in o.keys() {
        if !allowed.contains(&k.as_str()) {
            return Err(Error::new(path, format!("unknown field {k}")));
        }
    }
    Ok(())
}
fn decimal<T: std::str::FromStr + ToString>(v: &Value, path: &str) -> Result<T> {
    let s = text(v, path)?;
    let n: T = s
        .parse()
        .map_err(|_| Error::new(path, "integer is outside its signed domain"))?;
    if n.to_string() != s {
        return Err(Error::new(path, "expected canonical signed decimal"));
    }
    Ok(n)
}
fn hex(v: &Value, width: usize, path: &str) -> Result<u64> {
    let s = text(v, path)?;
    if s.len() != width
        || !s
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(Error::new(
            path,
            "expected fixed-width lowercase hexadecimal bits",
        ));
    }
    u64::from_str_radix(s, 16).map_err(|_| Error::new(path, "invalid floating-point bits"))
}
pub fn decode_scalar(v: &Value, path: &str) -> Result<Scalar> {
    let o = object(v, path)?;
    let ty = text(get(o, "type", path)?, path)?;
    if ty == "String" {
        fields(o, &["type", "value", "utf16"], path)?;
        return match (o.get("value"), o.get("utf16")) {
            (Some(v), None) => Ok(Scalar::String(text(v, path)?.into())),
            (None, Some(v)) => {
                let units = v
                    .as_array()
                    .ok_or_else(|| Error::new(path, "utf16 must be an array"))?
                    .iter()
                    .map(|v| {
                        v.as_u64()
                            .and_then(|n| u16::try_from(n).ok())
                            .ok_or_else(|| Error::new(path, "invalid UTF-16 code unit"))
                    })
                    .collect::<Result<Vec<_>>>()?;
                Ok(Scalar::String(JavaString(units)))
            }
            _ => Err(Error::new(
                path,
                "String requires exactly one of value and utf16",
            )),
        };
    }
    if matches!(ty, "Float" | "Double") {
        fields(o, &["type", "bits"], path)?;
        let bits = hex(
            get(o, "bits", path)?,
            if ty == "Float" { 8 } else { 16 },
            path,
        )?;
        return Ok(if ty == "Float" {
            Scalar::Float(bits as u32)
        } else {
            Scalar::Double(bits)
        });
    }
    fields(o, &["type", "value"], path)?;
    let value = get(o, "value", path)?;
    Ok(match ty {
        "Boolean" => Scalar::Boolean(
            value
                .as_bool()
                .ok_or_else(|| Error::new(path, "expected boolean"))?,
        ),
        "Byte" => Scalar::Byte(decimal(value, path)?),
        "Short" => Scalar::Short(decimal(value, path)?),
        "Int" => Scalar::Int(decimal(value, path)?),
        "Long" => Scalar::Long(decimal(value, path)?),
        "Char" => Scalar::Char(
            value
                .as_u64()
                .and_then(|n| u16::try_from(n).ok())
                .ok_or_else(|| Error::new(path, "Char must be a UTF-16 code unit"))?,
        ),
        "NodeRef" => Scalar::NodeRef(text(value, path)?.into()),
        "List" => Scalar::List(
            value
                .as_array()
                .ok_or_else(|| Error::new(path, "List scalar requires an array"))?
                .iter()
                .enumerate()
                .map(|(i, v)| decode_scalar(v, &format!("{path}/value/{i}")))
                .collect::<Result<Vec<_>>>()?,
        ),
        _ => return Err(Error::new(path, format!("unknown scalar type {ty}"))),
    })
}
pub fn encode_scalar(v: &Scalar) -> Value {
    match v {
        Scalar::Boolean(v) => json!({"type":"Boolean","value":v}),
        Scalar::String(v) => match v.to_utf8() {
            Ok(s) => json!({"type":"String","value":s}),
            Err(_) => json!({"type":"String","utf16":v.units()}),
        },
        Scalar::Byte(v) => json!({"type":"Byte","value":v.to_string()}),
        Scalar::Short(v) => json!({"type":"Short","value":v.to_string()}),
        Scalar::Int(v) => json!({"type":"Int","value":v.to_string()}),
        Scalar::Long(v) => json!({"type":"Long","value":v.to_string()}),
        Scalar::Float(v) => json!({"type":"Float","bits":format!("{v:08x}")}),
        Scalar::Double(v) => json!({"type":"Double","bits":format!("{v:016x}")}),
        Scalar::Char(v) => json!({"type":"Char","value":v}),
        Scalar::NodeRef(v) => json!({"type":"NodeRef","value":v}),
        Scalar::List(v) => {
            json!({"type":"List","value":v.iter().map(encode_scalar).collect::<Vec<_>>()})
        }
    }
}
fn decode_properties(v: &Value, path: &str) -> Result<BTreeMap<String, PropertyValue>> {
    object(v, path)?
        .iter()
        .map(|(k, v)| {
            let p = format!("{path}/{k}");
            let v = if let Some(a) = v.as_array() {
                PropertyValue::List(
                    a.iter()
                        .enumerate()
                        .map(|(i, v)| decode_scalar(v, &format!("{p}/{i}")))
                        .collect::<Result<Vec<_>>>()?,
                )
            } else {
                PropertyValue::Scalar(decode_scalar(v, &p)?)
            };
            Ok((k.clone(), v))
        })
        .collect()
}
fn encode_properties(p: &BTreeMap<String, PropertyValue>) -> Value {
    Value::Object(
        p.iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    match v {
                        PropertyValue::Scalar(v) => encode_scalar(v),
                        PropertyValue::List(v) => {
                            Value::Array(v.iter().map(encode_scalar).collect())
                        }
                    },
                )
            })
            .collect(),
    )
}
fn decode_contained(v: Option<&Value>, path: &str) -> Result<BTreeMap<String, ContainedValue>> {
    let Some(v) = v else {
        return Ok(BTreeMap::new());
    };
    object(v, path)?
        .iter()
        .map(|(k, v)| {
            let value = if v.is_null() {
                ContainedValue::One(None)
            } else if let Some(a) = v.as_array() {
                ContainedValue::List(
                    a.iter()
                        .map(|v| text(v, path).map(|s| NodeId(s.into())))
                        .collect::<Result<Vec<_>>>()?,
                )
            } else {
                ContainedValue::One(Some(NodeId(text(v, path)?.into())))
            };
            Ok((k.clone(), value))
        })
        .collect()
}
pub fn decode_graph(v: &Value) -> Result<Graph> {
    let o = object(v, "")?;
    fields(
        o,
        &[
            "format",
            "formatVersion",
            "schemaRevision",
            "nodes",
            "edges",
        ],
        "",
    )?;
    if text(get(o, "format", "")?, "/format")? != "CPGSchemaGraph" {
        return Err(Error::new("/format", "expected CPGSchemaGraph"));
    }
    let version = get(o, "formatVersion", "")?
        .as_u64()
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| Error::new("/formatVersion", "expected unsigned version"))?;
    let revision = text(get(o, "schemaRevision", "")?, "/schemaRevision")?.into();
    let ns = get(o, "nodes", "")?
        .as_array()
        .ok_or_else(|| Error::new("/nodes", "expected array"))?;
    let mut nodes = vec![];
    for (i, v) in ns.iter().enumerate() {
        let p = format!("/nodes/{i}");
        let o = object(v, &p)?;
        fields(o, &["id", "label", "properties", "contained"], &p)?;
        nodes.push(Node {
            id: NodeId(text(get(o, "id", &p)?, &p)?.into()),
            label: text(get(o, "label", &p)?, &p)?.into(),
            properties: decode_properties(get(o, "properties", &p)?, &format!("{p}/properties"))?,
            contained: decode_contained(o.get("contained"), &format!("{p}/contained"))?,
        });
    }
    let es = get(o, "edges", "")?
        .as_array()
        .ok_or_else(|| Error::new("/edges", "expected array"))?;
    let mut edges = vec![];
    for (i, v) in es.iter().enumerate() {
        let p = format!("/edges/{i}");
        let o = object(v, &p)?;
        fields(o, &["id", "source", "target", "label", "properties"], &p)?;
        edges.push(Edge {
            id: EdgeId(text(get(o, "id", &p)?, &p)?.into()),
            source: NodeId(text(get(o, "source", &p)?, &p)?.into()),
            target: NodeId(text(get(o, "target", &p)?, &p)?.into()),
            label: text(get(o, "label", &p)?, &p)?.into(),
            properties: decode_properties(get(o, "properties", &p)?, &format!("{p}/properties"))?,
        });
    }
    Ok(Graph {
        format_version: version,
        schema_revision: revision,
        nodes,
        edges,
    })
}
pub fn encode_graph(graph: &Graph) -> Value {
    json!({"format":"CPGSchemaGraph","formatVersion":graph.format_version,"schemaRevision":graph.schema_revision,
    "nodes":graph.nodes.iter().map(|n|{let contained:Map<String,Value>=n.contained.iter().map(|(k,v)|(k.clone(),match v{ContainedValue::One(None)=>Value::Null,ContainedValue::One(Some(id))=>Value::String(id.0.clone()),ContainedValue::List(ids)=>Value::Array(ids.iter().map(|v|Value::String(v.0.clone())).collect())})).collect();json!({"id":n.id.0,"label":n.label,"properties":encode_properties(&n.properties),"contained":contained})}).collect::<Vec<_>>(),
    "edges":graph.edges.iter().map(|e|json!({"id":e.id.0,"source":e.source.0,"target":e.target.0,"label":e.label,"properties":encode_properties(&e.properties)})).collect::<Vec<_>>()})
}
pub fn from_str(text: &str) -> Result<Graph> {
    let value = serde_json::from_str(text).map_err(|e| Error::new("", e.to_string()))?;
    decode_graph(&value)
}
pub fn to_string(graph: &Graph) -> Result<String> {
    serde_json::to_string_pretty(&encode_graph(graph)).map_err(|e| Error::new("", e.to_string()))
}
