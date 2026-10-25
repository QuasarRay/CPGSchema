use serde_json::Value;
use std::{fs, path::Path};

const CPG_VOCABULARY_URI: &str = "https://shiftleft.io/codepropertygraph/schema/vocab/core/2026-10";

fn read_json(path: &str) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("invalid JSON in {path}: {error}"))
}

fn validate_schema_document(
    dialect_validator: &jsonschema::Validator,
    path: &str,
) {
    let candidate = read_json(path);
    if let Err(error) = dialect_validator.validate(&candidate) {
        panic!("{path} does not conform to the CPG dialect: {error}");
    }
}

fn main() {
    for path in [
        "schemas/cpg-dialect.schema.json",
        "schemas/base.schema.json",
        "schemas/metadata.schema.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let dialect = read_json("schemas/cpg-dialect.schema.json");

    // This runs only during `cargo build`. It guarantees that the static schema
    // documents conform structurally to the CPG meta-schema; there is no
    // corresponding validator construction in the application binary.
    let dialect_validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .with_vocabulary(CPG_VOCABULARY_URI)
        .build(&dialect)
        .expect("CPG dialect meta-schema must compile");

    validate_schema_document(&dialect_validator, "schemas/base.schema.json");
    validate_schema_document(&dialect_validator, "schemas/metadata.schema.json");
}
