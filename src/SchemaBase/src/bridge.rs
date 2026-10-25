//! Optional bridge from the JSON Schema binding to the Rust-native contract.
//!
//! Neither side depends on this bridge:
//! - `schema_base` is usable without JSON.
//! - JSON schema documents are usable without Rust consumers.
//! This module is only an adapter for applications that use both.

use crate::{
    json_dialect::CpgDialectMetaSchemaValidator,
    schema_base::{SchemaBase, SchemaBaseRef},
};
use serde_json::Value;
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonSchemaBaseError {
    InvalidDialectDocument(String),
    MissingDocIndex,
    DocIndexOutOfRange,
    MissingDescription,
}

impl fmt::Display for JsonSchemaBaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDialectDocument(message) => formatter.write_str(message),
            Self::MissingDocIndex => formatter.write_str("missing cpg:docIndex"),
            Self::DocIndexOutOfRange => formatter.write_str("cpg:docIndex is outside i32"),
            Self::MissingDescription => formatter.write_str("missing description"),
        }
    }
}

impl Error for JsonSchemaBaseError {}

/// Borrowed adapter implementing the Rust-native SchemaBase interface for a
/// validated JSON schema document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JsonSchemaBase<'a> {
    metadata: SchemaBaseRef<'a>,
}

impl<'a> JsonSchemaBase<'a> {
    pub fn parse(document: &'a Value) -> Result<Self, JsonSchemaBaseError> {
        CpgDialectMetaSchemaValidator::validate(document).map_err(|error| {
            JsonSchemaBaseError::InvalidDialectDocument(error.to_string())
        })?;

        let raw_doc_index = document
            .get("cpg:docIndex")
            .and_then(Value::as_i64)
            .ok_or(JsonSchemaBaseError::MissingDocIndex)?;
        let doc_index =
            i32::try_from(raw_doc_index).map_err(|_| JsonSchemaBaseError::DocIndexOutOfRange)?;

        let description = document
            .get("description")
            .and_then(Value::as_str)
            .ok_or(JsonSchemaBaseError::MissingDescription)?;

        let provided_by_frontend = document
            .get("cpg:providedByFrontend")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        Ok(Self {
            metadata: SchemaBaseRef {
                doc_index,
                description,
                provided_by_frontend,
            },
        })
    }
}

impl SchemaBase for JsonSchemaBase<'_> {
    #[inline]
    fn doc_index(&self) -> i32 {
        self.metadata.doc_index
    }

    #[inline]
    fn description(&self) -> &str {
        self.metadata.description
    }

    #[inline]
    fn provided_by_frontend(&self) -> bool {
        self.metadata.provided_by_frontend
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_dialect::EXAMPLE_MODULE_SCHEMA_JSON;

    #[test]
    fn omitted_json_flag_maps_to_semantic_false() {
        let document: Value = serde_json::from_str(EXAMPLE_MODULE_SCHEMA_JSON).unwrap();
        let metadata = JsonSchemaBase::parse(&document).unwrap();

        assert_eq!(metadata.doc_index(), 0);
        assert_eq!(
            metadata.description(),
            "Example module implementing the SchemaBase contract."
        );
        assert!(!metadata.provided_by_frontend());
    }
}
