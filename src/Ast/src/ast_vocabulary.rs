//! Compile-time `jsonschema` hooks for the CPG graph-schema vocabulary.
//!
//! The CPG keywords describe schema-builder operations. They are annotations
//! with respect to arbitrary JSON data instances; schema-document shape is
//! checked separately by the vocabulary meta-schema during `cargo build`.

use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const AST_DIALECT_URI: &str = "https://shiftleft.io/codepropertygraph/schema/dialect/ast/2026-10";
pub const GRAPH_SCHEMA_VOCABULARY_URI: &str = "https://shiftleft.io/codepropertygraph/schema/vocab/graph-schema/2026-10";
pub const GRAPH_SCHEMA_VOCABULARY_META_SCHEMA_URI: &str = "https://shiftleft.io/codepropertygraph/schema/meta/graph-schema-vocabulary/2026-10";
pub const AST_SCHEMA_URI: &str = "https://shiftleft.io/codepropertygraph/schema/modules/ast/2026-10";

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
