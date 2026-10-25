//! Rust semantics for the CPG FileSystem-layer JSON Schema vocabulary.

use crate::semantic::validate_file_system_layer_model;
use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const FILESYSTEM_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/filesystem-layer/2026-10";
pub const FILESYSTEM_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/filesystem-layer/2026-10";
pub const FILESYSTEM_VOCABULARY_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/filesystem-layer-vocabulary/2026-10";
pub const FILESYSTEM_LAYER_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/filesystem-layer/2026-10";
pub const FILESYSTEM_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/layer/filesystem/2026-10";

#[derive(Clone, Copy, Debug, Default)]
pub struct AnnotationKeyword;

impl<'i> Keyword<'i> for AnnotationKeyword {
    fn validate(&self, _instance: &'i Value) -> Result<(), ValidationError<'i>> {
        Ok(())
    }

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

#[derive(Clone, Copy, Debug, Default)]
pub struct WellFormedFileSystemLayerKeyword;

impl<'i> Keyword<'i> for WellFormedFileSystemLayerKeyword {
    fn validate(&self, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        validate_file_system_layer_model(instance).map_err(ValidationError::custom)
    }

    fn is_valid(&self, instance: &'i Value) -> bool {
        validate_file_system_layer_model(instance).is_ok()
    }
}

pub fn well_formed_file_system_layer_keyword<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    if value.as_bool() != Some(true) {
        return Err(ValidationError::schema(
            "cpg:wellFormedFileSystemLayer must be the boolean true",
        ));
    }
    Ok(Box::new(WellFormedFileSystemLayerKeyword))
}
