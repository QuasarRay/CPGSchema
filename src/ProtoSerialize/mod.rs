//! Native port of ProtoSerialize.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{ModuleInfo, PropertyId, Result, SchemaBuilder};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "ProtoSerialize";
pub const DOC_INDEX: i32 = i32::MAX;
pub const DESCRIPTION: &str = r#"

"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub containedRef: PropertyId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder, ast: &crate::Ast::Schema) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Ast".into()],
        })?;
        let unknown = ast.unknown;
        let containedRef = builder.add_property(MODULE_NAME, "CONTAINED_REF", ValueType::String, "References to other nodes. This is not a real property; it exists here for the sake of proto serialization only. valueType and cardinality are meaningless.")?;
        builder.property_mut(containedRef)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(containedRef)?.proto_id = Some(crate::ProtoIds::ContainedRef);
        builder.add_properties(unknown, &[containedRef])?;
        Ok(Self { containedRef })
    }
}
impl crate::SchemaBase::SchemaBase for Schema {
    fn doc_index(&self) -> i32 {
        DOC_INDEX
    }
    fn description(&self) -> &str {
        DESCRIPTION
    }
    fn provided_by_frontend(&self) -> bool {
        PROVIDED_BY_FRONTEND
    }
}
