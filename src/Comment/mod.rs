//! Native port of Comment.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder, nonempty};
pub const MODULE_NAME: &str = "Comment";
pub const DOC_INDEX: i32 = 12;
pub const DESCRIPTION: &str = "";
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub comment: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        baseSchema: &crate::Base::Schema,
        astSchema: &crate::Ast::Schema,
        fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Base".into(), "Ast".into(), "FileSystem".into()],
        })?;
        let ast = astSchema.ast;
        let astNode = astSchema.astNode;
        let code = baseSchema.code;
        let file = fs.file;
        let filename = fs.filename;
        let sourceFile = fs.sourceFile;
        let comment = builder.add_node(
            MODULE_NAME,
            "COMMENT",
            NodeKind::Concrete,
            "A source code comment",
        )?;
        builder.node_mut(comment)?.proto_id = Some(crate::ProtoIds::Comment);
        builder.add_properties(comment, &[filename])?;
        builder.extend(comment, &[astNode])?;
        builder.node_mut(comment)?.primary_key = Some(code);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("file"),
            ..Relation::new(comment, sourceFile, comment)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("comment"),
            ..Relation::new(file, ast, comment)
        })?;
        Ok(Self { comment })
    }
}
impl crate::SchemaBase::SchemaBase for Schema {
    fn doc_index(&self) -> i32 {
        DOC_INDEX
    }
    fn description(&self) -> &str {
        DESCRIPTION
    }
    fn provided_by_frontend(&self) -> bool {
        PROVIDED_BY_FRONTEND
    }
}
