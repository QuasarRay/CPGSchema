//! Native port of Annotation.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{ModuleInfo, NodeKind, NodeTypeId, Relation, Result, SchemaBuilder};
pub const MODULE_NAME: &str = "Annotation";
pub const DOC_INDEX: i32 = 20;
pub const DESCRIPTION: &str = r#"
Java Annotation related CPG definitions.
"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub annotation: NodeTypeId,
    pub annotationParameterAssign: NodeTypeId,
    pub annotationParameter: NodeTypeId,
    pub annotationLiteral: NodeTypeId,
    pub arrayInitializer: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        methodSchema: &crate::Method::Schema,
        typeSchema: &crate::Type::Schema,
        astSchema: &crate::Ast::Schema,
        shortcuts: &crate::Shortcuts::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![
                "Base".into(),
                "Method".into(),
                "Type".into(),
                "Ast".into(),
                "Shortcuts".into(),
            ],
        })?;
        let ast = astSchema.ast;
        let astNode = astSchema.astNode;
        let evalType = shortcuts.evalType;
        let expression = astSchema.expression;
        let fullName = base.fullName;
        let identifier = astSchema.identifier;
        let literal = astSchema.literal;
        let member = typeSchema.member;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodRef = astSchema.methodRef;
        let name = base.name;
        let tpe = typeSchema.tpe;
        let typeDecl = typeSchema.typeDecl;
        let unknown = astSchema.unknown;
        let annotation = builder.add_node(
            MODULE_NAME,
            "ANNOTATION",
            NodeKind::Concrete,
            r#"A method annotation.
The semantics of the FULL_NAME property on this node differ from the usual FULL_NAME
semantics in the sense that FULL_NAME describes the represented annotation class/interface
itself and not the ANNOTATION node.
"#,
        )?;
        builder.node_mut(annotation)?.proto_id = Some(crate::ProtoIds::Annotation);
        builder.add_properties(annotation, &[name, fullName])?;
        builder.extend(annotation, &[expression])?;
        let annotationParameterAssign = builder.add_node(
            MODULE_NAME,
            "ANNOTATION_PARAMETER_ASSIGN",
            NodeKind::Concrete,
            "Assignment of annotation argument to annotation parameter",
        )?;
        builder.node_mut(annotationParameterAssign)?.proto_id =
            Some(crate::ProtoIds::AnnotationParameterAssign);
        builder.extend(annotationParameterAssign, &[astNode])?;
        let annotationParameter = builder.add_node(
            MODULE_NAME,
            "ANNOTATION_PARAMETER",
            NodeKind::Concrete,
            "Formal annotation parameter",
        )?;
        builder.node_mut(annotationParameter)?.proto_id =
            Some(crate::ProtoIds::AnnotationParameter);
        builder.extend(annotationParameter, &[astNode])?;
        let annotationLiteral = builder.add_node(
            MODULE_NAME,
            "ANNOTATION_LITERAL",
            NodeKind::Concrete,
            "A literal value assigned to an ANNOTATION_PARAMETER",
        )?;
        builder.node_mut(annotationLiteral)?.proto_id = Some(crate::ProtoIds::AnnotationLiteral);
        builder.add_properties(annotationLiteral, &[name])?;
        builder.extend(annotationLiteral, &[expression])?;
        let arrayInitializer = builder.add_node(
            MODULE_NAME,
            "ARRAY_INITIALIZER",
            NodeKind::Concrete,
            "Initialization construct for arrays",
        )?;
        builder.node_mut(arrayInitializer)?.proto_id = Some(crate::ProtoIds::ArrayInitializer);
        builder.extend(arrayInitializer, &[astNode])?;
        builder.extend(arrayInitializer, &[expression])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(annotation, ast, annotationParameterAssign)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(annotationParameterAssign, ast, annotationParameter)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(annotationParameterAssign, ast, arrayInitializer)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(annotationParameterAssign, ast, annotationLiteral)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(annotationParameterAssign, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(arrayInitializer, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(arrayInitializer, evalType, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(literal, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(identifier, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodParameterIn, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(member, ast, annotation)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, annotation)
        })?;
        Ok(Self {
            annotation,
            annotationParameterAssign,
            annotationParameter,
            annotationLiteral,
            arrayInitializer,
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
