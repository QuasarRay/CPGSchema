#[path = "src/semantic.rs"]
mod semantic;

use jsonschema::{Draft, Registry};
use serde_json::Value;
use std::fs;

const FILESYSTEM_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/filesystem-layer/2026-10";
const FILESYSTEM_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/filesystem-layer/2026-10";
const FILESYSTEM_VOCABULARY_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/filesystem-layer-vocabulary/2026-10";

fn read(path: &str) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("invalid JSON in {path}: {error}"))
}

#[derive(Clone, Copy, Debug, Default)]
struct WellFormedFileSystemLayerKeyword;

impl<'i> jsonschema::Keyword<'i> for WellFormedFileSystemLayerKeyword {
    fn validate(&self, instance: &'i Value) -> Result<(), jsonschema::ValidationError<'i>> {
        semantic::validate_file_system_layer_model(instance)
            .map_err(jsonschema::ValidationError::custom)
    }

    fn is_valid(&self, instance: &'i Value) -> bool {
        semantic::validate_file_system_layer_model(instance).is_ok()
    }
}

fn well_formed_file_system_layer_keyword<'a>(
    _parent: &'a serde_json::Map<String, Value>,
    value: &'a Value,
    _location: jsonschema::paths::Location,
) -> Result<Box<dyn for<'i> jsonschema::Keyword<'i>>, jsonschema::ValidationError<'a>> {
    if value.as_bool() != Some(true) {
        return Err(jsonschema::ValidationError::schema(
            "cpg:wellFormedFileSystemLayer must be the boolean true",
        ));
    }
    Ok(Box::new(WellFormedFileSystemLayerKeyword))
}

fn main() {
    for path in [
        "schemas/cpg-filesystem.vocabulary.meta.schema.json",
        "schemas/cpg-filesystem-dialect.meta.schema.json",
        "schemas/cpg-filesystem-layer.meta.schema.json",
        "schemas/filesystem.schema.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let vocabulary_meta = read("schemas/cpg-filesystem.vocabulary.meta.schema.json");
    let dialect_meta = read("schemas/cpg-filesystem-dialect.meta.schema.json");
    let layer_meta = read("schemas/cpg-filesystem-layer.meta.schema.json");
    let canonical = read("schemas/filesystem.schema.json");

    jsonschema::meta::validate(&vocabulary_meta)
        .expect("FileSystem vocabulary meta-schema must be valid Draft 2020-12 JSON Schema");
    jsonschema::meta::validate(&dialect_meta)
        .expect("FileSystem dialect meta-schema must be valid Draft 2020-12 JSON Schema");
    jsonschema::meta::validate(&layer_meta)
        .expect("FileSystem layer meta-schema must be valid Draft 2020-12 JSON Schema");

    let registry = Registry::new()
        .add(FILESYSTEM_VOCABULARY_META_SCHEMA_URI, vocabulary_meta)
        .expect("FileSystem vocabulary meta-schema must register")
        .add(FILESYSTEM_DIALECT_URI, dialect_meta)
        .expect("FileSystem dialect meta-schema must register")
        .prepare()
        .expect("FileSystem schema registry must prepare");

    jsonschema::meta::options()
        .with_registry(&registry)
        .validate(&canonical)
        .expect("filesystem.schema.json must satisfy its declared custom dialect");

    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(&registry)
        .with_vocabulary(FILESYSTEM_VOCABULARY_URI)
        .with_keyword(
            "cpg:wellFormedFileSystemLayer",
            well_formed_file_system_layer_keyword,
        )
        .build(&layer_meta)
        .expect("FileSystem layer meta-schema must compile");

    validator
        .validate(&canonical)
        .expect("filesystem.schema.json must satisfy FileSystem-layer syntax and semantics");
}
