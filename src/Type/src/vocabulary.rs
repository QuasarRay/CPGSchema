//! Rust semantics for the CPG Type-layer JSON Schema vocabulary.

use crate::semantic::validate_type_layer_model;
use jsonschema::{paths::Location, Keyword, ValidationError};
use serde_json::{Map, Value};

pub const TYPE_DIALECT_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/dialect/type-layer/2026-10";
pub const TYPE_VOCABULARY_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/vocab/type-layer/2026-10";
pub const TYPE_VOCABULARY_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/type-layer-vocabulary/2026-10";
pub const TYPE_LAYER_META_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/meta/type-layer/2026-10";
pub const TYPE_SCHEMA_URI: &str =
    "https://shiftleft.io/codepropertygraph/schema/layer/type/2026-10";

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
pub struct WellFormedTypeLayerKeyword;

impl<'i> Keyword<'i> for WellFormedTypeLayerKeyword {
    fn validate(&self, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        validate_type_layer_model(instance).map_err(ValidationError::custom)
    }

    fn is_valid(&self, instance: &'i Value) -> bool {
        validate_type_layer_model(instance).is_ok()
    }
}

pub fn well_formed_type_layer_keyword<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    if value.as_bool() != Some(true) {
        return Err(ValidationError::schema(
            "cpg:wellFormedTypeLayer must be the boolean true",
        ));
    }
    Ok(Box::new(WellFormedTypeLayerKeyword))
}
