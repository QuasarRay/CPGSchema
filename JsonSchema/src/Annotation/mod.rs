//! Independently maintained Annotation JSON schema module.
use crate::model::{ModuleInfo, NodeTypeId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Annotation", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub annotation: NodeTypeId,
    pub annotationParameterAssign: NodeTypeId,
    pub annotationParameter: NodeTypeId,
    pub annotationLiteral: NodeTypeId,
    pub arrayInitializer: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _base: &crate::Base::Schema,
        _methodSchema: &crate::Method::Schema,
        _typeSchema: &crate::Type::Schema,
        _astSchema: &crate::Ast::Schema,
        _shortcuts: &crate::Shortcuts::Schema,
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
                .find(|m| m.name == "Annotation")
                .cloned()
                .ok_or_else(|| crate::Error::new("Annotation", "module not constructed"))?,
            annotation: builder.binding("Annotation::annotation")?.node()?,
            annotationParameterAssign: builder
                .binding("Annotation::annotationParameterAssign")?
                .node()?,
            annotationParameter: builder.binding("Annotation::annotationParameter")?.node()?,
            annotationLiteral: builder.binding("Annotation::annotationLiteral")?.node()?,
            arrayInitializer: builder.binding("Annotation::arrayInitializer")?.node()?,
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
