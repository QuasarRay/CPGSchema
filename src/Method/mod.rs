//! Native port of Method.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    ModuleInfo, NodeKind, NodeTypeId, PropertyId, Relation, Result, SchemaBuilder, nonempty,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Method";
pub const DOC_INDEX: i32 = 5;
pub const DESCRIPTION: &str = r#"
The Method Layer contains declarations of methods, functions, and procedures.
Input parameters and output parameters (including return parameters) are
represented, however, method contents is not present in this layer.
"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub signature: PropertyId,
    pub method: NodeTypeId,
    pub isVariadic: PropertyId,
    pub methodParameterIn: NodeTypeId,
    pub methodParameterOut: NodeTypeId,
    pub methodReturn: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        typeDeclSchema: &crate::Type::Schema,
        fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Base".into(), "Type".into(), "FileSystem".into()],
        })?;
        let astParentFullName = base.astParentFullName;
        let astParentType = base.astParentType;
        let columnNumberEnd = fs.columnNumberEnd;
        let declaration = base.declaration;
        let file = fs.file;
        let filename = fs.filename;
        let fullName = base.fullName;
        let hash = base.hash;
        let index = base.index;
        let isExternal = base.isExternal;
        let lineNumberEnd = fs.lineNumberEnd;
        let name = base.name;
        let sourceFile = fs.sourceFile;
        let typeFullName = typeDeclSchema.typeFullName;
        let signature = builder.add_property(
            MODULE_NAME,
            "SIGNATURE",
            ValueType::String,
            r#"
The method signature encodes the types of parameters in a string.
The string SHOULD be human readable and suitable for differentiating methods
with different parameter types sufficiently to allow for resolving of
function overloading. The present specification does not enforce a strict
format for the signature, that is, it can be chosen by the frontend
implementor to fit the source language.
"#,
        )?;
        builder.property_mut(signature)?.cardinality = Cardinality::One(Some(Scalar::string("")));
        builder.property_mut(signature)?.proto_id = Some(crate::ProtoIds::Signature);
        let method = builder.add_node(
            MODULE_NAME,
            "METHOD",
            NodeKind::Concrete,
            r#"Programming languages offer many closely-related concepts for describing blocks
of code that can be executed with input parameters and return output parameters,
possibly causing side effects. In the CPG specification, we refer to all of these
concepts (procedures, functions, methods, etc.) as methods. A single METHOD node
must exist for each method found in the source program.

The `FULL_NAME` field specifies the method's fully-qualified name, including
information about the namespace it is contained in if applicable, the name field
is the function's short name. The field `IS_EXTERNAL` indicates whether it was
possible to identify a method body for the method. This is true for methods that
are defined in the source program, and false for methods that are dynamically
linked to the program, that is, methods that exist in an external dependency.

Line and column number information is specified in the optional fields
`LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and `COLUMN_NUMBER_END` and
the name of the source file is specified in `FILENAME`. An optional hash value
MAY be calculated over the function contents and included in the `HASH` field.

Finally, the fully qualified name of the program constructs that the method
is immediately contained in is stored in the `AST_PARENT_FULL_NAME` field
and its type is indicated in the `AST_PARENT_TYPE` field to be one of
`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.
"#,
        )?;
        builder.node_mut(method)?.proto_id = Some(crate::ProtoIds::Method);
        builder.add_properties(
            method,
            &[
                fullName,
                isExternal,
                signature,
                lineNumberEnd,
                columnNumberEnd,
                filename,
                hash,
            ],
        )?;
        builder.add_properties(method, &[astParentType, astParentFullName])?;
        builder.extend(method, &[declaration])?;
        builder.node_mut(method)?.primary_key = Some(name);
        let isVariadic = builder.add_property(
            MODULE_NAME,
            "IS_VARIADIC",
            ValueType::Boolean,
            r#"
Specifies whether a parameter is the variadic argument handling parameter of
a variadic method. Only one parameter of a method is allowed to have this
property set to true.
"#,
        )?;
        builder.property_mut(isVariadic)?.cardinality =
            Cardinality::One(Some(Scalar::Boolean(false)));
        builder.property_mut(isVariadic)?.proto_id = Some(crate::ProtoIds::IsVariadic);
        let methodParameterIn = builder.add_node(
            MODULE_NAME,
            "METHOD_PARAMETER_IN",
            NodeKind::Concrete,
            r#"
This node represents a formal input parameter. The field `NAME` contains its
name, while the field `TYPE_FULL_NAME` contains the fully qualified type name.
"#,
        )?;
        builder.node_mut(methodParameterIn)?.proto_id = Some(crate::ProtoIds::MethodParameterIn);
        builder.add_properties(methodParameterIn, &[typeFullName, isVariadic, index])?;
        builder.extend(methodParameterIn, &[declaration])?;
        builder.node_mut(methodParameterIn)?.primary_key = Some(name);
        let methodParameterOut = builder.add_node(
            MODULE_NAME,
            "METHOD_PARAMETER_OUT",
            NodeKind::Concrete,
            r#"This node represents a formal output parameter. Corresponding output parameters
for input parameters MUST NOT be created by the frontend as they are automatically
created upon first loading the CPG.
"#,
        )?;
        builder.node_mut(methodParameterOut)?.proto_id = Some(crate::ProtoIds::MethodParameterOut);
        builder.add_properties(methodParameterOut, &[typeFullName, isVariadic, index])?;
        builder.extend(methodParameterOut, &[declaration])?;
        let methodReturn = builder.add_node(
            MODULE_NAME,
            "METHOD_RETURN",
            NodeKind::Concrete,
            r#"This node represents an (unnamed) formal method return parameter. It carries its
fully qualified type name in `TYPE_FULL_NAME`. The `CODE` field MAY be set freely,
e.g., to the constant `RET`, however, subsequent layer creators MUST NOT depend
on this value.
"#,
        )?;
        builder.node_mut(methodReturn)?.proto_id = Some(crate::ProtoIds::MethodReturn);
        builder.add_properties(methodReturn, &[typeFullName])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_in: nonempty("method"),
            ..Relation::new(method, sourceFile, file)
        })?;
        Ok(Self {
            signature,
            method,
            isVariadic,
            methodParameterIn,
            methodParameterOut,
            methodReturn,
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
