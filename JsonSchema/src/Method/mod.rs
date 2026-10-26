//! Independently maintained Method JSON schema module.
use crate::model::{ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Method", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub signature: PropertyId,
    pub method: NodeTypeId,
    pub isVariadic: PropertyId,
    pub methodParameterIn: NodeTypeId,
    pub methodParameterOut: NodeTypeId,
    pub methodReturn: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _base: &crate::Base::Schema,
        _typeDeclSchema: &crate::Type::Schema,
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
                .find(|m| m.name == "Method")
                .cloned()
                .ok_or_else(|| crate::Error::new("Method", "module not constructed"))?,
            signature: builder.binding("Method::signature")?.property()?,
            method: builder.binding("Method::method")?.node()?,
            isVariadic: builder.binding("Method::isVariadic")?.property()?,
            methodParameterIn: builder.binding("Method::methodParameterIn")?.node()?,
            methodParameterOut: builder.binding("Method::methodParameterOut")?.node()?,
            methodReturn: builder.binding("Method::methodReturn")?.node()?,
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
