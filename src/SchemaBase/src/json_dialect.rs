//! JSON Schema binding of the SchemaBase semantic contract.
//!
//! Everything here is JSON-specific. The Rust-native `SchemaBase` trait lives
//! separately in `schema_base.rs` and does not depend on this module.

use super::cpg_vocabulary::{
    CPG_DIALECT_URI, SCHEMA_BASE_META_SCHEMA_URI, SCHEMA_BASE_VOCABULARY_URI,
};

/// Raw meta-schema documents are exposed for tooling/export without runtime
/// filesystem access.
pub const SCHEMA_BASE_VOCABULARY_META_SCHEMA_JSON: &str =
    include_str!("../schemas/schema-base.vocabulary.meta.schema.json");

pub const CPG_DIALECT_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-dialect.meta.schema.json");

pub const EXAMPLE_MODULE_SCHEMA_JSON: &str =
    include_str!("../schemas/examples/schema-base.example.schema.json");

/// Compile-time generated validator for the language-neutral SchemaBase
/// vocabulary represented as a JSON schema document.
///
/// The generated validator validates *schema documents* as JSON values.
#[jsonschema::validator(
    path = "schemas/schema-base.vocabulary.meta.schema.json",
    draft = Draft202012
)]
pub struct SchemaBaseMetaSchemaValidator;

/// Compile-time generated validator for the full CPG JSON dialect.
///
/// The custom SchemaBase vocabulary is declared as required and its keyword
/// implementations are registered at macro expansion time.
#[jsonschema::validator(
    path = "schemas/cpg-dialect.meta.schema.json",
    draft = Draft202012,
    resources = {
        "https://shiftleft.io/codepropertygraph/schema/meta/schema-base/2026-10" => {
            path = "schemas/schema-base.vocabulary.meta.schema.json"
        }
    },
    vocabularies = [
        "https://shiftleft.io/codepropertygraph/schema/vocab/schema-base/2026-10"
    ],
    keywords = {
        "cpg:docIndex" => crate::cpg_vocabulary::annotation_keyword,
        "cpg:providedByFrontend" => crate::cpg_vocabulary::annotation_keyword
    }
)]
pub struct CpgDialectMetaSchemaValidator;

/// Compile a representative module schema with the custom dialect during
/// `cargo build`. This proves that the dialect resource, required vocabulary,
/// and keyword factories can be resolved by the compile-time validator macro.
#[jsonschema::validator(
    path = "schemas/examples/schema-base.example.schema.json",
    resources = {
        "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10" => {
            path = "schemas/cpg-dialect.meta.schema.json"
        },
        "https://shiftleft.io/codepropertygraph/schema/meta/schema-base/2026-10" => {
            path = "schemas/schema-base.vocabulary.meta.schema.json"
        }
    },
    vocabularies = [
        "https://shiftleft.io/codepropertygraph/schema/vocab/schema-base/2026-10"
    ],
    keywords = {
        "cpg:docIndex" => crate::cpg_vocabulary::annotation_keyword,
        "cpg:providedByFrontend" => crate::cpg_vocabulary::annotation_keyword
    }
)]
pub struct ExampleModuleInstanceValidator;

/// Stable identifiers are re-exported from this binding for consumers that
/// need to construct or recognize CPG JSON schema documents.
#[inline]
pub const fn dialect_uri() -> &'static str {
    CPG_DIALECT_URI
}

#[inline]
pub const fn vocabulary_uri() -> &'static str {
    SCHEMA_BASE_VOCABULARY_URI
}

#[inline]
pub const fn meta_schema_uri() -> &'static str {
    SCHEMA_BASE_META_SCHEMA_URI
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn example_conforms_to_schema_base_meta_contract() {
        let value: Value = serde_json::from_str(EXAMPLE_MODULE_SCHEMA_JSON).unwrap();
        SchemaBaseMetaSchemaValidator::validate(&value).unwrap();
    }

    #[test]
    fn example_conforms_to_full_json_dialect() {
        let value: Value = serde_json::from_str(EXAMPLE_MODULE_SCHEMA_JSON).unwrap();
        CpgDialectMetaSchemaValidator::validate(&value).unwrap();
    }

    #[test]
    fn compile_time_generated_instance_validator_exists() {
        let instance = serde_json::json!({"anything": true});
        assert!(ExampleModuleInstanceValidator::is_valid(&instance));
    }
}
