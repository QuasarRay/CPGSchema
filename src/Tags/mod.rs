//! Native port of Tags.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    ContainedNode, EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder,
};
use crate::value::Cardinality;
pub const MODULE_NAME: &str = "Tags";
pub const DOC_INDEX: i32 = 17;
pub const DESCRIPTION: &str = r#"
The Code Property Graph specification allows for tags to be attached
to arbitrary nodes. Conceptually, this is similar to the creation of
Finding nodes, however, tags are to be used for intermediate results
rather than end-results that are to be reported to the user.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub taggedBy: EdgeTypeId,
    pub tag: NodeTypeId,
    pub tagNodePair: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        typeSchema: &crate::Type::Schema,
        methodSchema: &crate::Method::Schema,
        ast: &crate::Ast::Schema,
        fs: &crate::FileSystem::Schema,
        _callGraph: &crate::CallGraph::Schema,
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
                "Ast".into(),
                "FileSystem".into(),
                "CallGraph".into(),
            ],
        })?;
        let block = ast.block;
        let callNode = ast.callNode;
        let controlStructure = ast.controlStructure;
        let fieldIdentifier = ast.fieldIdentifier;
        let file = fs.file;
        let identifier = ast.identifier;
        let jumpTarget = ast.jumpTarget;
        let literal = ast.literal;
        let local = ast.local;
        let member = typeSchema.member;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodParameterOut = methodSchema.methodParameterOut;
        let methodRef = ast.methodRef;
        let methodReturn = methodSchema.methodReturn;
        let name = base.name;
        let ret = ast.ret;
        let typeDecl = typeSchema.typeDecl;
        let typeRef = ast.typeRef;
        let unknown = ast.unknown;
        let value = base.value;
        let taggedBy = builder.add_edge(
            MODULE_NAME,
            "TAGGED_BY",
            "Edges from nodes to the tags they are tagged by.",
        )?;
        builder.edge_mut(taggedBy)?.proto_id = Some(crate::ProtoIds::TaggedBy);
        let tag = builder.add_node(
            MODULE_NAME,
            "TAG",
            NodeKind::Concrete,
            "This node represents a tag.",
        )?;
        builder.node_mut(tag)?.proto_id = Some(crate::ProtoIds::Tag);
        builder.add_properties(tag, &[name, value])?;
        builder.node_mut(tag)?.primary_key = Some(name);
        let tagNodePair = builder.add_node(
            MODULE_NAME,
            "TAG_NODE_PAIR",
            NodeKind::Concrete,
            "This node contains an arbitrary node and an associated tag node.",
        )?;
        builder.node_mut(tagNodePair)?.proto_id = Some(crate::ProtoIds::TagNodePair);
        builder.add_properties(tagNodePair, &[])?;
        builder.add_contained(
            tagNodePair,
            ContainedNode {
                node_type: tag,
                local_name: "tag".into(),
                cardinality: Cardinality::One(None),
                comment: None,
            },
        )?;
        builder.add_contained(
            tagNodePair,
            ContainedNode {
                node_type: builder.any_node(),
                local_name: "node".into(),
                cardinality: Cardinality::One(None),
                comment: None,
            },
        )?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodReturn, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(literal, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(local, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(member, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(identifier, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(fieldIdentifier, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodParameterIn, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeRef, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(jumpTarget, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(file, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodParameterOut, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(tag, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, taggedBy, tag)
        })?;
        Ok(Self {
            taggedBy,
            tag,
            tagNodePair,
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
