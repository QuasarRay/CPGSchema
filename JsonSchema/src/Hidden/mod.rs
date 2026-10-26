//! Independently maintained Hidden JSON schema module.
use crate::model::{EdgeTypeId, ModuleInfo, NodeTypeId, PropertyId, Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn document() -> Result<serde_json::Value> {
    serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("Hidden", e.to_string()))
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub module_info: ModuleInfo,
    pub closureBindingId: PropertyId,
    pub capture: EdgeTypeId,
    pub closureBinding: NodeTypeId,
    pub capturedBy: EdgeTypeId,
    pub templateDOM: NodeTypeId,
    pub dependencyGroupId: PropertyId,
    pub dependency: NodeTypeId,
    pub dynamicTypeHintFullName: PropertyId,
    pub possibleTypes: PropertyId,
    pub importNode: NodeTypeId,
    pub importedEntity: PropertyId,
    pub importedAs: PropertyId,
    pub explicitAs: PropertyId,
    pub isWildcard: PropertyId,
    pub isExplicit: PropertyId,
    pub isModuleImport: PropertyId,
    pub imports: EdgeTypeId,
    pub isCallForImport: EdgeTypeId,
    pub secondaryId: PropertyId,
    pub genericSignature: PropertyId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        _base: &crate::Base::Schema,
        _methodSchema: &crate::Method::Schema,
        _typeDeclSchema: &crate::Type::Schema,
        _astSchema: &crate::Ast::Schema,
        _cfgSchema: &crate::Cfg::Schema,
        _fsSchema: &crate::FileSystem::Schema,
        _callGraph: &crate::CallGraph::Schema,
        _pdg: &crate::Pdg::Schema,
        _tags: &crate::Tags::Schema,
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
                .find(|m| m.name == "Hidden")
                .cloned()
                .ok_or_else(|| crate::Error::new("Hidden", "module not constructed"))?,
            closureBindingId: builder.binding("Hidden::closureBindingId")?.property()?,
            capture: builder.binding("Hidden::capture")?.edge()?,
            closureBinding: builder.binding("Hidden::closureBinding")?.node()?,
            capturedBy: builder.binding("Hidden::capturedBy")?.edge()?,
            templateDOM: builder.binding("Hidden::templateDOM")?.node()?,
            dependencyGroupId: builder.binding("Hidden::dependencyGroupId")?.property()?,
            dependency: builder.binding("Hidden::dependency")?.node()?,
            dynamicTypeHintFullName: builder
                .binding("Hidden::dynamicTypeHintFullName")?
                .property()?,
            possibleTypes: builder.binding("Hidden::possibleTypes")?.property()?,
            importNode: builder.binding("Hidden::importNode")?.node()?,
            importedEntity: builder.binding("Hidden::importedEntity")?.property()?,
            importedAs: builder.binding("Hidden::importedAs")?.property()?,
            explicitAs: builder.binding("Hidden::explicitAs")?.property()?,
            isWildcard: builder.binding("Hidden::isWildcard")?.property()?,
            isExplicit: builder.binding("Hidden::isExplicit")?.property()?,
            isModuleImport: builder.binding("Hidden::isModuleImport")?.property()?,
            imports: builder.binding("Hidden::imports")?.edge()?,
            isCallForImport: builder.binding("Hidden::isCallForImport")?.edge()?,
            secondaryId: builder.binding("Hidden::secondaryId")?.property()?,
            genericSignature: builder.binding("Hidden::genericSignature")?.property()?,
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
