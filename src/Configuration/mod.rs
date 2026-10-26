//! Native port of Configuration.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{ModuleInfo, NodeKind, NodeTypeId, Result, SchemaBuilder};
pub const MODULE_NAME: &str = "Configuration";
pub const DOC_INDEX: i32 = 18;
pub const DESCRIPTION: &str = r#"
The code property graph specification currently does not contain
schema elements for the representation of configuration files in
a structured format, however, it does allow configuration files
to be included verbatim in the graph to enable language-/framework-
specific passes to access them. This layer provides the necessary
schema elements for this basic support of configuration files.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub configFile: NodeTypeId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder, base: &crate::Base::Schema) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Base".into()],
        })?;
        let content = base.content;
        let name = base.name;
        let configFile = builder.add_node(
            MODULE_NAME,
            "CONFIG_FILE",
            NodeKind::Concrete,
            r#"This node type represent a configuration file, where `NAME` is the name
of the file and `content` is its content. The exact representation of the
name is left undefined and can be chosen as required by consumers of
the corresponding configuration files.
"#,
        )?;
        builder.node_mut(configFile)?.proto_id = Some(crate::ProtoIds::ConfigFile);
        builder.add_properties(configFile, &[name, content])?;
        Ok(Self { configFile })
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
