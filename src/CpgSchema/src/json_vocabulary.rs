//! `jsonschema` implementation hooks for the CPG schema vocabulary.
//!
//! These keywords describe the schema composition and are annotations with
//! respect to arbitrary data-instance validation. Their shapes are constrained
//! by the vocabulary/meta-schema and the project-wide semantic contract.

use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const CPG_DIALECT_URI: &str = "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10";
pub const CPG_VOCABULARY_URI: &str = "https://shiftleft.io/codepropertygraph/schema/vocab/cpg-schema/2026-10";
pub const CPG_VOCABULARY_META_SCHEMA_URI: &str = "https://shiftleft.io/codepropertygraph/schema/meta/cpg-schema-vocabulary/2026-10";
pub const CPG_SCHEMA_META_SCHEMA_URI: &str = "https://shiftleft.io/codepropertygraph/schema/meta/cpg-schema/2026-10";
pub const CPG_SCHEMA_URI: &str = "https://shiftleft.io/codepropertygraph/schema/cpg/2026-10";

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

pub fn annotation_keyword<'a>(
    _parent: &'a Map<String, Value>,
    _value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    Ok(Box::new(AnnotationKeyword))
}
