use serde_json::Value;

/// Common metadata exposed by CPG schema definitions.
pub trait SchemaBase {
    /// Ordering key used by generated documentation.
    /// Smaller values are shown first.
    fn doc_index(&self) -> i32;

    /// Human-readable description of this schema.
    fn description(&self) -> &str;

    /// Whether this schema is supplied by a frontend.
    ///
    /// Matches the Scala default: `false`.
    fn provided_by_frontend(&self) -> bool {
        false
    }
}

#[jsonschema::validator(
    schema = r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "docIndex": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
            },
            "description": {
                "type": "string"
            },
            "providedByFrontend": {
                "type": "boolean",
                "default": false
            }
        },
        "required": ["docIndex", "description"]
    }"#,
    draft = Draft202012
)]
pub struct SchemaBaseValidator;

#[inline]
pub fn validate_schema_base_json(
    value: &Value,
) -> Result<(), jsonschema::ValidationError<'_>> {
    SchemaBaseValidator::validate(value)
}

#[inline]
pub fn is_valid_schema_base_json(value: &Value) -> bool {
    SchemaBaseValidator::is_valid(value)
}