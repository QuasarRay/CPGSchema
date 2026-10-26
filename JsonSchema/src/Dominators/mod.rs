//! Independently maintained Dominators JSON schema module.
use crate::model::{EdgeTypeId, ModuleInfo, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Dominators", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub dominate: EdgeTypeId,
    pub postDominate: EdgeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _methodSchema: &crate::Method::Schema,
        _ast: &crate::Ast::Schema,
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
                .find(|m| m.name == "Dominators")
                .cloned()
                .ok_or_else(|| crate::Error::new("Dominators", "module not constructed"))?,
            dominate: builder.binding("Dominators::dominate")?.edge()?,
            postDominate: builder.binding("Dominators::postDominate")?.edge()?,
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
