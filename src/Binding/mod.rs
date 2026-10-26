//! Native port of Binding.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder, nonempty,
};
use crate::value::EdgeCardinality;
pub const MODULE_NAME: &str = "Binding";
pub const DOC_INDEX: i32 = 19;
pub const DESCRIPTION: &str = r#"
We use the concept of "bindings" to support resolving of
(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each
pair that we can resolve, we create a `BINDING` node that is connected the
the type declaration via an incoming `BINDS` edge. The `BINDING` node is
connected to the method it resolves to via an outgoing `REF` edge.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub binding: NodeTypeId,
    pub binds: EdgeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        typeDeclSchema: &crate::Type::Schema,
        methodSchema: &crate::Method::Schema,
        callGraphSchema: &crate::CallGraph::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![
                "Base".into(),
                "Type".into(),
                "Method".into(),
                "CallGraph".into(),
            ],
        })?;
        let method = methodSchema.method;
        let methodFullName = callGraphSchema.methodFullName;
        let name = base.name;
        let r#ref = base.r#ref;
        let signature = methodSchema.signature;
        let typeDecl = typeDeclSchema.typeDecl;
        let binding = builder.add_node(
            MODULE_NAME,
            "BINDING",
            NodeKind::Concrete,
            r#"`BINDING` nodes represent name-signature pairs that can be resolved at a
type declaration (`TYPE_DECL`). They are connected to `TYPE_DECL` nodes via
incoming `BINDS` edges. The bound method is either associated with an outgoing
`REF` edge to a `METHOD` or with the `METHOD_FULL_NAME` property. The `REF` edge
if present has priority.
"#,
        )?;
        builder.node_mut(binding)?.proto_id = Some(crate::ProtoIds::Binding);
        builder.add_properties(binding, &[name, signature, methodFullName])?;
        let binds = builder.add_edge(MODULE_NAME, "BINDS", r#"This edge connects a type declaration (`TYPE_DECL`) with a binding node (`BINDING`) and
indicates that the type declaration has the binding represented by the binding node, in
other words, there is a (name, signature) pair that can be resolved for the type
declaration as stored in the binding node.
"#)?;
        builder.edge_mut(binds)?.proto_id = Some(crate::ProtoIds::Binds);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            step_name_in: nonempty("bindingTypeDecl"),
            ..Relation::new(typeDecl, binds, binding)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            step_name_out: nonempty("boundMethod"),
            ..Relation::new(binding, r#ref, method)
        })?;
        Ok(Self { binding, binds })
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
