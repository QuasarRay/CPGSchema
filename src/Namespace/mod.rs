//! Native port of Namespace.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder, nonempty};
pub const MODULE_NAME: &str = "Namespace";
pub const DOC_INDEX: i32 = 4;
pub const DESCRIPTION: &str = r#"
Many programming languages allow code to be structured into namespaces. The Namespace
Layer makes these namespaces explicit and associates program constructs with the
namespaces they are defined in.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub namespaceBlock: NodeTypeId,
    pub namespace: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Base".into(), "FileSystem".into()],
        })?;
        let file = fs.file;
        let filename = fs.filename;
        let fullName = base.fullName;
        let name = base.name;
        let sourceFile = fs.sourceFile;
        let namespaceBlock = builder.add_node(
            MODULE_NAME,
            "NAMESPACE_BLOCK",
            NodeKind::Concrete,
            r#"A reference to a namespace.
We borrow the concept of a "namespace block" from C++, that is, a namespace block
is a block of code that has been placed in the same namespace by a programmer.
This block may be introduced via a `package` statement in Java or
a `namespace{ }` statement in C++.

The `FULL_NAME` field contains a unique identifier to represent the namespace block
itself not just the namespace it references. So in addition to the namespace name
it can be useful to use the containing file name to derive a unique identifier.

The `NAME` field contains the namespace name in a human-readable format.
The name should be given in dot-separated form where a dot indicates
that the right hand side is a sub namespace of the left hand side, e.g.,
`foo.bar` denotes the namespace `bar` contained in the namespace `foo`.

"#,
        )?;
        builder.node_mut(namespaceBlock)?.proto_id = Some(crate::ProtoIds::NamespaceBlock);
        builder.add_properties(namespaceBlock, &[name, fullName, filename])?;
        builder.node_mut(namespaceBlock)?.primary_key = Some(name);
        let namespace = builder.add_node(
            MODULE_NAME,
            "NAMESPACE",
            NodeKind::Concrete,
            r#"This node represents a namespace. Similar to FILE nodes, NAMESPACE nodes
serve as indices that allow all definitions inside a namespace to be
obtained by following outgoing edges from a NAMESPACE node.

NAMESPACE nodes MUST NOT be created by language frontends. Instead,
they are generated from NAMESPACE_BLOCK nodes automatically upon
first loading of the CPG.
"#,
        )?;
        builder.node_mut(namespace)?.proto_id = Some(crate::ProtoIds::Namespace);
        builder.add_properties(namespace, &[name])?;
        builder.node_mut(namespace)?.primary_key = Some(name);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_in: nonempty("namespaceBlock"),
            ..Relation::new(namespaceBlock, sourceFile, file)
        })?;
        Ok(Self {
            namespaceBlock,
            namespace,
        })
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
