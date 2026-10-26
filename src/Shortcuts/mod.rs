//! Native port of Shortcuts.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{EdgeTypeId, ModuleInfo, Relation, Result, SchemaBuilder, nonempty};
use crate::value::EdgeCardinality;
pub const MODULE_NAME: &str = "Shortcuts";
pub const DOC_INDEX: i32 = 16;
pub const DESCRIPTION: &str = r#"
The Shortcuts Layer provides shortcut edges calculated to speed up
subsequent queries. Language frontends MUST NOT create shortcut edges.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub evalType: EdgeTypeId,
    pub contains: EdgeTypeId,
    pub parameterLink: EdgeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        methodSchema: &crate::Method::Schema,
        astSchema: &crate::Ast::Schema,
        typeSchema: &crate::Type::Schema,
        fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![
                "Base".into(),
                "Method".into(),
                "Ast".into(),
                "Type".into(),
                "FileSystem".into(),
            ],
        })?;
        let block = astSchema.block;
        let callNode = astSchema.callNode;
        let controlStructure = astSchema.controlStructure;
        let fieldIdentifier = astSchema.fieldIdentifier;
        let file = fs.file;
        let identifier = astSchema.identifier;
        let jumpTarget = astSchema.jumpTarget;
        let literal = astSchema.literal;
        let local = astSchema.local;
        let member = typeSchema.member;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodParameterOut = methodSchema.methodParameterOut;
        let methodRef = astSchema.methodRef;
        let methodReturn = methodSchema.methodReturn;
        let r#ref = base.r#ref;
        let ret = astSchema.ret;
        let tpe = typeSchema.tpe;
        let typeDecl = typeSchema.typeDecl;
        let typeRef = astSchema.typeRef;
        let unknown = astSchema.unknown;
        let evalType = builder.add_edge(
            MODULE_NAME,
            "EVAL_TYPE",
            "This edge connects a node to its evaluation type.",
        )?;
        builder.edge_mut(evalType)?.proto_id = Some(crate::ProtoIds::EvalType);
        let contains = builder.add_edge(
            MODULE_NAME,
            "CONTAINS",
            "This edge connects a node to the method that contains it.",
        )?;
        builder.edge_mut(contains)?.proto_id = Some(crate::ProtoIds::Contains);
        let parameterLink = builder.add_edge(
            MODULE_NAME,
            "PARAMETER_LINK",
            r#"This edge connects a method input parameter to the corresponding
method output parameter.
"#,
        )?;
        builder.edge_mut(parameterLink)?.proto_id = Some(crate::ProtoIds::ParameterLink);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_in: nonempty("asInput"),
            step_name_out: nonempty("asOutput"),
            step_doc_out: nonempty("Traverse to corresponding formal output parameter"),
            ..Relation::new(methodParameterIn, parameterLink, methodParameterOut)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(file, contains, typeDecl)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(file, contains, method)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, fieldIdentifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("literal"),
            step_doc_out: nonempty("Literals used in the method"),
            ..Relation::new(method, contains, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            step_name_out: nonempty("typ"),
            step_doc_out: nonempty("Traverse to parameter type"),
            ..Relation::new(methodParameterIn, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("typ"),
            step_doc_out: nonempty("Traverse to parameter type"),
            ..Relation::new(methodParameterOut, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodReturn, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            step_name_out: nonempty("referencedMethod"),
            step_doc_out: nonempty("Traverse to referenced method."),
            ..Relation::new(methodRef, r#ref, method)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeRef, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("referencedTypeDecl"),
            step_doc_out: nonempty("Type declaration which is referenced by this type."),
            ..Relation::new(tpe, r#ref, typeDecl)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, contains, method)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("typ"),
            step_doc_out: nonempty("Traverse to member type"),
            ..Relation::new(member, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(literal, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("referencedMember"),
            step_doc_out: nonempty("Traverse to referenced members"),
            ..Relation::new(callNode, r#ref, member)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("typ"),
            step_doc_out: nonempty("The type of the local."),
            ..Relation::new(local, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(identifier, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, evalType, tpe)
        })?;
        Ok(Self {
            evalType,
            contains,
            parameterLink,
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
