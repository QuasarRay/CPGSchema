//! Independently maintained FileSystem JSON schema module.
use crate::model::{EdgeTypeId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("FileSystem", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub filename: PropertyId,
    pub lineNumber: PropertyId,
    pub columnNumber: PropertyId,
    pub lineNumberEnd: PropertyId,
    pub columnNumberEnd: PropertyId,
    pub sourceFile: EdgeTypeId,
    pub file: NodeTypeId,
    pub offset: PropertyId,
    pub offsetEnd: PropertyId,
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
                .find(|m| m.name == "FileSystem")
                .cloned()
                .ok_or_else(|| crate::Error::new("FileSystem", "module not constructed"))?,
            filename: builder.binding("FileSystem::filename")?.property()?,
            lineNumber: builder.binding("FileSystem::lineNumber")?.property()?,
            columnNumber: builder.binding("FileSystem::columnNumber")?.property()?,
            lineNumberEnd: builder.binding("FileSystem::lineNumberEnd")?.property()?,
            columnNumberEnd: builder.binding("FileSystem::columnNumberEnd")?.property()?,
            sourceFile: builder.binding("FileSystem::sourceFile")?.edge()?,
            file: builder.binding("FileSystem::file")?.node()?,
            offset: builder.binding("FileSystem::offset")?.property()?,
            offsetEnd: builder.binding("FileSystem::offsetEnd")?.property()?,
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
