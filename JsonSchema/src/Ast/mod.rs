//! Independently maintained Ast JSON schema module.
use crate::model::{
    ConstantCategoryId, EdgeTypeId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder,
};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Ast", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub order: PropertyId,
    pub astNode: NodeTypeId,
    pub block: NodeTypeId,
    pub literal: NodeTypeId,
    pub local: NodeTypeId,
    pub identifier: NodeTypeId,
    pub canonicalName: PropertyId,
    pub fieldIdentifier: NodeTypeId,
    pub modifierType: PropertyId,
    pub modifierTypes: ConstantCategoryId,
    pub modifier: NodeTypeId,
    pub jumpTarget: NodeTypeId,
    pub jumpLabel: NodeTypeId,
    pub methodRef: NodeTypeId,
    pub typeRef: NodeTypeId,
    pub ret: NodeTypeId,
    pub controlStructureType: PropertyId,
    pub controlStructureTypes: ConstantCategoryId,
    pub controlStructure: NodeTypeId,
    pub unknown: NodeTypeId,
    pub ast: EdgeTypeId,
    pub condition: EdgeTypeId,
    pub trueBody: EdgeTypeId,
    pub falseBody: EdgeTypeId,
    pub doBody: EdgeTypeId,
    pub forInit: EdgeTypeId,
    pub forUpdate: EdgeTypeId,
    pub forBody: EdgeTypeId,
    pub tryBody: EdgeTypeId,
    pub catchBody: EdgeTypeId,
    pub finallyBody: EdgeTypeId,
    pub jumpArgument: EdgeTypeId,
    pub callRepr: NodeTypeId,
    pub callNode: NodeTypeId,
    pub expression: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _base: &crate::Base::Schema,
        _namespaces: &crate::Namespace::Schema,
        _methodSchema: &crate::Method::Schema,
        _typeSchema: &crate::Type::Schema,
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
                .find(|m| m.name == "Ast")
                .cloned()
                .ok_or_else(|| crate::Error::new("Ast", "module not constructed"))?,
            order: builder.binding("Ast::order")?.property()?,
            astNode: builder.binding("Ast::astNode")?.node()?,
            block: builder.binding("Ast::block")?.node()?,
            literal: builder.binding("Ast::literal")?.node()?,
            local: builder.binding("Ast::local")?.node()?,
            identifier: builder.binding("Ast::identifier")?.node()?,
            canonicalName: builder.binding("Ast::canonicalName")?.property()?,
            fieldIdentifier: builder.binding("Ast::fieldIdentifier")?.node()?,
            modifierType: builder.binding("Ast::modifierType")?.property()?,
            modifierTypes: builder.binding("Ast::modifierTypes")?.constants()?,
            modifier: builder.binding("Ast::modifier")?.node()?,
            jumpTarget: builder.binding("Ast::jumpTarget")?.node()?,
            jumpLabel: builder.binding("Ast::jumpLabel")?.node()?,
            methodRef: builder.binding("Ast::methodRef")?.node()?,
            typeRef: builder.binding("Ast::typeRef")?.node()?,
            ret: builder.binding("Ast::ret")?.node()?,
            controlStructureType: builder.binding("Ast::controlStructureType")?.property()?,
            controlStructureTypes: builder.binding("Ast::controlStructureTypes")?.constants()?,
            controlStructure: builder.binding("Ast::controlStructure")?.node()?,
            unknown: builder.binding("Ast::unknown")?.node()?,
            ast: builder.binding("Ast::ast")?.edge()?,
            condition: builder.binding("Ast::condition")?.edge()?,
            trueBody: builder.binding("Ast::trueBody")?.edge()?,
            falseBody: builder.binding("Ast::falseBody")?.edge()?,
            doBody: builder.binding("Ast::doBody")?.edge()?,
            forInit: builder.binding("Ast::forInit")?.edge()?,
            forUpdate: builder.binding("Ast::forUpdate")?.edge()?,
            forBody: builder.binding("Ast::forBody")?.edge()?,
            tryBody: builder.binding("Ast::tryBody")?.edge()?,
            catchBody: builder.binding("Ast::catchBody")?.edge()?,
            finallyBody: builder.binding("Ast::finallyBody")?.edge()?,
            jumpArgument: builder.binding("Ast::jumpArgument")?.edge()?,
            callRepr: builder.binding("Ast::callRepr")?.node()?,
            callNode: builder.binding("Ast::callNode")?.node()?,
            expression: builder.binding("Ast::expression")?.node()?,
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
