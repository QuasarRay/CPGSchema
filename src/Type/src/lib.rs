//! JSON Schema dialect/vocabulary implementation of the CPG Type layer.
//!
//! The JSON document is the schema representation. Rust implements only the
//! vocabulary semantics needed for cross-reference and builder-contract checks;
//! it does not introduce a parallel schema AST.

mod semantic;
pub mod vocabulary;

use jsonschema::{Draft, Registry};
use serde_json::Value;
use vocabulary::*;

pub const TYPE_VOCABULARY_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-type.vocabulary.meta.schema.json");
pub const TYPE_DIALECT_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-type-dialect.meta.schema.json");
pub const TYPE_LAYER_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-type-layer.meta.schema.json");
pub const TYPE_SCHEMA_JSON: &str = include_str!("../schemas/type.schema.json");

fn parse(text: &str, name: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|error| format!("invalid JSON in {name}: {error}"))
}

/// Validate a Type-layer schema document against both its JSON shape and the
/// Rust implementation of the vocabulary's static semantics.
pub fn validate_type_schema_document(value: &Value) -> Result<(), String> {
    let vocabulary_meta = parse(
        TYPE_VOCABULARY_META_SCHEMA_JSON,
        "cpg-type.vocabulary.meta.schema.json",
    )?;
    let dialect_meta = parse(
        TYPE_DIALECT_META_SCHEMA_JSON,
        "cpg-type-dialect.meta.schema.json",
    )?;
    let layer_meta = parse(
        TYPE_LAYER_META_SCHEMA_JSON,
        "cpg-type-layer.meta.schema.json",
    )?;

    let registry = Registry::new()
        .add(TYPE_VOCABULARY_META_SCHEMA_URI, vocabulary_meta)
        .map_err(|error| format!("failed to register Type vocabulary meta-schema: {error}"))?
        .add(TYPE_DIALECT_URI, dialect_meta)
        .map_err(|error| format!("failed to register Type dialect meta-schema: {error}"))?
        .prepare()
        .map_err(|error| format!("failed to prepare Type schema registry: {error}"))?;

    // Resolve the document's custom `$schema` and validate it as a JSON Schema
    // using the registered Type dialect.
    jsonschema::meta::options()
        .with_registry(&registry)
        .validate(value)
        .map_err(|error| format!("Type document violates its declared dialect: {error}"))?;

    // Compile the document as a schema as well. `with_vocabulary` explicitly
    // declares support for the required vocabulary URI; the vocabulary's model
    // keywords are annotations when the schema is used as a data validator.
    jsonschema::options()
        .with_registry(&registry)
        .with_vocabulary(TYPE_VOCABULARY_URI)
        .with_keyword("cpg:layerName", vocabulary::annotation_keyword)
        .with_keyword("cpg:docIndex", vocabulary::annotation_keyword)
        .with_keyword("cpg:providedByFrontend", vocabulary::annotation_keyword)
        .with_keyword("cpg:imports", vocabulary::annotation_keyword)
        .with_keyword("cpg:properties", vocabulary::annotation_keyword)
        .with_keyword("cpg:nodeTypes", vocabulary::annotation_keyword)
        .with_keyword("cpg:edgeTypes", vocabulary::annotation_keyword)
        .with_keyword("cpg:outEdges", vocabulary::annotation_keyword)
        .with_keyword("cpg:derivedEdges", vocabulary::annotation_keyword)
        .with_keyword("cpg:references", vocabulary::annotation_keyword)
        .build(value)
        .map_err(|error| format!("failed to compile Type document under its dialect: {error}"))?;

    // The specialized layer meta-schema invokes the Rust semantic keyword for
    // cross-reference and builder-contract invariants.
    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(&registry)
        .with_vocabulary(TYPE_VOCABULARY_URI)
        .with_keyword(
            "cpg:wellFormedTypeLayer",
            vocabulary::well_formed_type_layer_keyword,
        )
        .build(&layer_meta)
        .map_err(|error| format!("failed to compile Type layer meta-schema: {error}"))?;

    validator
        .validate(value)
        .map_err(|error| format!("Type layer schema is invalid: {error}"))
}

pub fn canonical_type_schema() -> Result<Value, String> {
    let value = parse(TYPE_SCHEMA_JSON, "type.schema.json")?;
    validate_type_schema_document(&value)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical() -> Value {
        serde_json::from_str(TYPE_SCHEMA_JSON).unwrap()
    }

    #[test]
    fn canonical_schema_is_well_formed() {
        validate_type_schema_document(&canonical()).unwrap();
    }

    #[test]
    fn rejects_unknown_node_property() {
        let mut value = canonical();
        value["cpg:nodeTypes"][0]["properties"]
            .as_array_mut()
            .unwrap()
            .push(Value::String("DOES_NOT_EXIST".to_owned()));
        assert!(validate_type_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_primary_key_not_on_node_or_base() {
        let mut value = canonical();
        value["cpg:nodeTypes"][4]["primaryKey"] = Value::String("NOPE".to_owned());
        assert!(validate_type_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_invalid_out_edge_endpoint() {
        let mut value = canonical();
        value["cpg:outEdges"][0]["inNode"] = Value::String("NOPE".to_owned());
        assert!(validate_type_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_derived_edge_cardinality_drift() {
        let mut value = canonical();
        value["cpg:derivedEdges"][1]["sourceCardinality"] = Value::String("one".to_owned());
        assert!(validate_type_schema_document(&value).is_err());
    }

    #[test]
    fn type_argument_preserves_builder_exactness() {
        let value = canonical();
        let type_argument = value["cpg:nodeTypes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| node["name"] == "TYPE_ARGUMENT")
            .unwrap();
        assert_eq!(type_argument["properties"], serde_json::json!([]));
    }
}
