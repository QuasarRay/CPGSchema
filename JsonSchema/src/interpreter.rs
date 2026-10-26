//! Independent ordered interpretation of declarative CPG schema documents.
use crate::{
    model::*,
    value::{Cardinality, EdgeCardinality, ValueType},
};
use serde_json::{Map, Value};
fn object<'a>(v: &'a Value, p: &str) -> Result<&'a Map<String, Value>> {
    v.as_object()
        .ok_or_else(|| Error::new(p, "expected object"))
}
fn field<'a>(o: &'a Map<String, Value>, k: &str, p: &str) -> Result<&'a Value> {
    o.get(k)
        .ok_or_else(|| Error::new(p, format!("missing {k}")))
}
fn string<'a>(o: &'a Map<String, Value>, k: &str, p: &str) -> Result<&'a str> {
    field(o, k, p)?
        .as_str()
        .ok_or_else(|| Error::new(p, format!("{k} must be a string")))
}
fn opt_string<'a>(o: &'a Map<String, Value>, k: &str, p: &str) -> Result<&'a str> {
    match o.get(k) {
        None => Ok(""),
        Some(v) => v
            .as_str()
            .ok_or_else(|| Error::new(p, format!("{k} must be a string"))),
    }
}
fn array<'a>(o: &'a Map<String, Value>, k: &str, p: &str) -> Result<&'a [Value]> {
    field(o, k, p)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| Error::new(p, format!("{k} must be an array")))
}
fn int(o: &Map<String, Value>, k: &str, p: &str) -> Result<i32> {
    (if let Some(name) = field(o, k, p)?.get("$protoId").and_then(Value::as_str) {
        return crate::ProtoIds::resolve(name);
    } else {
        field(o, k, p)?
    })
    .as_i64()
    .and_then(|n| i32::try_from(n).ok())
    .ok_or_else(|| Error::new(p, format!("{k} must be signed i32")))
}
fn edge_card(o: &Map<String, Value>, k: &str, p: &str) -> Result<EdgeCardinality> {
    Ok(match opt_string(o, k, p)? {
        "" | "List" => EdgeCardinality::List,
        "One" => EdgeCardinality::One,
        "ZeroOrOne" => EdgeCardinality::ZeroOrOne,
        _ => return Err(Error::new(p, format!("unknown {k}"))),
    })
}
fn refs(o: &Map<String, Value>, b: &SchemaBuilder, p: &str) -> Result<Vec<Binding>> {
    array(o, "references", p)?
        .iter()
        .map(|v| {
            b.binding(
                v.as_str()
                    .ok_or_else(|| Error::new(p, "reference must be a string"))?,
            )
        })
        .collect()
}
/// Mutations are ordered and resolve against this builder's independent symbol table.
pub fn apply_module(builder: &mut SchemaBuilder, document: &Value) -> Result<()> {
    crate::dialect::validate_module_syntax(document)?;
    let o = object(document, "module")?;
    let name = string(o, "cpg:module", "module")?;
    let deps = array(o, "cpg:imports", name)?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .ok_or_else(|| Error::new(name, "module import must be a string"))
        })
        .collect::<Result<Vec<_>>>()?;
    let provided = o
        .get("cpg:providedByFrontend")
        .map(|v| {
            v.as_bool()
                .ok_or_else(|| Error::new(name, "providedByFrontend must be boolean"))
        })
        .transpose()?
        .unwrap_or(false);
    builder.add_module(ModuleInfo {
        name: name.into(),
        doc_index: int(o, "cpg:docIndex", name)?,
        description: opt_string(o, "description", name)?.into(),
        provided_by_frontend: provided,
        dependencies: deps,
    })?;
    for (i, value) in array(o, "cpg:operations", name)?.iter().enumerate() {
        let path = format!("{name}/cpg:operations/{i}");
        let op = object(value, &path)?;
        let mut referenced = vec![];
        for key in [
            "target",
            "source",
            "edge",
            "destination",
            "reference",
            "node",
        ] {
            if let Some(v) = op.get(key).and_then(Value::as_str) {
                referenced.push(v);
            }
        }
        if let Some(v) = op.get("references").and_then(Value::as_array) {
            referenced.extend(v.iter().filter_map(Value::as_str));
        }
        for r in referenced {
            if r == "AnyNode" {
                continue;
            }
            let owner = r.split("::").next().unwrap_or("");
            if owner != name
                && !builder
                    .schema()
                    .modules
                    .last()
                    .is_some_and(|m| m.dependencies.iter().any(|d| d == owner))
            {
                return Err(Error::new(
                    &path,
                    format!("reference to unimported module {owner}"),
                ));
            }
        }
        let kind = string(op, "op", &path)?;
        let reference =
            |k: &str, b: &SchemaBuilder| -> Result<Binding> { b.binding(string(op, k, &path)?) };
        match kind {
            "property" | "node" | "nodeBase" | "edge" | "constants" => {
                let symbol = string(op, "id", &path)?;
                if !symbol.starts_with(&format!("{name}::")) {
                    return Err(Error::new(&path, "definition id must belong to its module"));
                }
                let binding = match kind {
                    "property" => Binding::Property(
                        builder.add_property(
                            name,
                            string(op, "name", &path)?,
                            ValueType::from_name(string(op, "valueType", &path)?)
                                .ok_or_else(|| Error::new(&path, "unknown value type"))?,
                            opt_string(op, "comment", &path)?,
                        )?,
                    ),
                    "node" | "nodeBase" => Binding::Node(builder.add_node(
                        name,
                        string(op, "name", &path)?,
                        if kind == "node" {
                            NodeKind::Concrete
                        } else {
                            NodeKind::Base
                        },
                        opt_string(op, "comment", &path)?,
                    )?),
                    "edge" => Binding::Edge(builder.add_edge(
                        name,
                        string(op, "name", &path)?,
                        opt_string(op, "comment", &path)?,
                    )?),
                    "constants" => {
                        let mut entries = vec![];
                        for v in array(op, "entries", &path)? {
                            let c = object(v, &path)?;
                            let mut entry = Constant::new(
                                string(c, "name", &path)?,
                                string(c, "value", &path)?,
                                ValueType::from_name(string(c, "valueType", &path)?).ok_or_else(
                                    || Error::new(&path, "unknown constant value type"),
                                )?,
                                opt_string(c, "comment", &path)?,
                                name,
                            );
                            if c.contains_key("protoId") {
                                entry.proto_id = Some(int(c, "protoId", &path)?);
                            }
                            entries.push(entry);
                        }
                        Binding::Constants(
                            builder.add_constants(string(op, "category", &path)?, entries)?,
                        )
                    }
                    _ => unreachable!("matched declaration kinds"),
                };
                builder.bind(symbol, binding)?;
            }
            "protoId" => {
                let id = int(op, "value", &path)?;
                match reference("target", builder)? {
                    Binding::Property(p) => builder.property_mut(p)?.proto_id = Some(id),
                    Binding::Node(n) => builder.node_mut(n)?.proto_id = Some(id),
                    Binding::Edge(e) => builder.edge_mut(e)?.proto_id = Some(id),
                    _ => return Err(Error::new(&path, "constants use per-entry protoId")),
                }
            }
            "mandatory" => {
                let p = reference("target", builder)?.property()?;
                let default = field(op, "default", &path)?;
                let value = if let Some(name) = default.get("$cpgDefault").and_then(Value::as_str) {
                    builder
                        .defaults
                        .get(name)
                        .cloned()
                        .map(Ok)
                        .unwrap_or_else(|| property_default(name))?
                } else {
                    crate::codec::decode_scalar(default, &path)?
                };
                if value.value_type() != builder.schema().property(p)?.value_type {
                    return Err(Error::new(
                        &path,
                        "default does not match declared property type",
                    ));
                }
                builder.property_mut(p)?.cardinality = Cardinality::One(Some(value));
            }
            "asList" => {
                let p = reference("target", builder)?.property()?;
                builder.property_mut(p)?.cardinality = Cardinality::List;
            }
            "addProperty" | "addProperties" => {
                let n = reference("target", builder)?.node()?;
                let ps = refs(op, builder, &path)?
                    .into_iter()
                    .map(Binding::property)
                    .collect::<Result<Vec<_>>>()?;
                builder.add_properties(n, &ps)?;
            }
            "extendz" => {
                let n = reference("target", builder)?.node()?;
                let ns = refs(op, builder, &path)?
                    .into_iter()
                    .map(Binding::node)
                    .collect::<Result<Vec<_>>>()?;
                builder.extend(n, &ns)?;
            }
            "primaryKey" => {
                let n = reference("target", builder)?.node()?;
                let p = reference("reference", builder)?.property()?;
                builder.node_mut(n)?.primary_key = Some(p);
            }
            "withProperty" => {
                let e = reference("target", builder)?.edge()?;
                let p = reference("reference", builder)?.property()?;
                builder.edge_mut(e)?.property = Some(p);
            }
            "starterName" => {
                let n = reference("target", builder)?.node()?;
                builder.node_mut(n)?.starter_name = Some(string(op, "value", &path)?.into());
            }
            "withDefaultAccessorName" => {
                let e = reference("target", builder)?.edge()?;
                builder.edge_mut(e)?.default_accessor_name =
                    Some(string(op, "value", &path)?.into());
            }
            "relation" => {
                let source = reference("source", builder)?.node()?;
                let edge = reference("edge", builder)?.edge()?;
                let target = reference("destination", builder)?.node()?;
                builder.add_relation(Relation {
                    module: name.into(),
                    cardinality_out: edge_card(op, "cardinalityOut", &path)?,
                    cardinality_in: edge_card(op, "cardinalityIn", &path)?,
                    step_name_out: nonempty(opt_string(op, "stepNameOut", &path)?),
                    step_name_in: nonempty(opt_string(op, "stepNameIn", &path)?),
                    step_doc_out: nonempty(opt_string(op, "stepNameOutDoc", &path)?),
                    step_doc_in: nonempty(opt_string(op, "stepNameInDoc", &path)?),
                    ..Relation::new(source, edge, target)
                })?;
            }
            "contained" => {
                let owner = reference("target", builder)?.node()?;
                let target = reference("node", builder)?.node()?;
                let cardinality = match string(op, "cardinality", &path)? {
                    "List" => Cardinality::List,
                    "ZeroOrOne" => Cardinality::ZeroOrOne,
                    "One" => {
                        if !field(op, "default", &path)?.is_null() {
                            return Err(Error::new(&path, "contained-node default must be null"));
                        }
                        Cardinality::One(None)
                    }
                    _ => return Err(Error::new(&path, "unknown contained cardinality")),
                };
                builder.add_contained(
                    owner,
                    ContainedNode {
                        node_type: target,
                        local_name: string(op, "localName", &path)?.into(),
                        cardinality,
                        comment: nonempty(opt_string(op, "comment", &path)?),
                    },
                )?;
            }
            _ => return Err(Error::new(path, format!("unknown schema operation {kind}"))),
        }
    }
    Ok(())
}
pub fn configured_builder(bundle: &Value) -> Result<SchemaBuilder> {
    let root = object(bundle, "bundle")?;
    let o = object(field(root, "cpg:builder", "bundle")?, "cpg:builder")?;
    let traversals = array(o, "additionalTraversalsPackages", "cpg:builder")?
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| Error::new("cpg:builder", "traversal package must be string"))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut builder = SchemaBuilder::new(
        string(o, "domainShortName", "cpg:builder")?,
        string(o, "basePackage", "cpg:builder")?,
        &traversals,
    );
    if let Some(defaults) = root.get("cpg:propertyDefaults") {
        for (name, value) in object(defaults, "cpg:propertyDefaults")? {
            builder.defaults.insert(
                name.clone(),
                crate::codec::decode_scalar(value, &format!("cpg:propertyDefaults/{name}"))?,
            );
        }
    }
    Ok(builder)
}
pub fn builder_from_bundle(bundle: &Value) -> Result<SchemaBuilder> {
    let root = object(bundle, "bundle")?;
    let mut builder = configured_builder(bundle)?;
    if let Some(modules) = root.get("cpg:modules") {
        for module in modules
            .as_array()
            .ok_or_else(|| Error::new("cpg:modules", "expected array"))?
        {
            apply_module(&mut builder, module)?;
        }
    } else {
        for module in array(root, "cpg:composition", "bundle")? {
            let name = module
                .as_str()
                .ok_or_else(|| Error::new("cpg:composition", "expected module name"))?;
            apply_module(&mut builder, &canonical_module(name)?)?;
        }
    }
    Ok(builder)
}
pub fn interpret_bundle(bundle: &Value) -> Result<Schema> {
    crate::dialect::validate_bundle_syntax(bundle)?;
    let schema = builder_from_bundle(bundle)?.build()?;
    schema.validate()?;
    Ok(schema)
}
pub fn canonical_bundle() -> Result<Value> {
    let mut bundle: Value = serde_json::from_str(crate::CpgSchema::DOCUMENT)
        .map_err(|e| Error::new("CpgSchema", e.to_string()))?;
    let names = bundle["cpg:composition"]
        .as_array()
        .ok_or_else(|| Error::new("CpgSchema", "missing composition"))?
        .clone();
    let modules = names
        .iter()
        .map(|v| {
            canonical_module(
                v.as_str()
                    .ok_or_else(|| Error::new("CpgSchema", "module name must be string"))?,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    bundle
        .as_object_mut()
        .ok_or_else(|| Error::new("CpgSchema", "expected object"))?
        .remove("cpg:composition");
    bundle["cpg:modules"] = Value::Array(modules);
    Ok(bundle)
}
pub fn canonical_module(name: &str) -> Result<Value> {
    match name {
        "Base" => crate::Base::document(),
        "FileSystem" => crate::FileSystem::document(),
        "Namespace" => crate::Namespace::document(),
        "Operators" => crate::Operators::document(),
        "Configuration" => crate::Configuration::document(),
        "MetaData" => crate::MetaData::document(),
        "Type" => crate::Type::document(),
        "Method" => crate::Method::document(),
        "Ast" => crate::Ast::document(),
        "CallGraph" => crate::CallGraph::document(),
        "Cfg" => crate::Cfg::document(),
        "Dominators" => crate::Dominators::document(),
        "Pdg" => crate::Pdg::document(),
        "Shortcuts" => crate::Shortcuts::document(),
        "Comment" => crate::Comment::document(),
        "Tags" => crate::Tags::document(),
        "Binding" => crate::Binding::document(),
        "Annotation" => crate::Annotation::document(),
        "Finding" => crate::Finding::document(),
        "Hidden" => crate::Hidden::document(),
        "ProtoSerialize" => crate::ProtoSerialize::document(),
        _ => Err(Error::new(name, "unknown bundled module")),
    }
}

pub fn property_default(name: &str) -> Result<crate::value::Scalar> {
    let doc: Value = serde_json::from_str(crate::CpgSchema::DOCUMENT)
        .map_err(|e| Error::new("PropertyDefaults", e.to_string()))?;
    let value = doc
        .get("cpg:propertyDefaults")
        .and_then(|v| v.get(name))
        .ok_or_else(|| Error::new(name, "unknown property default"))?;
    crate::codec::decode_scalar(value, name)
}
