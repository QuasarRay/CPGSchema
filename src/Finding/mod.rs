//! Native port of Finding.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    ContainedNode, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Result, SchemaBuilder,
};
use crate::value::{Cardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Finding";
pub const DOC_INDEX: i32 = 15;
pub const DESCRIPTION: &str = r#"
We allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to
be stored in the Findings Layer.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub key: PropertyId,
    pub finding: NodeTypeId,
    pub keyValuePair: NodeTypeId,
    pub evidenceDescription: PropertyId,
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
        let value = base.value;
        let key = builder.add_property(
            MODULE_NAME,
            "KEY",
            ValueType::String,
            "This property denotes a key of a key-value pair.",
        )?;
        builder.property_mut(key)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(key)?.proto_id = Some(crate::ProtoIds::Key);
        let finding = builder.add_node(
            MODULE_NAME,
            "FINDING",
            NodeKind::Concrete,
            r#"Finding nodes may be used to store analysis results in the graph
that are to be exposed to an end-user, e.g., information about
potential vulnerabilities or dangerous programming practices.
A Finding node may contain an abitrary list of key value pairs
that characterize the finding, as well as a list of nodes that
serve as evidence for the finding.
"#,
        )?;
        builder.node_mut(finding)?.proto_id = Some(crate::ProtoIds::Finding);
        builder.add_properties(finding, &[])?;
        let keyValuePair = builder.add_node(
            MODULE_NAME,
            "KEY_VALUE_PAIR",
            NodeKind::Concrete,
            "This node represents a key value pair, where both the key and the value are strings.",
        )?;
        builder.node_mut(keyValuePair)?.proto_id = Some(crate::ProtoIds::KeyValuePair);
        builder.add_properties(keyValuePair, &[key, value])?;
        let evidenceDescription = builder.add_property(MODULE_NAME, "EVIDENCE_DESCRIPTION", ValueType::String, r#"Optional description for nodes in evidence. Used to give a hint about the kind of evidence
provided by a node. The evidence description and evidence nodes are associated by index.
"#)?;
        builder.property_mut(evidenceDescription)?.cardinality = Cardinality::List;
        builder.property_mut(evidenceDescription)?.proto_id =
            Some(crate::ProtoIds::EvidenceDescription);
        builder.add_contained(
            finding,
            ContainedNode {
                node_type: builder.any_node(),
                local_name: "evidence".into(),
                cardinality: Cardinality::List,
                comment: None,
            },
        )?;
        builder.add_contained(
            finding,
            ContainedNode {
                node_type: keyValuePair,
                local_name: "keyValuePairs".into(),
                cardinality: Cardinality::List,
                comment: None,
            },
        )?;
        builder.add_properties(finding, &[evidenceDescription])?;
        Ok(Self {
            key,
            finding,
            keyValuePair,
            evidenceDescription,
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
