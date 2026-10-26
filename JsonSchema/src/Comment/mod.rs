//! Independently maintained Comment JSON schema module.
use crate::model::{ModuleInfo, NodeTypeId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Comment", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub comment: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _baseSchema: &crate::Base::Schema,
        _astSchema: &crate::Ast::Schema,
        _fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        crate::interpreter::apply_module(builder, &document()?)?;
        Self::from_builder(builder)
    }
    pub fn from_builder(builder: &SchemaBuilder) -> Result<Self> {
        Ok(Self {
            module_info: builder
                .schema()
                .modules
                .iter()
                .find(|m| m.name == "Comment")
                .cloned()
                .ok_or_else(|| crate::Error::new("Comment", "module not constructed"))?,
            comment: builder.binding("Comment::comment")?.node()?,
        })
    }
}
impl crate::SchemaBase::SchemaBase for Schema {
    fn doc_index(&self) -> i32 {
        self.module_info.doc_index
    }
    fn description(&self) -> &str {
        &self.module_info.description
    }
    fn provided_by_frontend(&self) -> bool {
        self.module_info.provided_by_frontend
    }
}
