//! Independently maintained Type JSON schema module.
use crate::model::{EdgeTypeId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Type", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub typeFullName: PropertyId,
    pub aliasTypeFullName: PropertyId,
    pub inheritsFromTypeFullName: PropertyId,
    pub typeDeclFullName: PropertyId,
    pub typeDecl: NodeTypeId,
    pub typeParameter: NodeTypeId,
    pub typeArgument: NodeTypeId,
    pub member: NodeTypeId,
    pub tpe: NodeTypeId,
    pub bindsTo: EdgeTypeId,
    pub aliasOf: EdgeTypeId,
    pub inheritsFrom: EdgeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _base: &crate::Base::Schema,
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
                .find(|m| m.name == "Type")
                .cloned()
                .ok_or_else(|| crate::Error::new("Type", "module not constructed"))?,
            typeFullName: builder.binding("Type::typeFullName")?.property()?,
            aliasTypeFullName: builder.binding("Type::aliasTypeFullName")?.property()?,
            inheritsFromTypeFullName: builder
                .binding("Type::inheritsFromTypeFullName")?
                .property()?,
            typeDeclFullName: builder.binding("Type::typeDeclFullName")?.property()?,
            typeDecl: builder.binding("Type::typeDecl")?.node()?,
            typeParameter: builder.binding("Type::typeParameter")?.node()?,
            typeArgument: builder.binding("Type::typeArgument")?.node()?,
            member: builder.binding("Type::member")?.node()?,
            tpe: builder.binding("Type::tpe")?.node()?,
            bindsTo: builder.binding("Type::bindsTo")?.edge()?,
            aliasOf: builder.binding("Type::aliasOf")?.edge()?,
            inheritsFrom: builder.binding("Type::inheritsFrom")?.edge()?,
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
