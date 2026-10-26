//! Native port of Base.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Result, SchemaBuilder,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Base";
pub const DOC_INDEX: i32 = i32::MAX;
pub const DESCRIPTION: &str = r#"
"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub version: PropertyId,
    pub hash: PropertyId,
    pub code: PropertyId,
    pub isExternal: PropertyId,
    pub index: PropertyId,
    pub name: PropertyId,
    pub fullName: PropertyId,
    pub parserTypeName: PropertyId,
    pub value: PropertyId,
    pub content: PropertyId,
    pub astParentType: PropertyId,
    pub astParentFullName: PropertyId,
    pub declaration: NodeTypeId,
    pub r#ref: EdgeTypeId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![],
        })?;

        let version = builder.add_property(
            MODULE_NAME,
            "VERSION",
            ValueType::String,
            r#"A version, given as a string. Used, for example, in the META_DATA node to
indicate which version of the CPG spec this CPG conforms to
"#,
        )?;
        builder.property_mut(version)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(version)?.proto_id = Some(crate::ProtoIds::Version);
        let hash = builder.add_property(
            MODULE_NAME,
            "HASH",
            ValueType::String,
            r#"This property contains a hash value in the form of a string.
Hashes can be used to summarize data, e.g., to summarize the
contents of source files or sub graphs. Such summaries are useful
to determine whether code has already been analyzed in incremental
analysis pipelines. This property is optional to allow its calculation
to be deferred or skipped if the hash is not needed.
"#,
        )?;
        builder.property_mut(hash)?.proto_id = Some(crate::ProtoIds::Hash);
        let code = builder.add_property(
            MODULE_NAME,
            "CODE",
            ValueType::String,
            "This field holds the code snippet that the node represents.",
        )?;
        builder.property_mut(code)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(code)?.proto_id = Some(crate::ProtoIds::Code);
        let isExternal = builder.add_property(
            MODULE_NAME,
            "IS_EXTERNAL",
            ValueType::Boolean,
            r#"Indicates that the construct (METHOD or TYPE_DECL) is external, that is,
it is referenced but not defined in the code (applies both to insular
parsing and to library functions where we have header files only)
"#,
        )?;
        builder.property_mut(isExternal)?.cardinality =
            Cardinality::One(Some(Scalar::Boolean(false)));
        builder.property_mut(isExternal)?.proto_id = Some(crate::ProtoIds::IsExternal);
        let index = builder.add_property(
            MODULE_NAME,
            "INDEX",
            ValueType::Int,
            r#"
Specifies an index, e.g., for a parameter or argument.
 Explicit parameters are numbered from 1 to N, while index 0 is reserved for implicit
 self / this parameter.
"#,
        )?;
        builder.property_mut(index)?.cardinality = Cardinality::One(Some(Scalar::Int(-2)));
        builder.property_mut(index)?.proto_id = Some(crate::ProtoIds::Index);
        let name = builder.add_property(
            MODULE_NAME,
            "NAME",
            ValueType::String,
            "Name of represented object, e.g., method name (e.g. \"run\")",
        )?;
        builder.property_mut(name)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(name)?.proto_id = Some(crate::ProtoIds::Name);
        let fullName = builder.add_property(
            MODULE_NAME,
            "FULL_NAME",
            ValueType::String,
            r#"This is the fully-qualified name of an entity, e.g., the fully-qualified
name of a method or type. The details of what constitutes a fully-qualified
name are language specific. This field SHOULD be human readable.
"#,
        )?;
        builder.property_mut(fullName)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(fullName)?.proto_id = Some(crate::ProtoIds::FullName);
        let parserTypeName = builder.add_property(
            MODULE_NAME,
            "PARSER_TYPE_NAME",
            ValueType::String,
            "AST node type name emitted by parser.",
        )?;
        builder.property_mut(parserTypeName)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(parserTypeName)?.proto_id = Some(crate::ProtoIds::ParserTypeName);
        let value = builder.add_property(
            MODULE_NAME,
            "VALUE",
            ValueType::String,
            "This property denotes a string value as used in a key-value pair.",
        )?;
        builder.property_mut(value)?.cardinality = Cardinality::One(Some(Scalar::string("")));
        builder.property_mut(value)?.proto_id = Some(crate::ProtoIds::Value);
        let content = builder.add_property(
            MODULE_NAME,
            "CONTENT",
            ValueType::String,
            r#"Certain files, e.g., configuration files, may be included in the CPG as-is.
For such files, the `CONTENT` field contains the files content.
"#,
        )?;
        builder.property_mut(content)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(content)?.proto_id = Some(crate::ProtoIds::Content);
        let astParentType = builder.add_property(
            MODULE_NAME,
            "AST_PARENT_TYPE",
            ValueType::String,
            r#"The type of the AST parent. Since this is only used in some parts of the graph,
the list does not include all possible parents by intention.
Possible parents: METHOD, TYPE_DECL, NAMESPACE_BLOCK.
"#,
        )?;
        builder.property_mut(astParentType)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(astParentType)?.proto_id = Some(crate::ProtoIds::AstParentType);
        let astParentFullName = builder.add_property(
            MODULE_NAME,
            "AST_PARENT_FULL_NAME",
            ValueType::String,
            "This field holds the FULL_NAME of the AST parent of an entity.",
        )?;
        builder.property_mut(astParentFullName)?.cardinality = Cardinality::One(Some(
            Scalar::string(crate::CpgSchema::PropertyDefaults::String),
        ));
        builder.property_mut(astParentFullName)?.proto_id =
            Some(crate::ProtoIds::AstParentFullName);
        let declaration = builder.add_node(
            MODULE_NAME,
            "DECLARATION",
            NodeKind::Base,
            "This is the base node class for all declarations.",
        )?;
        builder.add_properties(declaration, &[name])?;
        let r#ref = builder.add_edge(
            MODULE_NAME,
            "REF",
            r#"This edge indicates that the source node is an identifier that denotes
access to the destination node. For example, an identifier may reference
a local variable.
"#,
        )?;
        builder.edge_mut(r#ref)?.proto_id = Some(crate::ProtoIds::Ref);
        Ok(Self {
            version,
            hash,
            code,
            isExternal,
            index,
            name,
            fullName,
            parserTypeName,
            value,
            content,
            astParentType,
            astParentFullName,
            declaration,
            r#ref,
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
