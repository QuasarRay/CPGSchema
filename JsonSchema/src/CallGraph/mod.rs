//! Independently maintained CallGraph JSON schema module.
use crate::model::{ConstantCategoryId, EdgeTypeId, ModuleInfo, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("CallGraph", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub argumentIndex: PropertyId,
    pub argumentName: PropertyId,
    pub argumentLabel: PropertyId,
    pub methodFullName: PropertyId,
    pub evaluationStrategy: PropertyId,
    pub evaluationStrategies: ConstantCategoryId,
    pub dispatchType: PropertyId,
    pub call: EdgeTypeId,
    pub argument: EdgeTypeId,
    pub receiver: EdgeTypeId,
    pub dispatchTypes: ConstantCategoryId,
    pub staticReceiver: PropertyId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _methodSchema: &crate::Method::Schema,
        _astSchema: &crate::Ast::Schema,
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
                .find(|m| m.name == "CallGraph")
                .cloned()
                .ok_or_else(|| crate::Error::new("CallGraph", "module not constructed"))?,
            argumentIndex: builder.binding("CallGraph::argumentIndex")?.property()?,
            argumentName: builder.binding("CallGraph::argumentName")?.property()?,
            argumentLabel: builder.binding("CallGraph::argumentLabel")?.property()?,
            methodFullName: builder.binding("CallGraph::methodFullName")?.property()?,
            evaluationStrategy: builder
                .binding("CallGraph::evaluationStrategy")?
                .property()?,
            evaluationStrategies: builder
                .binding("CallGraph::evaluationStrategies")?
                .constants()?,
            dispatchType: builder.binding("CallGraph::dispatchType")?.property()?,
            call: builder.binding("CallGraph::call")?.edge()?,
            argument: builder.binding("CallGraph::argument")?.edge()?,
            receiver: builder.binding("CallGraph::receiver")?.edge()?,
            dispatchTypes: builder.binding("CallGraph::dispatchTypes")?.constants()?,
            staticReceiver: builder.binding("CallGraph::staticReceiver")?.property()?,
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
