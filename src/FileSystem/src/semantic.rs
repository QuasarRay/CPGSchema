use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

fn object<'a>(value: &'a Value, context: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{context} must be an object"))
}

fn array<'a>(value: &'a Value, context: &str) -> Result<&'a Vec<Value>, String> {
    value
        .as_array()
        .ok_or_else(|| format!("{context} must be an array"))
}

fn field<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Value, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{context} is missing `{key}`"))
}

fn string_field<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a str, String> {
    field(object, key, context)?
        .as_str()
        .ok_or_else(|| format!("{context}.{key} must be a string"))
}

fn u64_field(object: &Map<String, Value>, key: &str, context: &str) -> Result<u64, String> {
    field(object, key, context)?
        .as_u64()
        .ok_or_else(|| format!("{context}.{key} must be a positive integer"))
}

fn strings(value: &Value, context: &str) -> Result<BTreeSet<String>, String> {
    array(value, context)?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("{context} must contain only strings"))
        })
        .collect()
}

fn value_matches(value_type: &str, value: &Value) -> bool {
    match value_type {
        "String" => value.is_string(),
        "Boolean" => value.is_boolean(),
        "Byte" => value.as_i64().is_some_and(|n| i8::try_from(n).is_ok()),
        "Short" => value.as_i64().is_some_and(|n| i16::try_from(n).is_ok()),
        "Int" => value.as_i64().is_some_and(|n| i32::try_from(n).is_ok()),
        "Long" => value.as_i64().is_some() || value.as_u64().is_some(),
        "Float" | "Double" => value.is_number(),
        "Char" => value
            .as_str()
            .is_some_and(|text| text.chars().count() == 1),
        _ => false,
    }
}

#[derive(Clone, Debug)]
struct PropertyInfo {
    value_type: String,
    cardinality: String,
}

#[derive(Clone, Debug)]
struct NodeInfo {
    properties: BTreeSet<String>,
    primary_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DerivedEdgeInfo {
    edge: String,
    source_property: String,
    target_node: String,
}

fn insert_unique_name(
    set: &mut BTreeSet<String>,
    name: &str,
    kind: &str,
) -> Result<(), String> {
    if set.insert(name.to_owned()) {
        Ok(())
    } else {
        Err(format!("duplicate {kind} name `{name}`"))
    }
}

fn insert_unique_proto(
    ids: &mut BTreeMap<u64, String>,
    id: u64,
    name: &str,
    kind: &str,
) -> Result<(), String> {
    if id == 0 {
        return Err(format!("{kind} `{name}` has invalid protoId 0"));
    }
    if let Some(previous) = ids.insert(id, name.to_owned()) {
        return Err(format!(
            "{kind} protoId {id} is shared by `{previous}` and `{name}`"
        ));
    }
    Ok(())
}

/// Validate the static semantics of a declarative CPG FileSystem-layer schema.
///
/// The JSON document itself is the schema representation. This validator works
/// directly on `serde_json::Value`; it intentionally does not build a second
/// Rust schema AST/IR.
pub fn validate_file_system_layer_model(value: &Value) -> Result<(), String> {
    let root = object(value, "FileSystem layer schema")?;

    let imports = object(
        field(root, "cpg:imports", "FileSystem layer schema")?,
        "cpg:imports",
    )?;
    let imported_properties = strings(
        field(imports, "properties", "cpg:imports")?,
        "cpg:imports.properties",
    )?;
    let imported_node_types = strings(
        field(imports, "nodeTypes", "cpg:imports")?,
        "cpg:imports.nodeTypes",
    )?;
    let imported_edge_types = strings(
        field(imports, "edgeTypes", "cpg:imports")?,
        "cpg:imports.edgeTypes",
    )?;

    let mut property_names = BTreeSet::<String>::new();
    let mut property_proto_ids = BTreeMap::<u64, String>::new();
    let mut properties = BTreeMap::<String, PropertyInfo>::new();

    for (index, value) in array(
        field(root, "cpg:properties", "FileSystem layer schema")?,
        "cpg:properties",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:properties[{index}]");
        let property = object(value, &context)?;
        let name = string_field(property, "name", &context)?;
        let value_type = string_field(property, "valueType", &context)?;
        let cardinality = string_field(property, "cardinality", &context)?;
        let proto_id = u64_field(property, "protoId", &context)?;

        insert_unique_name(&mut property_names, name, "property")?;
        insert_unique_proto(&mut property_proto_ids, proto_id, name, "property")?;

        if imported_properties.contains(name) {
            return Err(format!(
                "local property `{name}` duplicates an imported property name"
            ));
        }

        if !matches!(cardinality, "optional" | "one" | "list") {
            return Err(format!(
                "property `{name}` has unknown cardinality `{cardinality}`"
            ));
        }

        let default = property.get("default");
        match cardinality {
            "one" => {
                let default = default.ok_or_else(|| {
                    format!("mandatory property `{name}` must declare its builder default")
                })?;
                if !value_matches(value_type, default) {
                    return Err(format!(
                        "default for property `{name}` does not match ValueType.{value_type}"
                    ));
                }
            }
            "optional" => {
                if let Some(default) = default {
                    if !value_matches(value_type, default) {
                        return Err(format!(
                            "default for optional property `{name}` does not match ValueType.{value_type}"
                        ));
                    }
                }
            }
            "list" => {
                if let Some(default) = default {
                    let items = default.as_array().ok_or_else(|| {
                        format!("default for list property `{name}` must be an array")
                    })?;
                    if items.iter().any(|item| !value_matches(value_type, item)) {
                        return Err(format!(
                            "default list for property `{name}` contains a value incompatible with ValueType.{value_type}"
                        ));
                    }
                }
            }
            _ => unreachable!(),
        }

        properties.insert(
            name.to_owned(),
            PropertyInfo {
                value_type: value_type.to_owned(),
                cardinality: cardinality.to_owned(),
            },
        );
    }

    let property_exists = |name: &str| {
        properties.contains_key(name) || imported_properties.contains(name)
    };

    let mut node_names = BTreeSet::<String>::new();
    let mut node_proto_ids = BTreeMap::<u64, String>::new();
    let mut nodes = BTreeMap::<String, NodeInfo>::new();

    for (index, value) in array(
        field(root, "cpg:nodeTypes", "FileSystem layer schema")?,
        "cpg:nodeTypes",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:nodeTypes[{index}]");
        let node = object(value, &context)?;
        let name = string_field(node, "name", &context)?;
        let proto_id = u64_field(node, "protoId", &context)?;
        let direct_properties = strings(
            field(node, "properties", &context)?,
            &format!("{context}.properties"),
        )?;

        insert_unique_name(&mut node_names, name, "node type")?;
        insert_unique_proto(&mut node_proto_ids, proto_id, name, "node type")?;

        if imported_node_types.contains(name) {
            return Err(format!(
                "local node type `{name}` duplicates an imported node type name"
            ));
        }

        for property in &direct_properties {
            if !property_exists(property) {
                return Err(format!(
                    "node type `{name}` references unknown property `{property}`"
                ));
            }
        }

        let primary_key = node
            .get("primaryKey")
            .map(|value| {
                value
                    .as_str()
                    .map(ToOwned::to_owned)
                    .ok_or_else(|| format!("{context}.primaryKey must be a string"))
            })
            .transpose()?;

        if let Some(primary_key) = &primary_key {
            if !direct_properties.contains(primary_key) {
                return Err(format!(
                    "primary key `{primary_key}` of node type `{name}` is not one of its properties"
                ));
            }
        }

        nodes.insert(
            name.to_owned(),
            NodeInfo {
                properties: direct_properties,
                primary_key,
            },
        );
    }

    let node_exists = |name: &str| {
        nodes.contains_key(name) || imported_node_types.contains(name)
    };

    let mut edge_names = BTreeSet::<String>::new();
    let mut edge_proto_ids = BTreeMap::<u64, String>::new();

    for (index, value) in array(
        field(root, "cpg:edgeTypes", "FileSystem layer schema")?,
        "cpg:edgeTypes",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:edgeTypes[{index}]");
        let edge = object(value, &context)?;
        let name = string_field(edge, "name", &context)?;
        let proto_id = u64_field(edge, "protoId", &context)?;

        insert_unique_name(&mut edge_names, name, "edge type")?;
        insert_unique_proto(&mut edge_proto_ids, proto_id, name, "edge type")?;

        if imported_edge_types.contains(name) {
            return Err(format!(
                "local edge type `{name}` duplicates an imported edge type name"
            ));
        }
    }

    let edge_exists = |name: &str| {
        edge_names.contains(name) || imported_edge_types.contains(name)
    };

    let mut out_edges = BTreeSet::<(String, String, String)>::new();
    for (index, value) in array(
        field(root, "cpg:outEdges", "FileSystem layer schema")?,
        "cpg:outEdges",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:outEdges[{index}]");
        let relation = object(value, &context)?;
        let out_node = string_field(relation, "outNode", &context)?;
        let edge = string_field(relation, "edge", &context)?;
        let in_node = string_field(relation, "inNode", &context)?;

        if !node_exists(out_node) {
            return Err(format!("out-edge references unknown source node `{out_node}`"));
        }
        if !node_exists(in_node) {
            return Err(format!("out-edge references unknown target node `{in_node}`"));
        }
        if !edge_exists(edge) {
            return Err(format!("out-edge references unknown edge type `{edge}`"));
        }

        if !out_edges.insert((out_node.to_owned(), edge.to_owned(), in_node.to_owned())) {
            return Err(format!(
                "duplicate out-edge declaration `{out_node}` -[{edge}]-> `{in_node}`"
            ));
        }
    }

    let mut derived_edges = BTreeSet::<DerivedEdgeInfo>::new();
    for (index, value) in array(
        field(root, "cpg:derivedEdges", "FileSystem layer schema")?,
        "cpg:derivedEdges",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:derivedEdges[{index}]");
        let derived = object(value, &context)?;
        let edge = string_field(derived, "edge", &context)?;
        let source_scope = string_field(derived, "sourceScope", &context)?;
        let source_property = string_field(derived, "sourceProperty", &context)?;
        let target_node = string_field(derived, "targetNode", &context)?;

        if source_scope != "anyNodeWithProperty" {
            return Err(format!(
                "derived edge `{edge}` has unsupported sourceScope `{source_scope}`"
            ));
        }
        if !edge_exists(edge) {
            return Err(format!("derived edge references unknown edge type `{edge}`"));
        }
        if !property_exists(source_property) {
            return Err(format!(
                "derived edge `{edge}` references unknown source property `{source_property}`"
            ));
        }
        if !node_exists(target_node) {
            return Err(format!(
                "derived edge `{edge}` references unknown target node `{target_node}`"
            ));
        }
        if derived.get("createdAutomatically").and_then(Value::as_bool) != Some(true) {
            return Err(format!(
                "derived edge `{edge}` must declare createdAutomatically=true"
            ));
        }
        if derived.get("frontendCreates").and_then(Value::as_bool) != Some(false) {
            return Err(format!(
                "derived edge `{edge}` must declare frontendCreates=false"
            ));
        }

        let info = DerivedEdgeInfo {
            edge: edge.to_owned(),
            source_property: source_property.to_owned(),
            target_node: target_node.to_owned(),
        };
        if !derived_edges.insert(info) {
            return Err(format!(
                "duplicate derived-edge rule for `{edge}` from `{source_property}` to `{target_node}`"
            ));
        }
    }

    let mut materializations = BTreeSet::<(String, String)>::new();
    for (index, value) in array(
        field(root, "cpg:autoNodeMaterialization", "FileSystem layer schema")?,
        "cpg:autoNodeMaterialization",
    )?
    .iter()
    .enumerate()
    {
        let context = format!("cpg:autoNodeMaterialization[{index}]");
        let rule = object(value, &context)?;
        let node_type = string_field(rule, "nodeType", &context)?;
        let from_property = string_field(rule, "fromProperty", &context)?;

        if !node_exists(node_type) {
            return Err(format!(
                "auto-node materialization references unknown node type `{node_type}`"
            ));
        }
        if !property_exists(from_property) {
            return Err(format!(
                "auto-node materialization references unknown source property `{from_property}`"
            ));
        }
        if rule.get("whenMissing").and_then(Value::as_bool) != Some(true) {
            return Err(format!(
                "auto-node materialization for `{node_type}` must declare whenMissing=true"
            ));
        }

        if !materializations.insert((node_type.to_owned(), from_property.to_owned())) {
            return Err(format!(
                "duplicate auto-node materialization for `{node_type}` from `{from_property}`"
            ));
        }

        if let Some(local_node) = nodes.get(node_type) {
            if local_node.primary_key.is_none() {
                return Err(format!(
                    "auto-materialized local node type `{node_type}` must have a primary key"
                ));
            }
        }

        let has_related_derived_edge = derived_edges.iter().any(|derived| {
            derived.source_property == from_property && derived.target_node == node_type
        });
        if !has_related_derived_edge {
            return Err(format!(
                "auto-node materialization `{node_type}` from `{from_property}` has no corresponding derived-edge rule"
            ));
        }
    }

    let offset = object(
        field(root, "cpg:offsetSemantics", "FileSystem layer schema")?,
        "cpg:offsetSemantics",
    )?;
    let start_property = string_field(offset, "startProperty", "cpg:offsetSemantics")?;
    let end_property = string_field(offset, "endProperty", "cpg:offsetSemantics")?;
    let content_property = string_field(offset, "contentProperty", "cpg:offsetSemantics")?;
    let encoding = string_field(offset, "encoding", "cpg:offsetSemantics")?;

    for property in [start_property, end_property, content_property] {
        if !property_exists(property) {
            return Err(format!(
                "offset semantics references unknown property `{property}`"
            ));
        }
    }

    for property in [start_property, end_property] {
        if let Some(info) = properties.get(property) {
            if info.value_type != "Int" {
                return Err(format!(
                    "offset property `{property}` must have ValueType.Int"
                ));
            }
            if info.cardinality != "optional" {
                return Err(format!(
                    "offset property `{property}` must be optional to preserve the Scala builder"
                ));
            }
        }
    }

    if encoding != "UTF-16" {
        return Err(format!(
            "FileSystem offsets must declare UTF-16 encoding, found `{encoding}`"
        ));
    }
    if offset.get("endExclusive").and_then(Value::as_bool) != Some(true) {
        return Err("FileSystem OFFSET_END must be exclusive".to_owned());
    }

    // Read each node's property set to ensure future semantic changes cannot
    // accidentally leave it inert in this implementation.
    for (name, info) in &nodes {
        if info.properties.is_empty() && info.primary_key.is_some() {
            return Err(format!(
                "node type `{name}` has a primary key but no properties"
            ));
        }
    }

    Ok(())
}
