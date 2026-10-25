#[path = "src/semantic.rs"]
mod semantic;

use jsonschema::{Draft, Registry};
use serde_json::Value;
use std::fs;

const TYPE_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/type-layer/2026-10";
const TYPE_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/type-layer/2026-10";
const TYPE_VOCABULARY_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/type-layer-vocabulary/2026-10";

fn read(path: &str) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("invalid JSON in {path}: {error}"))
}

#[derive(Clone, Copy, Debug, Default)]
struct WellFormedTypeLayerKeyword;

impl<'i> jsonschema::Keyword<'i> for WellFormedTypeLayerKeyword {
    fn validate(&self, instance: &'i Value) -> Result<(), jsonschema::ValidationError<'i>> {
        semantic::validate_type_layer_model(instance).map_err(jsonschema::ValidationError::custom)
    }

    fn is_valid(&self, instance: &'i Value) -> bool {
        semantic::validate_type_layer_model(instance).is_ok()
    }
}

fn well_formed_type_layer_keyword<'a>(
    _parent: &'a serde_json::Map<String, Value>,
    value: &'a Value,
    _location: jsonschema::paths::Location,
) -> Result<Box<dyn for<'i> jsonschema::Keyword<'i>>, jsonschema::ValidationError<'a>> {
    if value.as_bool() != Some(true) {
        return Err(jsonschema::ValidationError::schema(
            "cpg:wellFormedTypeLayer must be the boolean true",
        ));
    }
    Ok(Box::new(WellFormedTypeLayerKeyword))
}

fn main() {
    for path in [
        "schemas/cpg-type.vocabulary.meta.schema.json",
        "schemas/cpg-type-dialect.meta.schema.json",
        "schemas/cpg-type-layer.meta.schema.json",
        "schemas/type.schema.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let vocabulary_meta = read("schemas/cpg-type.vocabulary.meta.schema.json");
    let dialect_meta = read("schemas/cpg-type-dialect.meta.schema.json");
    let layer_meta = read("schemas/cpg-type-layer.meta.schema.json");
    let canonical = read("schemas/type.schema.json");

    jsonschema::meta::validate(&vocabulary_meta)
        .expect("Type vocabulary meta-schema must be valid Draft 2020-12 JSON Schema");
    jsonschema::meta::validate(&dialect_meta)
        .expect("Type dialect meta-schema must be valid Draft 2020-12 JSON Schema");
    jsonschema::meta::validate(&layer_meta)
        .expect("Type layer meta-schema must be valid Draft 2020-12 JSON Schema");

    let registry = Registry::new()
        .add(TYPE_VOCABULARY_META_SCHEMA_URI, vocabulary_meta)
        .expect("Type vocabulary meta-schema must register")
        .add(TYPE_DIALECT_URI, dialect_meta)
        .expect("Type dialect meta-schema must register")
        .prepare()
        .expect("Type schema registry must prepare");

    jsonschema::meta::options()
        .with_registry(&registry)
        .validate(&canonical)
        .expect("type.schema.json must satisfy its declared custom dialect");

    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(&registry)
        .with_vocabulary(TYPE_VOCABULARY_URI)
        .with_keyword(
            "cpg:wellFormedTypeLayer",
            well_formed_type_layer_keyword,
        )
        .build(&layer_meta)
        .expect("Type layer meta-schema must compile");

    validator
        .validate(&canonical)
        .expect("type.schema.json must satisfy Type-layer syntax and semantics");
}
