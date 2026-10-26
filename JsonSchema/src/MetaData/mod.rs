//! Independently maintained MetaData JSON schema module.
use crate::model::{ConstantCategoryId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("MetaData", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub overlays: PropertyId,
    pub language: PropertyId,
    pub root: PropertyId,
    pub metaData: NodeTypeId,
    pub languages: ConstantCategoryId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder, _base: &crate::Base::Schema) -> Result<Self> {
        crate::interpreter::apply_module(builder, &document()?)?;
        Self::from_builder(builder)
    }
    pub fn from_builder(builder: &SchemaBuilder) -> Result<Self> {
        Ok(Self {
            module_info: builder
                .schema()
                .modules
                .iter()
                .find(|m| m.name == "MetaData")
                .cloned()
                .ok_or_else(|| crate::Error::new("MetaData", "module not constructed"))?,
            overlays: builder.binding("MetaData::overlays")?.property()?,
            language: builder.binding("MetaData::language")?.property()?,
            root: builder.binding("MetaData::root")?.property()?,
            metaData: builder.binding("MetaData::metaData")?.node()?,
            languages: builder.binding("MetaData::languages")?.constants()?,
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
