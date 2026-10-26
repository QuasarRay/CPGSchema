//! Native port of MetaData.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    Constant, ConstantCategoryId, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Result,
    SchemaBuilder,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "MetaData";
pub const DOC_INDEX: i32 = 2;
pub const DESCRIPTION: &str = r#"
The Meta Data Layer contains information about CPG creation. In particular,
it indicates which language frontend generated the CPG and which overlays
have been applied. The layer consists of a single node - the Meta Data node -
and language frontends MUST create this node. Overlay creators MUST edit
this node to indicate that a layer has been successfully applied in all
cases where applying the layer more than once is prohibitive.
"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub overlays: PropertyId,
    pub language: PropertyId,
    pub root: PropertyId,
    pub metaData: NodeTypeId,
    pub languages: ConstantCategoryId,
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
        let hash = base.hash;
        let version = base.version;
        let overlays = builder.add_property(
            MODULE_NAME,
            "OVERLAYS",
            ValueType::String,
            r#"The field contains the names of the overlays applied to this CPG, in order of their
application. Names are free-form strings, that is, this specification does not
dictate them but rather requires tool producers and consumers to communicate them
between each other.
"#,
        )?;
        builder.property_mut(overlays)?.cardinality = Cardinality::List;
        builder.property_mut(overlays)?.proto_id = Some(crate::ProtoIds::Overlays);
        let language = builder.add_property(
            MODULE_NAME,
            "LANGUAGE",
            ValueType::String,
            r#"This field indicates which CPG language frontend generated the CPG.
Frontend developers may freely choose a value that describes their frontend
so long as it is not used by an existing frontend. Reserved values are to date:
C, LLVM, GHIDRA, PHP.
"#,
        )?;
        builder.property_mut(language)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(language)?.proto_id = Some(crate::ProtoIds::Language);
        let root = builder.add_property(
            MODULE_NAME,
            "ROOT",
            ValueType::String,
            "The path to the root directory of the source/binary this CPG is generated from.",
        )?;
        builder.property_mut(root)?.proto_id = Some(crate::ProtoIds::Root);
        builder.property_mut(root)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        let metaData = builder.add_node(
            MODULE_NAME,
            "META_DATA",
            NodeKind::Concrete,
            r#"
This node contains the CPG meta data. Exactly one node of this type
MUST exist per CPG. The `HASH` property MAY contain a hash value calculated
over the source files this CPG was generated from. The `VERSION` MUST be
set to the version of the specification ("1.1"). The language field indicates
which language frontend was used to generate the CPG and the list property
`OVERLAYS` specifies which overlays have been applied to the CPG.
 "#,
        )?;
        builder.node_mut(metaData)?.proto_id = Some(crate::ProtoIds::MetaData);
        builder.add_properties(metaData, &[language, version, overlays, hash, root])?;
        let languages = builder.add_constants(
            "Languages",
            vec![
                Constant::new("JAVA", "JAVA", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::Java),
                Constant::new(
                    "JAVASCRIPT",
                    "JAVASCRIPT",
                    ValueType::String,
                    "",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::JavaScript),
                Constant::new("GOLANG", "GOLANG", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::Golang),
                Constant::new("CSHARP", "CSHARP", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::CSharp),
                Constant::new("ABAP", "ABAP", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::ABAP),
                Constant::new("C", "C", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::C),
                Constant::new("PYTHON", "PYTHON", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::Python),
                Constant::new("LLVM", "LLVM", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::LLVM),
                Constant::new("PHP", "PHP", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::PHP),
                Constant::new(
                    "FUZZY_TEST_LANG",
                    "FUZZY_TEST_LANG",
                    ValueType::String,
                    "",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::FuzzyTestLang),
                Constant::new(
                    "GHIDRA",
                    "GHIDRA",
                    ValueType::String,
                    "generic reverse engineering framework",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::Ghidra),
                Constant::new("KOTLIN", "KOTLIN", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::Kotlin),
                Constant::new(
                    "NEWC",
                    "NEWC",
                    ValueType::String,
                    "Eclipse CDT based parser for C/C++",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::NewC),
                Constant::new(
                    "JAVASRC",
                    "JAVASRC",
                    ValueType::String,
                    "Source-based front-end for Java",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::JavaSrc),
                Constant::new(
                    "PYTHONSRC",
                    "PYTHONSRC",
                    ValueType::String,
                    "Source-based front-end for Python",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::PythonSrc),
                Constant::new(
                    "JSSRC",
                    "JSSRC",
                    ValueType::String,
                    "Source-based JS frontend based on Babel",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::JsSrc),
                Constant::new(
                    "RUBYSRC",
                    "RUBYSRC",
                    ValueType::String,
                    "Source-based frontend for Ruby",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::RubySrc),
                Constant::new(
                    "SWIFTSRC",
                    "SWIFTSRC",
                    ValueType::String,
                    "Source-based frontend for Swift",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::SwiftSrc),
                Constant::new(
                    "CSHARPSRC",
                    "CSHARPSRC",
                    ValueType::String,
                    "Source-based frontend for C# and .NET",
                    MODULE_NAME,
                )
                .with_proto_id(crate::ProtoIds::CSharpSrc),
                Constant::new("RUST", "RUST", ValueType::String, "", MODULE_NAME)
                    .with_proto_id(crate::ProtoIds::Rust),
            ],
        )?;
        Ok(Self {
            overlays,
            language,
            root,
            metaData,
            languages,
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

/// Removed upstream language identifier, previously Solidity.
pub const RESERVED_LANGUAGE_PROTO_IDS: &[i32] = &[16];
