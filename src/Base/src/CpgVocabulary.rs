//! CPG JSON Schema dialect vocabulary support for `jsonschema` 0.60.1.
//!
//! The vocabulary is registered by `#[jsonschema::validator]` at compile time.
//! There is deliberately no runtime `Registry`, `OnceLock`, or validator build.
//!
//! The CPG keywords defined here are annotation/schema-definition keywords.
//! Their *shape* is checked by the CPG dialect meta-schema. They do not impose
//! assertions on arbitrary JSON instances, so their keyword validator is a
//! no-op, analogous to metadata/annotation vocabularies in JSON Schema.

use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const CPG_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10";

pub const CPG_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/core/2026-10";

pub const BASE_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/modules/base/2026-10";

/// Annotation-only implementation for CPG schema-definition keywords.
///
/// Structural validation of keyword values is performed by
/// `schemas/cpg-dialect.schema.json` while the schema itself is compiled.
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

/// Factory signature required by `jsonschema::ValidationOptions::with_keyword`
/// and by the compile-time `#[jsonschema::validator(keywords = {...})]` macro.
pub fn annotation_keyword<'a>(
    _parent: &'a Map<String, Value>,
    _value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    Ok(Box::new(AnnotationKeyword))
}
