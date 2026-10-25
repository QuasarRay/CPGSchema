//! Rust implementation hooks for the SchemaBase JSON Schema vocabulary.
//!
//! The custom CPG keywords are metadata/annotation keywords with respect to
//! instance validation. Their schema-document shape is constrained by the
//! SchemaBase vocabulary meta-schema.

use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const SCHEMA_BASE_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/schema-base/2026-10";
pub const SCHEMA_BASE_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/schema-base/2026-10";
pub const CPG_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10";

#[derive(Clone, Copy, Debug, Default)]
pub struct AnnotationKeyword;

impl<'i> Keyword<'i> for AnnotationKeyword {
    #[inline]
    fn validate(&self, _instance: &'i Value) -> Result<(), ValidationError<'i>> {
        Ok(())
    }

    #[inline]
    fn is_valid(&self, _instance: &'i Value) -> bool {
        true
    }
}

/// Factory used by `jsonschema` for both SchemaBase vocabulary keywords.
pub fn annotation_keyword<'a>(
    _parent: &'a Map<String, Value>,
    _value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    Ok(Box::new(AnnotationKeyword))
}
