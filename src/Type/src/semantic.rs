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

fn field<'a>(object: &'a Map<String, Value>, key: &str, context: &str) -> Result<&'a Value, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{context} is missing `{key}`"))
}

fn string_field<'a>(object: &'a Map<String, Value>, key: &str, context: &str) -> Result<&'a str, String> {
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
        "Byte" => value
            .as_i64()
            .is_some_and(|n| i8::try_from(n).is_ok()),
        "Short" => value
            .as_i64()
            .is_some_and(|n| i16::try_from(n).is_ok()),
        "Int" => value
            .as_i64()
            .is_some_and(|n| i32::try_from(n).is_ok()),
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

#[derive(Clone, Debug, Default)]
struct NodeInfo {
    properties: BTreeSet<String>,
    extends: BTreeSet<String>,
}

fn resolve_node_properties(
    name: &str,
    local_nodes: &BTreeMap<String, NodeInfo>,
    imported_bases: &BTreeMap<String, BTreeSet<String>>,
    stack: &mut BTreeSet<String>,
) -> Result<BTreeSet<String>, String> {
    if let Some(properties) = imported_bases.get(name) {
        return Ok(properties.clone());
    }

    let Some(node) = local_nodes.get(name) else {
        return Ok(BTreeSet::new());
    };

    if !stack.insert(name.to_owned()) {
        return Err(format!("node inheritance cycle involving `{name}`"));
    }

    let mut result = node.properties.clone();
    for parent in &node.extends {
        result.extend(resolve_node_properties(
            parent,
            local_nodes,
            imported_bases,
            stack,
        )?);
    }
    stack.remove(name);
    Ok(result)
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

/// Validates the static semantics of a declarative CPG Type-layer schema.
///
/// This deliberately operates on JSON values directly. The JSON Schema document
/// is the representation; no parallel Rust schema IR is constructed.
pub fn validate_type_layer_model(value: &Value) -> Result<(), String> {
    let root = object(value, "Type layer schema")?;

    let imports = object(field(root, "cpg:imports", "Type layer schema")?, "cpg:imports")?;
    let imported_properties = strings(field(imports, "properties", "cpg:imports")?, "cpg:imports.properties")?;
    let imported_node_types = strings(field(imports, "nodeTypes", "cpg:imports")?, "cpg:imports.nodeTypes")?;
    let imported_edge_types = strings(field(imports, "edgeTypes", "cpg:imports")?, "cpg:imports.edgeTypes")?;

    let mut imported_bases = BTreeMap::<String, BTreeSet<String>>::new();
    for (index, value) in array(field(imports, "nodeBaseTypes", "cpg:imports")?, "cpg:imports.nodeBaseTypes")?
        .iter()
        .enumerate()
    {
        let context = format!("cpg:imports.nodeBaseTypes[{index}]");
        let base = object(value, &context)?;
        let name = string_field(base, "name", &context)?;
        let properties = strings(field(base, "properties", &context)?, &format!("{context}.properties"))?;
        if imported_bases.insert(name.to_owned(), properties).is_some() {
            return Err(format!("duplicate imported node base type `{name}`"));
        }
    }

    let mut property_names = BTreeSet::<String>::new();
    let mut property_proto_ids = BTreeMap::<u64, String>::new();
    let mut properties = BTreeMap::<String, PropertyInfo>::new();

    for (index, value) in array(field(root, "cpg:properties", "Type layer schema")?, "cpg:properties")?
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

        if !matches!(cardinality, "optional" | "one" | "list") {
            return Err(format!("property `{name}` has unknown cardinality `{cardinality}`"));
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

    let mut node_names = BTreeSet::<String>::new();
    let mut node_proto_ids = BTreeMap::<u64, String>::new();
    let node_values = array(field(root, "cpg:nodeTypes", "Type layer schema")?, "cpg:nodeTypes")?;

    // First pass: names, proto IDs, direct properties, inheritance.
    let mut nodes = BTreeMap::<String, NodeInfo>::new();
    for (index, value) in node_values.iter().enumerate() {
        let context = format!("cpg:nodeTypes[{index}]");
        let node = object(value, &context)?;
        let name = string_field(node, "name", &context)?;
        let proto_id = u64_field(node, "protoId", &context)?;
        insert_unique_name(&mut node_names, name, "node type")?;
        insert_unique_proto(&mut node_proto_ids, proto_id, name, "node type")?;

        let direct_properties = strings(field(node, "properties", &context)?, &format!("{context}.properties"))?;
        for property in &direct_properties {
            if !properties.contains_key(property) && !imported_properties.contains(property) {
                return Err(format!(
                    "node type `{name}` references unknown property `{property}`"
                ));
            }
        }

        let extends = node
            .get("extends")
            .map(|value| strings(value, &format!("{context}.extends")))
            .transpose()?
            .unwrap_or_default();

        nodes.insert(
            name.to_owned(),
            NodeInfo {
                properties: direct_properties,
                extends,
            },
        );
    }

    // Second pass: inheritance targets and primary keys.
    for (index, value) in node_values.iter().enumerate() {
        let context = format!("cpg:nodeTypes[{index}]");
        let node = object(value, &context)?;
        let name = string_field(node, "name", &context)?;
        let info = nodes.get(name).expect("node inserted in first pass");

        for parent in &info.extends {
            if !nodes.contains_key(parent) && !imported_bases.contains_key(parent) {
                return Err(format!(
                    "node type `{name}` extends unknown node base/type `{parent}`"
                ));
            }
        }

        if let Some(primary_key) = node.get("primaryKey").and_then(Value::as_str) {
            let resolved = resolve_node_properties(
                name,
                &nodes,
                &imported_bases,
                &mut BTreeSet::new(),
            )?;
            if !resolved.contains(primary_key) {
                return Err(format!(
                    "primary key `{primary_key}` of node type `{name}` is not one of its direct or inherited properties"
                ));
            }
        }
    }

    let mut edge_names = BTreeSet::<String>::new();
    let mut edge_proto_ids = BTreeMap::<u64, String>::new();
    for (index, value) in array(field(root, "cpg:edgeTypes", "Type layer schema")?, "cpg:edgeTypes")?
        .iter()
        .enumerate()
    {
        let context = format!("cpg:edgeTypes[{index}]");
        let edge = object(value, &context)?;
        let name = string_field(edge, "name", &context)?;
        let proto_id = u64_field(edge, "protoId", &context)?;
        insert_unique_name(&mut edge_names, name, "edge type")?;
        insert_unique_proto(&mut edge_proto_ids, proto_id, name, "edge type")?;
    }

    let node_exists = |name: &str| node_names.contains(name) || imported_node_types.contains(name);
    let edge_exists = |name: &str| edge_names.contains(name) || imported_edge_types.contains(name);

    let mut out_edges = BTreeSet::<(String, String, String)>::new();
    for (index, value) in array(field(root, "cpg:outEdges", "Type layer schema")?, "cpg:outEdges")?
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

    let node_has_property = |node_name: &str, property: &str| -> Result<bool, String> {
        if let Some(node) = nodes.get(node_name) {
            let props = resolve_node_properties(
                node_name,
                &nodes,
                &imported_bases,
                &mut BTreeSet::new(),
            )?;
            return Ok(props.contains(property));
        }
        // Imported concrete node types are opaque to this layer.
        Ok(imported_node_types.contains(node_name) && imported_properties.contains(property))
    };

    for (index, value) in array(field(root, "cpg:derivedEdges", "Type layer schema")?, "cpg:derivedEdges")?
        .iter()
        .enumerate()
    {
        let context = format!("cpg:derivedEdges[{index}]");
        let derived = object(value, &context)?;
        let source_node = string_field(derived, "sourceNode", &context)?;
        let source_property = string_field(derived, "sourceProperty", &context)?;
        let edge = string_field(derived, "edge", &context)?;
        let target_node = string_field(derived, "targetNode", &context)?;
        let target_property = string_field(derived, "targetProperty", &context)?;
        let source_cardinality = string_field(derived, "sourceCardinality", &context)?;

        if !out_edges.contains(&(
            source_node.to_owned(),
            edge.to_owned(),
            target_node.to_owned(),
        )) {
            return Err(format!(
                "derived edge `{edge}` has no matching out-edge declaration `{source_node}` -> `{target_node}`"
            ));
        }
        if !node_has_property(source_node, source_property)? {
            return Err(format!(
                "derived edge `{edge}` uses unknown source property `{source_node}.{source_property}`"
            ));
        }
        if !node_has_property(target_node, target_property)? {
            return Err(format!(
                "derived edge `{edge}` uses unknown target property `{target_node}.{target_property}`"
            ));
        }
        if let Some(info) = properties.get(source_property) {
            if info.cardinality != source_cardinality {
                return Err(format!(
                    "derived edge `{edge}` declares sourceCardinality `{source_cardinality}`, but `{source_property}` is `{}`",
                    info.cardinality
                ));
            }
        }
        if derived.get("frontendCreates").and_then(Value::as_bool) != Some(false) {
            return Err(format!(
                "derived edge `{edge}` must declare frontendCreates=false"
            ));
        }
        if derived.get("createdBy").and_then(Value::as_str) != Some("loader") {
            return Err(format!("derived edge `{edge}` must be createdBy=loader"));
        }
    }

    for (index, value) in array(field(root, "cpg:references", "Type layer schema")?, "cpg:references")?
        .iter()
        .enumerate()
    {
        let context = format!("cpg:references[{index}]");
        let reference = object(value, &context)?;
        let source_node = string_field(reference, "sourceNode", &context)?;
        let source_property = string_field(reference, "sourceProperty", &context)?;
        let target_node = string_field(reference, "targetNode", &context)?;
        let target_property = string_field(reference, "targetProperty", &context)?;

        if !node_exists(source_node) || !node_exists(target_node) {
            return Err(format!(
                "reference `{source_node}.{source_property}` -> `{target_node}.{target_property}` uses an unknown node type"
            ));
        }
        if !node_has_property(source_node, source_property)? {
            return Err(format!(
                "reference uses unknown source property `{source_node}.{source_property}`"
            ));
        }
        if !node_has_property(target_node, target_property)? {
            return Err(format!(
                "reference uses unknown target property `{target_node}.{target_property}`"
            ));
        }
    }

    // Keep this field read so accidental vocabulary drift is caught by the
    // semantic implementation rather than silently becoming inert metadata.
    for info in properties.values() {
        if info.value_type.is_empty() {
            return Err("property ValueType must not be empty".to_owned());
        }
    }

    Ok(())
}
