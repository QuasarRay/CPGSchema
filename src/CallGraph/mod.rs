//! Native port of CallGraph.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    Constant, ConstantCategoryId, EdgeTypeId, ModuleInfo, PropertyId, Relation, Result,
    SchemaBuilder,
};
use crate::value::{Cardinality, EdgeCardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "CallGraph";
pub const DOC_INDEX: i32 = 8;
pub const DESCRIPTION: &str = r#"
The Call Graph Layer represents call relations between methods.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub argumentIndex: PropertyId,
    pub argumentName: PropertyId,
    pub argumentLabel: PropertyId,
    pub methodFullName: PropertyId,
    pub evaluationStrategy: PropertyId,
    pub evaluationStrategies: ConstantCategoryId,
    pub dispatchType: PropertyId,
    pub call: EdgeTypeId,
    pub argument: EdgeTypeId,
    pub receiver: EdgeTypeId,
    pub dispatchTypes: ConstantCategoryId,
    pub staticReceiver: PropertyId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        methodSchema: &crate::Method::Schema,
        astSchema: &crate::Ast::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Method".into(), "Ast".into()],
        })?;
        let block = astSchema.block;
        let callNode = astSchema.callNode;
        let controlStructure = astSchema.controlStructure;
        let expression = astSchema.expression;
        let fieldIdentifier = astSchema.fieldIdentifier;
        let identifier = astSchema.identifier;
        let jumpTarget = astSchema.jumpTarget;
        let literal = astSchema.literal;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodParameterOut = methodSchema.methodParameterOut;
        let methodRef = astSchema.methodRef;
        let methodReturn = methodSchema.methodReturn;
        let ret = astSchema.ret;
        let typeRef = astSchema.typeRef;
        let unknown = astSchema.unknown;
        let argumentIndex = builder.add_property(
            MODULE_NAME,
            "ARGUMENT_INDEX",
            ValueType::Int,
            r#"AST-children of CALL nodes have an argument index, that is used to match
call-site arguments with callee parameters. Explicit parameters are numbered
from 1 to N, while index 0 is reserved for implicit self / this parameter.
CALLs without implicit parameter therefore have arguments starting with index 1.
AST-children of BLOCK nodes may have an argument index as well; in this case,
the last argument index determines the return expression of a BLOCK expression.
If the `PARAMETER_NAME` field is set, then the `ARGUMENT_INDEX` field is
ignored. It is suggested to set it to -1.
"#,
        )?;
        builder.property_mut(argumentIndex)?.cardinality = Cardinality::One(Some(Scalar::Int(-1)));
        builder.property_mut(argumentIndex)?.proto_id = Some(crate::ProtoIds::ArgumentIndex);
        let argumentName = builder.add_property(
            MODULE_NAME,
            "ARGUMENT_NAME",
            ValueType::String,
            r#"
For calls involving named parameters, the `ARGUMENT_NAME` field holds the
name of the parameter initialized by the expression. For all other calls,
this field is unset.
Note that the `ARGUMENT_NAME` should be an exact match of the NAME of a
METHOD_PARAMETER_{IN,OUT}. It overrides ARGUMENT_INDEX for dataflow purposes.
"#,
        )?;
        builder.property_mut(argumentName)?.proto_id = Some(crate::ProtoIds::ArgumentName);
        let argumentLabel = builder.add_property(MODULE_NAME, "ARGUMENT_LABEL", ValueType::String, r#"
This field is used to keep track of the argument label for languages that support them, such as Swift.
It is used in addition to `ARGUMENT_INDEX` and can be used to reconstruct the original call syntax more faithfully.
For example, in Swift, a method call may look like `foo(arg1: 42, arg2: "hello")` where `arg1` and `arg2`
are argument labels. In this case, the `ARGUMENT_LABEL` field for the first argument would be set to `arg1`
and for the second argument it would be set to `arg2`.
Contrary to the `ARGUMENT_NAME` the label should not be expected to match the name of any parameter,
and is not needed for dataflow purposes at all.
"#)?;
        builder.property_mut(argumentLabel)?.proto_id = Some(crate::ProtoIds::ArgumentLabel);
        let methodFullName = builder.add_property(
            MODULE_NAME,
            "METHOD_FULL_NAME",
            ValueType::String,
            r#"The FULL_NAME of a method. Used to link CALL and METHOD nodes. It is required
to have exactly one METHOD node for each METHOD_FULL_NAME"#,
        )?;
        builder.property_mut(methodFullName)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(methodFullName)?.proto_id = Some(crate::ProtoIds::MethodFullName);
        builder.add_properties(expression, &[argumentIndex, argumentName, argumentLabel])?;
        let evaluationStrategy = builder.add_property(
            MODULE_NAME,
            "EVALUATION_STRATEGY",
            ValueType::String,
            r#"For formal method input parameters, output parameters, and return parameters,
this field holds the evaluation strategy, which is one of the following:
1) `BY_REFERENCE` indicates that the parameter is passed by reference, 2)
`BY_VALUE` indicates that it is passed by value, that is, a copy is made,
3) `BY_SHARING` the parameter is a pointer/reference and it is shared with
the caller/callee. While a copy of the pointer is made, a copy of the object
that it points to is not made.
"#,
        )?;
        builder.property_mut(evaluationStrategy)?.cardinality = Cardinality::One(Some(
            Scalar::string(crate::CpgSchema::PropertyDefaults::String),
        ));
        builder.property_mut(evaluationStrategy)?.proto_id =
            Some(crate::ProtoIds::EvaluationStrategy);
        let evaluationStrategies = builder.add_constants("EvaluationStrategies", vec![
    Constant::new("BY_REFERENCE", "BY_REFERENCE", ValueType::String, "A parameter or return of a function is passed by reference which means an address is used behind the scenes", MODULE_NAME).with_proto_id(crate::ProtoIds::ByReference),
    Constant::new("BY_SHARING", "BY_SHARING", ValueType::String, "Only applicable to object parameter or return values. The pointer to the object is passed by value but the object itself is not copied and changes to it are thus propagated out of the method context", MODULE_NAME).with_proto_id(crate::ProtoIds::BySharing),
    Constant::new("BY_VALUE", "BY_VALUE", ValueType::String, "A parameter or return of a function passed by value which means a flat copy is used", MODULE_NAME).with_proto_id(crate::ProtoIds::ByValue)
])?;
        builder.add_properties(methodParameterIn, &[evaluationStrategy])?;
        builder.add_properties(methodReturn, &[evaluationStrategy])?;
        builder.add_properties(methodParameterOut, &[evaluationStrategy])?;
        let dispatchType = builder.add_property(
            MODULE_NAME,
            "DISPATCH_TYPE",
            ValueType::String,
            r#"This field holds the dispatch type of a call, which is either `STATIC_DISPATCH` or
`DYNAMIC_DISPATCH`. For statically dispatched method calls, the call target is known
at compile time while for dynamically dispatched calls, it can only be determined at
runtime as it may depend on the type of an object (as is the case for virtual method
calls) or calculation of an offset.
"#,
        )?;
        builder.property_mut(dispatchType)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(dispatchType)?.proto_id = Some(crate::ProtoIds::DispatchType);
        let call = builder.add_edge(
            MODULE_NAME,
            "CALL",
            r#"This edge connects call sites, i.e., nodes with the type `CALL`, to the
method node that represent the method they invoke. The frontend MAY create
`CALL` edges but is not required to do so. Instead, of the `METHOD_FULL_NAME`
field of the `CALL` node is set correctly, `CALL` edges are created
automatically as the CPG is first loaded.
"#,
        )?;
        builder.edge_mut(call)?.default_accessor_name = Some("STATIC_CALL".into());
        builder.edge_mut(call)?.proto_id = Some(crate::ProtoIds::CallEdge);
        let argument = builder.add_edge(
            MODULE_NAME,
            "ARGUMENT",
            r#"Argument edges connect call sites (node type `CALL`) to their arguments
(node type `EXPRESSION`) as well as `RETURN` nodes to the expressions
that return.
"#,
        )?;
        builder.edge_mut(argument)?.proto_id = Some(crate::ProtoIds::Argument);
        let receiver = builder.add_edge(
            MODULE_NAME,
            "RECEIVER",
            r#"Similar to `ARGUMENT` edges, `RECEIVER` edges connect call sites
to their receiver arguments. A receiver argument is the object on
which a method operates, that is, it is the expression that is
assigned to the `this` pointer as control is transferred to the method.
"#,
        )?;
        builder.edge_mut(receiver)?.proto_id = Some(crate::ProtoIds::Receiver);
        let dispatchTypes = builder.add_constants("DispatchTypes", vec![
    Constant::new("STATIC_DISPATCH", "STATIC_DISPATCH", ValueType::String, "For statically dispatched calls the call target is known before program execution", MODULE_NAME).with_proto_id(crate::ProtoIds::StaticDispatch),
    Constant::new("DYNAMIC_DISPATCH", "DYNAMIC_DISPATCH", ValueType::String, "For dynamically dispatched calls the target is determined during runtime", MODULE_NAME).with_proto_id(crate::ProtoIds::DynamicDispatch),
    Constant::new("INLINED", "INLINED", ValueType::String, "For macro expansions, code is inlined.", MODULE_NAME).with_proto_id(crate::ProtoIds::Inlined)
])?;
        let staticReceiver = builder.add_property(MODULE_NAME, "STATIC_RECEIVER", ValueType::String, r#"The `STATIC_RECEIVER` field is used to keep track of the type on which a static method
is called for static methods which may be inherited. This information can then be used to find
the true `METHOD_FULL_NAME` of the method being called during call linking. For example, if a
class `Foo` defines a static method `foo` and a class `Bar extends Foo`, then the `STATIC_RECEIVER`
of a`Bar.foo()` call is `Bar` and the `METHOD_FULL_NAME` of the `foo` call is rewritten to
`Foo.foo:<signature>`.
"#)?;
        builder.property_mut(staticReceiver)?.proto_id = Some(crate::ProtoIds::StaticReceiver);
        builder.add_properties(callNode, &[dispatchType, staticReceiver])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, call, method)
        })?;
        builder.add_properties(callNode, &[methodFullName])?;
        builder.extend(callNode, &[expression])?;
        builder.add_properties(jumpTarget, &[argumentIndex])?;
        builder.add_properties(methodRef, &[methodFullName])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, receiver, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, receiver, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, receiver, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, receiver, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, receiver, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, receiver, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, receiver, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, receiver, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, argument, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, argument, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(callNode, argument, fieldIdentifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, argument, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, argument, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, argument, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(callNode, argument, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, argument, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, argument, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, argument, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, argument, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(ret, argument, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, argument, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, argument, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, argument, unknown)
        })?;
        Ok(Self {
            argumentIndex,
            argumentName,
            argumentLabel,
            methodFullName,
            evaluationStrategy,
            evaluationStrategies,
            dispatchType,
            call,
            argument,
            receiver,
            dispatchTypes,
            staticReceiver,
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
