//! Native port of Cfg.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder, nonempty,
};
use crate::value::EdgeCardinality;
pub const MODULE_NAME: &str = "Cfg";
pub const DOC_INDEX: i32 = 9;
pub const DESCRIPTION: &str = r#"
The Control Flow Graph Layer provides control flow graphs for each method. Control
flow graphs are constructed by marking a sub set of the abstract syntax tree nodes
as control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.
The control flow graph models both the control flow within the calculation of an
expression as well as from expression to expression. The layer can be automatically
generated from the syntax tree layer if only control structure types supported by
this specification are employed.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub cfgNode: NodeTypeId,
    pub cfg: EdgeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        methodSchema: &crate::Method::Schema,
        ast: &crate::Ast::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Method".into(), "Ast".into()],
        })?;
        let astNode = ast.astNode;
        let block = ast.block;
        let callNode = ast.callNode;
        let callRepr = ast.callRepr;
        let controlStructure = ast.controlStructure;
        let expression = ast.expression;
        let fieldIdentifier = ast.fieldIdentifier;
        let identifier = ast.identifier;
        let jumpTarget = ast.jumpTarget;
        let literal = ast.literal;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodParameterOut = methodSchema.methodParameterOut;
        let methodRef = ast.methodRef;
        let methodReturn = methodSchema.methodReturn;
        let ret = ast.ret;
        let typeRef = ast.typeRef;
        let unknown = ast.unknown;
        let cfgNode = builder.add_node(
            MODULE_NAME,
            "CFG_NODE",
            NodeKind::Base,
            r#"This is the base class for all control flow nodes. It is itself
a child class of `AST_NODE`, that is, all control flow graph nodes
are also syntax tree nodes in the CPG specification.
"#,
        )?;
        builder.extend(cfgNode, &[astNode])?;
        builder.extend(method, &[cfgNode])?;
        builder.extend(methodReturn, &[cfgNode])?;
        builder.extend(methodParameterIn, &[cfgNode])?;
        builder.extend(methodParameterOut, &[cfgNode])?;
        builder.extend(expression, &[cfgNode])?;
        builder.extend(callRepr, &[cfgNode])?;
        builder.extend(jumpTarget, &[cfgNode])?;
        let cfg = builder.add_edge(
            MODULE_NAME,
            "CFG",
            "This edge indicates control flow from the source to the destination node.",
        )?;
        builder.edge_mut(cfg)?.proto_id = Some(crate::ProtoIds::Cfg);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(method, cfg, methodReturn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("cfgFirst"),
            step_doc_out: nonempty("First control flow graph node"),
            ..Relation::new(method, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(fieldIdentifier, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(fieldIdentifier, cfg, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(jumpTarget, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(identifier, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(literal, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeRef, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, cfg, cfgNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            cardinality_in: EdgeCardinality::List,
            step_name_in: nonempty("toReturn"),
            ..Relation::new(ret, cfg, methodReturn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, cfg, methodReturn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, cfg, methodReturn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeRef, cfg, methodReturn)
        })?;
        Ok(Self { cfgNode, cfg })
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
