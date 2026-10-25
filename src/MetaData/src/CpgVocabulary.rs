use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const CPG_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10";
pub const CPG_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/core/2026-10";
pub const BASE_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/modules/base/2026-10";
pub const METADATA_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/modules/metadata/2026-10";

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
