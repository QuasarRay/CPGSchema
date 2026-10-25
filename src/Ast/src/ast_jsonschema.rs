//! Compile-time JSON Schema dialect binding for `Ast`.
//!
//! This module does not construct a runtime `jsonschema::Registry`.
//! `AstValidator` is generated during `cargo build`.

pub const AST_SCHEMA_JSON: &str = include_str!("../schemas/ast.schema.json");
pub const AST_DIALECT_META_SCHEMA_JSON: &str =
    include_str!("../schemas/ast-dialect.meta.schema.json");
pub const GRAPH_SCHEMA_VOCABULARY_META_SCHEMA_JSON: &str =
    include_str!("../schemas/cpg-graph.vocabulary.meta.schema.json");

/// Validates graph-schema documents as ordinary JSON values.
#[jsonschema::validator(
    path = "schemas/cpg-graph.vocabulary.meta.schema.json",
    draft = Draft202012
)]
pub struct GraphSchemaMetaSchemaValidator;

/// Compile-time validator generated from the concrete AST schema.
///
/// The schema's custom `$schema` URI is supplied as a compile-time resource,
/// and support for the required CPG graph-schema vocabulary is declared here.
#[jsonschema::validator(
    path = "schemas/ast.schema.json",
    resources = {
        "https://shiftleft.io/codepropertygraph/schema/dialect/ast/2026-10" => {
            path = "schemas/ast-dialect.meta.schema.json"
        },
        "https://shiftleft.io/codepropertygraph/schema/meta/graph-schema-vocabulary/2026-10" => {
            path = "schemas/cpg-graph.vocabulary.meta.schema.json"
        }
    },
    vocabularies = [
        "https://shiftleft.io/codepropertygraph/schema/vocab/graph-schema/2026-10"
    ],
    keywords = {
        "cpg:moduleName" => crate::ast_vocabulary::annotation_keyword,
        "cpg:docIndex" => crate::ast_vocabulary::annotation_keyword,
        "cpg:providedByFrontend" => crate::ast_vocabulary::annotation_keyword,
        "cpg:imports" => crate::ast_vocabulary::annotation_keyword,
        "cpg:externals" => crate::ast_vocabulary::annotation_keyword,
        "cpg:operations" => crate::ast_vocabulary::annotation_keyword,
        "cpg:valueType" => crate::ast_vocabulary::annotation_keyword,
        "cpg:mandatory" => crate::ast_vocabulary::annotation_keyword,
        "cpg:protoId" => crate::ast_vocabulary::annotation_keyword,
        "cpg:propertyRefs" => crate::ast_vocabulary::annotation_keyword,
        "cpg:extends" => crate::ast_vocabulary::annotation_keyword,
        "cpg:primaryKeyRef" => crate::ast_vocabulary::annotation_keyword,
        "cpg:starterName" => crate::ast_vocabulary::annotation_keyword,
        "cpg:constants" => crate::ast_vocabulary::annotation_keyword,
        "cpg:cardinalityOut" => crate::ast_vocabulary::annotation_keyword,
        "cpg:cardinalityIn" => crate::ast_vocabulary::annotation_keyword,
        "cpg:stepNameOut" => crate::ast_vocabulary::annotation_keyword,
        "cpg:stepNameOutDoc" => crate::ast_vocabulary::annotation_keyword,
        "cpg:stepNameIn" => crate::ast_vocabulary::annotation_keyword,
        "cpg:stepNameInDoc" => crate::ast_vocabulary::annotation_keyword
    }
)]
pub struct AstValidator;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn schema() -> Value {
        serde_json::from_str(AST_SCHEMA_JSON).unwrap()
    }

    #[test]
    fn ast_schema_matches_graph_schema_metaschema() {
        GraphSchemaMetaSchemaValidator::validate(&schema()).unwrap();
    }

    #[test]
    fn source_counts_are_preserved() {
        let schema = schema();
        let operations = schema["cpg:operations"].as_array().unwrap();

        let count = |kind: &str| {
            operations
                .iter()
                .filter(|op| op["kind"] == kind)
                .count()
        };

        assert_eq!(count("property"), 4);
        assert_eq!(count("nodeBase"), 3);
        assert_eq!(count("node"), 14);
        assert_eq!(count("constants"), 2);
        assert_eq!(count("edge"), 12);
        assert_eq!(count("extend"), 19);
        assert_eq!(count("outEdge"), 180);
    }

    #[test]
    fn ast_metadata_matches_source() {
        let schema = schema();
        assert_eq!(schema["cpg:moduleName"], "Ast");
        assert_eq!(schema["cpg:docIndex"], 7);
        assert_eq!(schema["cpg:providedByFrontend"], true);
    }
}
