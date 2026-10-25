//! JSON Schema dialect/vocabulary implementation of the CPG FileSystem layer.
//!
//! The JSON document is the schema representation. Rust implements only the
//! vocabulary semantics needed for cross-reference, derivation, and
//! builder-contract checks; it does not introduce a parallel schema AST.

mod semantic;
pub mod vocabulary;

use jsonschema::{Draft, Registry};
use serde_json::Value;
use vocabulary::*;

pub const FILESYSTEM_VOCABULARY_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-filesystem.vocabulary.meta.schema.json");
pub const FILESYSTEM_DIALECT_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-filesystem-dialect.meta.schema.json");
pub const FILESYSTEM_LAYER_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-filesystem-layer.meta.schema.json");
pub const FILESYSTEM_SCHEMA_JSON: &str = include_str!("../schemas/filesystem.schema.json");

fn parse(text: &str, name: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|error| format!("invalid JSON in {name}: {error}"))
}

/// Validate a FileSystem-layer schema document against its declared custom
/// dialect, vocabulary syntax, and Rust implementation of static semantics.
pub fn validate_file_system_schema_document(value: &Value) -> Result<(), String> {
    let vocabulary_meta = parse(
        FILESYSTEM_VOCABULARY_META_SCHEMA_JSON,
        "cpg-filesystem.vocabulary.meta.schema.json",
    )?;
    let dialect_meta = parse(
        FILESYSTEM_DIALECT_META_SCHEMA_JSON,
        "cpg-filesystem-dialect.meta.schema.json",
    )?;
    let layer_meta = parse(
        FILESYSTEM_LAYER_META_SCHEMA_JSON,
        "cpg-filesystem-layer.meta.schema.json",
    )?;

    let registry = Registry::new()
        .add(FILESYSTEM_VOCABULARY_META_SCHEMA_URI, vocabulary_meta)
        .map_err(|error| {
            format!("failed to register FileSystem vocabulary meta-schema: {error}")
        })?
        .add(FILESYSTEM_DIALECT_URI, dialect_meta)
        .map_err(|error| format!("failed to register FileSystem dialect meta-schema: {error}"))?
        .prepare()
        .map_err(|error| format!("failed to prepare FileSystem schema registry: {error}"))?;

    // Resolve the document's custom `$schema` and validate it as JSON Schema
    // using the registered FileSystem dialect.
    jsonschema::meta::options()
        .with_registry(&registry)
        .validate(value)
        .map_err(|error| {
            format!("FileSystem document violates its declared dialect: {error}")
        })?;

    // Compile the document under the required vocabulary. The modeling
    // keywords are annotations when the schema is used as a data validator;
    // their cross-reference semantics are checked by the specialized keyword
    // in the FileSystem layer meta-schema below.
    jsonschema::options()
        .with_registry(&registry)
        .with_vocabulary(FILESYSTEM_VOCABULARY_URI)
        .with_keyword("cpg:layerName", vocabulary::annotation_keyword)
        .with_keyword("cpg:docIndex", vocabulary::annotation_keyword)
        .with_keyword("cpg:providedByFrontend", vocabulary::annotation_keyword)
        .with_keyword("cpg:imports", vocabulary::annotation_keyword)
        .with_keyword("cpg:properties", vocabulary::annotation_keyword)
        .with_keyword("cpg:nodeTypes", vocabulary::annotation_keyword)
        .with_keyword("cpg:edgeTypes", vocabulary::annotation_keyword)
        .with_keyword("cpg:outEdges", vocabulary::annotation_keyword)
        .with_keyword("cpg:derivedEdges", vocabulary::annotation_keyword)
        .with_keyword(
            "cpg:autoNodeMaterialization",
            vocabulary::annotation_keyword,
        )
        .with_keyword("cpg:offsetSemantics", vocabulary::annotation_keyword)
        .build(value)
        .map_err(|error| {
            format!("failed to compile FileSystem document under its dialect: {error}")
        })?;

    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(&registry)
        .with_vocabulary(FILESYSTEM_VOCABULARY_URI)
        .with_keyword(
            "cpg:wellFormedFileSystemLayer",
            vocabulary::well_formed_file_system_layer_keyword,
        )
        .build(&layer_meta)
        .map_err(|error| format!("failed to compile FileSystem layer meta-schema: {error}"))?;

    validator
        .validate(value)
        .map_err(|error| format!("FileSystem layer schema is invalid: {error}"))
}

pub fn canonical_file_system_schema() -> Result<Value, String> {
    let value = parse(FILESYSTEM_SCHEMA_JSON, "filesystem.schema.json")?;
    validate_file_system_schema_document(&value)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn canonical() -> Value {
        serde_json::from_str(FILESYSTEM_SCHEMA_JSON).unwrap()
    }

    #[test]
    fn canonical_schema_is_well_formed() {
        validate_file_system_schema_document(&canonical()).unwrap();
    }

    #[test]
    fn inherited_provided_by_frontend_is_false() {
        assert_eq!(canonical()["cpg:providedByFrontend"], Value::Bool(false));
    }

    #[test]
    fn exact_builder_property_surface_is_preserved() {
        let value = canonical();
        let actual: Vec<Value> = value["cpg:properties"]
            .as_array()
            .unwrap()
            .iter()
            .map(|property| {
                json!({
                    "name": property["name"],
                    "valueType": property["valueType"],
                    "cardinality": property["cardinality"],
                    "default": property.get("default").cloned().unwrap_or(Value::Null),
                    "protoId": property["protoId"]
                })
            })
            .collect();

        assert_eq!(
            actual,
            vec![
                json!({"name":"FILENAME","valueType":"String","cardinality":"one","default":"<empty>","protoId":106}),
                json!({"name":"LINE_NUMBER","valueType":"Int","cardinality":"optional","default":null,"protoId":2}),
                json!({"name":"COLUMN_NUMBER","valueType":"Int","cardinality":"optional","default":null,"protoId":11}),
                json!({"name":"LINE_NUMBER_END","valueType":"Int","cardinality":"optional","default":null,"protoId":12}),
                json!({"name":"COLUMN_NUMBER_END","valueType":"Int","cardinality":"optional","default":null,"protoId":16}),
                json!({"name":"OFFSET","valueType":"Int","cardinality":"optional","default":null,"protoId":3812}),
                json!({"name":"OFFSET_END","valueType":"Int","cardinality":"optional","default":null,"protoId":3813})
            ]
        );
    }

    #[test]
    fn file_node_matches_builder() {
        let value = canonical();
        let nodes = value["cpg:nodeTypes"].as_array().unwrap();
        assert_eq!(nodes.len(), 1);
        let file = &nodes[0];
        assert_eq!(file["name"], "FILE");
        assert_eq!(file["protoId"], 38);
        assert_eq!(file["properties"], json!(["NAME", "HASH", "CONTENT"]));
        assert_eq!(file["primaryKey"], "NAME");
    }

    #[test]
    fn source_file_is_declared_without_inventing_endpoint_triples() {
        let value = canonical();
        assert_eq!(value["cpg:outEdges"], json!([]));
        assert_eq!(value["cpg:edgeTypes"][0]["name"], "SOURCE_FILE");
        assert_eq!(value["cpg:edgeTypes"][0]["protoId"], 157);
    }

    #[test]
    fn source_file_creation_policy_is_preserved() {
        let value = canonical();
        let rule = &value["cpg:derivedEdges"][0];
        assert_eq!(rule["edge"], "SOURCE_FILE");
        assert_eq!(rule["sourceScope"], "anyNodeWithProperty");
        assert_eq!(rule["sourceProperty"], "FILENAME");
        assert_eq!(rule["targetNode"], "FILE");
        assert_eq!(rule["createdAutomatically"], true);
        assert_eq!(rule["frontendCreates"], false);
    }

    #[test]
    fn rejects_unknown_node_property() {
        let mut value = canonical();
        value["cpg:nodeTypes"][0]["properties"]
            .as_array_mut()
            .unwrap()
            .push(Value::String("DOES_NOT_EXIST".to_owned()));
        assert!(validate_file_system_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_wrong_mandatory_default_type() {
        let mut value = canonical();
        value["cpg:properties"][0]["default"] = Value::Bool(false);
        assert!(validate_file_system_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_primary_key_not_on_file() {
        let mut value = canonical();
        value["cpg:nodeTypes"][0]["primaryKey"] = Value::String("NOPE".to_owned());
        assert!(validate_file_system_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_frontend_creation_of_source_file_edges() {
        let mut value = canonical();
        value["cpg:derivedEdges"][0]["frontendCreates"] = Value::Bool(true);
        assert!(validate_file_system_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_unrelated_auto_materialization_rule() {
        let mut value = canonical();
        value["cpg:autoNodeMaterialization"][0]["fromProperty"] =
            Value::String("LINE_NUMBER".to_owned());
        assert!(validate_file_system_schema_document(&value).is_err());
    }

    #[test]
    fn rejects_offset_encoding_drift() {
        let mut value = canonical();
        value["cpg:offsetSemantics"]["encoding"] = Value::String("UTF-8".to_owned());
        assert!(validate_file_system_schema_document(&value).is_err());
    }
}
