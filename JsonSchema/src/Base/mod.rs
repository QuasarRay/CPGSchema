//! Independently maintained Base JSON schema module.
use crate::model::{EdgeTypeId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Base", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub version: PropertyId,
    pub hash: PropertyId,
    pub code: PropertyId,
    pub isExternal: PropertyId,
    pub index: PropertyId,
    pub name: PropertyId,
    pub fullName: PropertyId,
    pub parserTypeName: PropertyId,
    pub value: PropertyId,
    pub content: PropertyId,
    pub astParentType: PropertyId,
    pub astParentFullName: PropertyId,
    pub declaration: NodeTypeId,
    pub r#ref: EdgeTypeId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder) -> Result<Self> {
        crate::interpreter::apply_module(builder, &document()?)?;
        Self::from_builder(builder)
    }
    pub fn from_builder(builder: &SchemaBuilder) -> Result<Self> {
        Ok(Self {
            module_info: builder
                .schema()
                .modules
                .iter()
                .find(|m| m.name == "Base")
                .cloned()
                .ok_or_else(|| crate::Error::new("Base", "module not constructed"))?,
            version: builder.binding("Base::version")?.property()?,
            hash: builder.binding("Base::hash")?.property()?,
            code: builder.binding("Base::code")?.property()?,
            isExternal: builder.binding("Base::isExternal")?.property()?,
            index: builder.binding("Base::index")?.property()?,
            name: builder.binding("Base::name")?.property()?,
            fullName: builder.binding("Base::fullName")?.property()?,
            parserTypeName: builder.binding("Base::parserTypeName")?.property()?,
            value: builder.binding("Base::value")?.property()?,
            content: builder.binding("Base::content")?.property()?,
            astParentType: builder.binding("Base::astParentType")?.property()?,
            astParentFullName: builder.binding("Base::astParentFullName")?.property()?,
            declaration: builder.binding("Base::declaration")?.node()?,
            r#ref: builder.binding("Base::ref")?.edge()?,
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
