//! Native port of Type.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Relation, Result, SchemaBuilder,
    nonempty,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Type";
pub const DOC_INDEX: i32 = 6;
pub const DESCRIPTION: &str = r#"
The Type Layer contains information about type declarations, relations between
types, and type instantiation and usage. In its current form, it allows modelling of
parametrized types, type hierarchies and aliases.
"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub typeFullName: PropertyId,
    pub aliasTypeFullName: PropertyId,
    pub inheritsFromTypeFullName: PropertyId,
    pub typeDeclFullName: PropertyId,
    pub typeDecl: NodeTypeId,
    pub typeParameter: NodeTypeId,
    /// No local CODE property: preserves the executable Scala definition despite its prose.
    pub typeArgument: NodeTypeId,
    pub member: NodeTypeId,
    pub tpe: NodeTypeId,
    pub bindsTo: EdgeTypeId,
    pub aliasOf: EdgeTypeId,
    pub inheritsFrom: EdgeTypeId,
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
        let astParentFullName = base.astParentFullName;
        let astParentType = base.astParentType;
        let declaration = base.declaration;
        let file = fs.file;
        let filename = fs.filename;
        let fullName = base.fullName;
        let isExternal = base.isExternal;
        let name = base.name;
        let r#ref = base.r#ref;
        let sourceFile = fs.sourceFile;
        let typeFullName = builder.add_property(
            MODULE_NAME,
            "TYPE_FULL_NAME",
            ValueType::String,
            r#"This field contains the fully-qualified static type name of the program
construct represented by a node. It is the name of an instantiated type, e.g.,
`java.util.List<Integer>`, rather than `java.util.List[T]`. If the type
cannot be determined, this field should be set to the empty string.
"#,
        )?;
        builder.property_mut(typeFullName)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(typeFullName)?.proto_id = Some(crate::ProtoIds::TypeFullName);
        let aliasTypeFullName = builder.add_property(
            MODULE_NAME,
            "ALIAS_TYPE_FULL_NAME",
            ValueType::String,
            r#"This property holds the fully qualified name of the type that the node is
a type alias of.
"#,
        )?;
        builder.property_mut(aliasTypeFullName)?.proto_id =
            Some(crate::ProtoIds::AliasTypeFullName);
        let inheritsFromTypeFullName = builder.add_property(
            MODULE_NAME,
            "INHERITS_FROM_TYPE_FULL_NAME",
            ValueType::String,
            r#"The static types a TYPE_DECL inherits from. This property is matched against the
FULL_NAME of TYPE nodes and thus it is required to have at least one TYPE node
for each TYPE_FULL_NAME"#,
        )?;
        builder.property_mut(inheritsFromTypeFullName)?.cardinality = Cardinality::List;
        builder.property_mut(inheritsFromTypeFullName)?.proto_id =
            Some(crate::ProtoIds::InheritsFromTypeFullName);
        let typeDeclFullName = builder.add_property(
            MODULE_NAME,
            "TYPE_DECL_FULL_NAME",
            ValueType::String,
            r#"The static type decl of a TYPE. This property is matched against the FULL_NAME
of TYPE_DECL nodes. It is required to have exactly one TYPE_DECL for each
different TYPE_DECL_FULL_NAME"#,
        )?;
        builder.property_mut(typeDeclFullName)?.cardinality = Cardinality::One(Some(
            Scalar::string(crate::CpgSchema::PropertyDefaults::String),
        ));
        builder.property_mut(typeDeclFullName)?.proto_id = Some(crate::ProtoIds::TypeDeclFullName);
        let typeDecl = builder.add_node(
            MODULE_NAME,
            "TYPE_DECL",
            NodeKind::Concrete,
            r#"This node represents a type declaration as for example given by a class-, struct-,
or union declaration. In contrast to a `TYPE` node, this node does not represent a
concrete instantiation of a type, e.g., for the parametrized type `List[T]`, it represents
`List[T]`, but not `List[Integer]` where `Integer` is a concrete type.

The language frontend MUST create type declarations for all types declared in the
source program and MAY provide type declarations for types that are not declared
but referenced by the source program. If a declaration is present in the source
program, the field `IS_EXTERNAL` is set to `false`. Otherwise, it is set to `true`.

The `FULL_NAME` field specifies the type's fully-qualified name, including
information about the namespace it is contained in if applicable, the name field
is the type's short name. Line and column number information is specified in the
optional fields `LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and
`COLUMN_NUMBER_END` and the name of the source file is specified in `FILENAME`.

Base types can be specified via the `INHERITS_FROM_TYPE_FULL_NAME` list, where
each entry contains the fully-qualified name of a base type. If the type is
known to be an alias of another type (as for example introduced via the C
`typedef` statement), the name of the alias is stored in `ALIAS_TYPE_FULL_NAME`.

Finally, the fully qualified name of the program constructs that the type declaration
is immediately contained in is stored in the `AST_PARENT_FULL_NAME` field
and its type is indicated in the `AST_PARENT_TYPE` field to be one of
`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.

"#,
        )?;
        builder.node_mut(typeDecl)?.proto_id = Some(crate::ProtoIds::TypeDecl);
        builder.add_properties(
            typeDecl,
            &[
                name,
                fullName,
                isExternal,
                inheritsFromTypeFullName,
                aliasTypeFullName,
                filename,
            ],
        )?;
        builder.add_properties(typeDecl, &[astParentType, astParentFullName])?;
        builder.node_mut(typeDecl)?.primary_key = Some(name);
        let typeParameter = builder.add_node(
            MODULE_NAME,
            "TYPE_PARAMETER",
            NodeKind::Concrete,
            r#"This node represents a formal type parameter, that is, the type parameter
as given in a type-parametrized method or type declaration. Examples for
languages that support type parameters are Java (via Generics) and C++
(via templates). Apart from the standard fields of AST nodes, the type
parameter carries only a `NAME` field that holds the parameters name.
"#,
        )?;
        builder.node_mut(typeParameter)?.proto_id = Some(crate::ProtoIds::TypeParameter);
        builder.add_properties(typeParameter, &[name])?;
        let typeArgument = builder.add_node(
            MODULE_NAME,
            "TYPE_ARGUMENT",
            NodeKind::Concrete,
            r#"An (actual) type argument as used to instantiate a parametrized type, in the
same way an (actual) arguments provides concrete values for a parameter
at method call sites. As it true for arguments, the method is not expected
to  interpret the type argument. It MUST however store its code in the
`CODE` field.
"#,
        )?;
        builder.node_mut(typeArgument)?.proto_id = Some(crate::ProtoIds::TypeArgument);
        let member = builder.add_node(
            MODULE_NAME,
            "MEMBER",
            NodeKind::Concrete,
            r#"This node represents a type member of a class, struct or union, e.g., for the
 type declaration `class Foo{ int i ; }`, it represents the declaration of the
 variable `i`.
"#,
        )?;
        builder.node_mut(member)?.proto_id = Some(crate::ProtoIds::Member);
        builder.add_properties(member, &[typeFullName])?;
        builder.add_properties(member, &[astParentType, astParentFullName])?;
        builder.extend(member, &[declaration])?;
        builder.node_mut(member)?.primary_key = Some(name);
        let tpe = builder.add_node(
            MODULE_NAME,
            "TYPE",
            NodeKind::Concrete,
            r#"This node represents a type instance, that is, a concrete instantiation
of a type declaration."#,
        )?;
        builder.node_mut(tpe)?.proto_id = Some(crate::ProtoIds::Type);
        builder.add_properties(tpe, &[name, fullName, typeDeclFullName])?;
        builder.node_mut(tpe)?.starter_name = Some("typ".into());
        builder.node_mut(tpe)?.primary_key = Some(name);
        let bindsTo = builder.add_edge(
            MODULE_NAME,
            "BINDS_TO",
            r#"This edge connects type arguments to type parameters to indicate
that the type argument is used to instantiate the type parameter.
"#,
        )?;
        builder.edge_mut(bindsTo)?.proto_id = Some(crate::ProtoIds::BindsTo);
        let aliasOf = builder.add_edge(
            MODULE_NAME,
            "ALIAS_OF",
            r#"This edge represents an alias relation between a type declaration and a type.
The language frontend MUST NOT create `ALIAS_OF` edges as they are created
automatically based on `ALIAS_TYPE_FULL_NAME` fields when the CPG is first loaded.
"#,
        )?;
        builder.edge_mut(aliasOf)?.proto_id = Some(crate::ProtoIds::AliasOf);
        let inheritsFrom = builder.add_edge(
            MODULE_NAME,
            "INHERITS_FROM",
            r#"Inheritance relation between a type declaration and a type. This edge MUST NOT
 be created by the language frontend as it is automatically created from
 `INHERITS_FROM_TYPE_FULL_NAME` fields then the CPG is first loaded.
"#,
        )?;
        builder.edge_mut(inheritsFrom)?.proto_id = Some(crate::ProtoIds::InheritsFrom);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, inheritsFrom, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_out: nonempty("aliasedType"),
            step_name_in: nonempty("aliasTypeDecl"),
            step_doc_in: nonempty("Direct alias type declarations."),
            ..Relation::new(typeDecl, aliasOf, tpe)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_in: nonempty("typeDecl"),
            ..Relation::new(typeDecl, sourceFile, file)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeArgument, bindsTo, typeParameter)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeArgument, r#ref, tpe)
        })?;
        Ok(Self {
            typeFullName,
            aliasTypeFullName,
            inheritsFromTypeFullName,
            typeDeclFullName,
            typeDecl,
            typeParameter,
            typeArgument,
            member,
            tpe,
            bindsTo,
            aliasOf,
            inheritsFrom,
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
