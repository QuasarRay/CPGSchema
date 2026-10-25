use crate::SchemaBase::SchemaBase;

/// Rust marker corresponding to the Scala singleton object `Base`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Base;

impl SchemaBase for Base {
    #[inline]
    fn doc_index(&self) -> i32 {
        i32::MAX
    }

    #[inline]
    fn description(&self) -> &str {
        "\n"
    }

    #[inline]
    fn provided_by_frontend(&self) -> bool {
        true
    }
}

/// The original Base schema document, embedded in the binary.
///
/// Unlike the previous implementation, this is not parsed/compiled at runtime.
/// It is exposed only for tooling, diagnostics, code generation, or export.
pub const SCHEMA_JSON: &str = include_str!("../schemas/base.schema.json");

/// Compile-time validator for the Base CPG-dialect schema.
///
/// During macro expansion `jsonschema`:
/// - reads `schemas/base.schema.json`;
/// - resolves the custom `$schema` URI from `resources`;
/// - declares support for the required CPG vocabulary;
/// - registers every CPG keyword factory;
/// - validates/compiles the schema; and
/// - emits specialized Rust validation code.
///
/// Invalid schema syntax or unresolved required schema resources therefore
/// become Rust compilation errors instead of runtime initialization failures.
#[jsonschema::validator(
    path = "schemas/base.schema.json",
    resources = {
        "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10" => {
            path = "schemas/cpg-dialect.schema.json"
        }
    },
    vocabularies = [
        "https://shiftleft.io/codepropertygraph/schema/vocab/core/2026-10"
    ],
    keywords = {
        "cpg:moduleName" => crate::CpgVocabulary::annotation_keyword,
        "cpg:docIndex" => crate::CpgVocabulary::annotation_keyword,
        "cpg:providedByFrontend" => crate::CpgVocabulary::annotation_keyword,
        "cpg:properties" => crate::CpgVocabulary::annotation_keyword,
        "cpg:mandatory" => crate::CpgVocabulary::annotation_keyword,
        "cpg:protoId" => crate::CpgVocabulary::annotation_keyword,
        "cpg:nodeBaseTypes" => crate::CpgVocabulary::annotation_keyword,
        "cpg:edgeTypes" => crate::CpgVocabulary::annotation_keyword
    },
    methods = {
        is_valid = true,
        validate = true,
        iter_errors = true
    }
)]
pub struct BaseValidator;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scala_object_metadata_is_preserved() {
        let base = Base;

        assert_eq!(base.doc_index(), i32::MAX);
        assert_eq!(base.description(), "\n");
        assert!(base.provided_by_frontend());
    }

    #[test]
    fn schema_is_embedded_without_runtime_file_io() {
        assert!(SCHEMA_JSON.contains("\"cpg:moduleName\": \"Base\""));
        assert!(SCHEMA_JSON.contains("\"VERSION\""));
        assert!(SCHEMA_JSON.contains("\"DECLARATION\""));
        assert!(SCHEMA_JSON.contains("\"REF\""));
    }

    #[test]
    fn generated_validator_is_available_without_runtime_compilation() {
        // The CPG vocabulary currently carries schema-definition annotations,
        // therefore Base itself does not assert a JSON instance shape.
        assert!(BaseValidator::is_valid(&json!(null)));
        assert!(BaseValidator::is_valid(&json!({"anything": "is allowed"})));
        BaseValidator::validate(&json!(42)).unwrap();
    }
}
