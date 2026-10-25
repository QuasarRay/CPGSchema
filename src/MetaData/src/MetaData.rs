use crate::SchemaBase::SchemaBase;

/// Rust marker corresponding to the Scala singleton object `MetaData`.
#[derive(Clone, Copy, Debug, Default)]
pub struct MetaData;

impl SchemaBase for MetaData {
    #[inline]
    fn doc_index(&self) -> i32 {
        2
    }

    #[inline]
    fn description(&self) -> &str {
        "\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n"
    }

    #[inline]
    fn provided_by_frontend(&self) -> bool {
        true
    }
}

/// Embedded source schema for tooling/export only.
///
/// Validation code is generated from this file during `cargo build`; it is not
/// parsed or compiled into a validator at application runtime.
pub const SCHEMA_JSON: &str = include_str!("../schemas/metadata.schema.json");

#[jsonschema::validator(
    path = "schemas/metadata.schema.json",
    resources = {
        "https://shiftleft.io/codepropertygraph/schema/dialect/2026-10" => {
            path = "schemas/cpg-dialect.schema.json"
        },
        "https://shiftleft.io/codepropertygraph/schema/modules/base/2026-10" => {
            path = "schemas/base.schema.json"
        }
    },
    vocabularies = [
        "https://shiftleft.io/codepropertygraph/schema/vocab/core/2026-10"
    ],
    keywords = {
        "cpg:moduleName" => crate::CpgVocabulary::annotation_keyword,
        "cpg:docIndex" => crate::CpgVocabulary::annotation_keyword,
        "cpg:providedByFrontend" => crate::CpgVocabulary::annotation_keyword,
        "cpg:imports" => crate::CpgVocabulary::annotation_keyword,
        "cpg:properties" => crate::CpgVocabulary::annotation_keyword,
        "cpg:mandatory" => crate::CpgVocabulary::annotation_keyword,
        "cpg:valueType" => crate::CpgVocabulary::annotation_keyword,
        "cpg:cardinality" => crate::CpgVocabulary::annotation_keyword,
        "cpg:protoId" => crate::CpgVocabulary::annotation_keyword,
        "cpg:nodeBaseTypes" => crate::CpgVocabulary::annotation_keyword,
        "cpg:nodeTypes" => crate::CpgVocabulary::annotation_keyword,
        "cpg:propertyRefs" => crate::CpgVocabulary::annotation_keyword,
        "cpg:edgeTypes" => crate::CpgVocabulary::annotation_keyword,
        "cpg:constantCategories" => crate::CpgVocabulary::annotation_keyword,
        "cpg:name" => crate::CpgVocabulary::annotation_keyword,
        "cpg:constants" => crate::CpgVocabulary::annotation_keyword
    },
    methods = {
        is_valid = true,
        validate = true,
        iter_errors = true
    }
)]
pub struct MetaDataValidator;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scala_metadata_is_preserved() {
        let metadata = MetaData;
        assert_eq!(metadata.doc_index(), 2);
        assert!(metadata.provided_by_frontend());
        assert!(metadata.description().contains("Meta Data Layer"));
    }

    #[test]
    fn source_schema_is_embedded_without_runtime_file_io() {
        assert!(SCHEMA_JSON.contains("\"cpg:moduleName\": \"MetaData\""));
        assert!(SCHEMA_JSON.contains("\"OVERLAYS\""));
        assert!(SCHEMA_JSON.contains("\"META_DATA\""));
        assert!(SCHEMA_JSON.contains("\"Languages\""));
    }

    #[test]
    fn generated_validator_requires_no_runtime_compilation() {
        // CPG keywords define graph-schema metadata, so they are annotations
        // with respect to arbitrary JSON instances.
        assert!(MetaDataValidator::is_valid(&json!(null)));
        assert!(MetaDataValidator::is_valid(&json!({"anything": true})));
        MetaDataValidator::validate(&json!(42)).unwrap();
    }
}
