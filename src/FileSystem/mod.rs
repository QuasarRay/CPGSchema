//! Native port of FileSystem.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Result, SchemaBuilder,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "FileSystem";
pub const DOC_INDEX: i32 = 3;
pub const DESCRIPTION: &str = r#"
CPGs are created from sets of files and the File System Layer describes the layout of
these files, that is, it provides information about source files
and shared objects for source-based and machine-code-based frontends
respectively. The purpose of including this information in the CPG
is to allow nodes of the graph to be mapped back to file system locations.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub filename: PropertyId,
    pub lineNumber: PropertyId,
    pub columnNumber: PropertyId,
    pub lineNumberEnd: PropertyId,
    pub columnNumberEnd: PropertyId,
    pub sourceFile: EdgeTypeId,
    pub file: NodeTypeId,
    pub offset: PropertyId,
    pub offsetEnd: PropertyId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder, base: &crate::Base::Schema) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec!["Base".into()],
        })?;
        let content = base.content;
        let hash = base.hash;
        let name = base.name;
        let filename = builder.add_property(
            MODULE_NAME,
            "FILENAME",
            ValueType::String,
            r#"The path of the source file this node was generated from, relative to the root
path in the meta data node. This field must be set but may be set to the value `<unknown>` to
indicate that no source file can be associated with the node, e.g., because the node represents
an entity known to exist because it is referenced, but for which the file that is is declared in
is unknown.
"#,
        )?;
        builder.property_mut(filename)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(filename)?.proto_id = Some(crate::ProtoIds::FileName);
        let lineNumber = builder.add_property(
            MODULE_NAME,
            "LINE_NUMBER",
            ValueType::Int,
            r#"This optional field provides the line number of the program construct
represented by the node.
"#,
        )?;
        builder.property_mut(lineNumber)?.proto_id = Some(crate::ProtoIds::LineNumber);
        let columnNumber = builder.add_property(
            MODULE_NAME,
            "COLUMN_NUMBER",
            ValueType::Int,
            r#"
This optional fields provides the column number of the program construct
represented by the node.
"#,
        )?;
        builder.property_mut(columnNumber)?.proto_id = Some(crate::ProtoIds::ColumnNumber);
        let lineNumberEnd = builder.add_property(
            MODULE_NAME,
            "LINE_NUMBER_END",
            ValueType::Int,
            r#"
This optional fields provides the line number at which the program construct
represented by the node ends.
        "#,
        )?;
        builder.property_mut(lineNumberEnd)?.proto_id = Some(crate::ProtoIds::LineNumberEnd);
        let columnNumberEnd = builder.add_property(
            MODULE_NAME,
            "COLUMN_NUMBER_END",
            ValueType::Int,
            r#"
This optional fields provides the column number at which the program construct
represented by the node ends.
        "#,
        )?;
        builder.property_mut(columnNumberEnd)?.proto_id = Some(crate::ProtoIds::ColumnNumberEnd);
        let sourceFile = builder.add_edge(
            MODULE_NAME,
            "SOURCE_FILE",
            r#"This edge connects a node to the node that represents its source file. These
edges MUST not be created by the language frontend but are automatically
created based on `FILENAME` fields.
"#,
        )?;
        builder.edge_mut(sourceFile)?.proto_id = Some(crate::ProtoIds::SourceFile);
        let file = builder.add_node(
            MODULE_NAME,
            "FILE",
            NodeKind::Concrete,
            r#"File nodes represent source files or a shared objects from which the CPG
was generated. File nodes serve as indices, that is, they allow looking up all
elements of the code by file.

For each file, the graph CAN contain exactly one File node, if not File nodes
are created as indicated by `FILENAME` property of other nodes.
As file nodes are root nodes of abstract syntax tress, they are AstNodes and
their order field is set to 0. This is because they have no sibling nodes,
not because they are the first node of the AST.
"#,
        )?;
        builder.node_mut(file)?.proto_id = Some(crate::ProtoIds::File);
        builder.add_properties(file, &[name, hash, content])?;
        builder.node_mut(file)?.primary_key = Some(name);
        let offset = builder.add_property(
            MODULE_NAME,
            "OFFSET",
            ValueType::Int,
            r#"
Start offset into the CONTENT property of the corresponding FILE node.
The offset is such that parts of the content can easily
be accessed via `content.substring(offset, offsetEnd)`.
This means that the offset must be measured in utf16 encoding (i.e. neither in
characters/codeunits nor in byte-offsets into a utf8 encoding).
E.g. for METHOD nodes this start offset points to the start of the methods
source code in the string holding the source code of the entire file.
        "#,
        )?;
        builder.property_mut(offset)?.proto_id = Some(crate::ProtoIds::Offset);
        let offsetEnd = builder.add_property(
            MODULE_NAME,
            "OFFSET_END",
            ValueType::Int,
            r#"
End offset (exclusive) into the CONTENT property of the corresponding FILE node.
See OFFSET documentation for finer details.
E.g. for METHOD nodes this end offset points to the first code position which is
not part of the method.
        "#,
        )?;
        builder.property_mut(offsetEnd)?.proto_id = Some(crate::ProtoIds::OffsetEnd);
        Ok(Self {
            filename,
            lineNumber,
            columnNumber,
            lineNumberEnd,
            columnNumberEnd,
            sourceFile,
            file,
            offset,
            offsetEnd,
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
