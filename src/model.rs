//! Schema construction with stable typed handles for cross-module mutations.
use crate::value::{Cardinality, EdgeCardinality, ValueType};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub path: String,
    pub message: String,
}
impl Error {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}
impl std::error::Error for Error {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PropertyId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeTypeId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeTypeId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstantCategoryId(pub usize);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleInfo {
    pub name: String,
    pub doc_index: i32,
    pub description: String,
    pub provided_by_frontend: bool,
    pub dependencies: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub value_type: ValueType,
    pub comment: Option<String>,
    pub module: String,
    pub proto_id: Option<i32>,
    pub cardinality: Cardinality,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Concrete,
    Base,
    Any,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainedNode {
    pub node_type: NodeTypeId,
    pub local_name: String,
    pub cardinality: Cardinality,
    pub comment: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeType {
    pub name: String,
    pub comment: Option<String>,
    pub module: String,
    pub kind: NodeKind,
    pub proto_id: Option<i32>,
    pub properties: Vec<PropertyId>,
    pub bases: Vec<NodeTypeId>,
    pub primary_key: Option<PropertyId>,
    pub starter_name: Option<String>,
    pub contained_nodes: Vec<ContainedNode>,
    pub marker_traits: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeType {
    pub name: String,
    pub comment: Option<String>,
    pub module: String,
    pub proto_id: Option<i32>,
    pub property: Option<PropertyId>,
    pub default_accessor_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub module: String,
    pub source: NodeTypeId,
    pub edge: EdgeTypeId,
    pub target: NodeTypeId,
    pub cardinality_out: EdgeCardinality,
    pub cardinality_in: EdgeCardinality,
    pub step_name_out: Option<String>,
    pub step_doc_out: Option<String>,
    pub step_name_in: Option<String>,
    pub step_doc_in: Option<String>,
}
impl Relation {
    pub fn new(source: NodeTypeId, edge: EdgeTypeId, target: NodeTypeId) -> Self {
        Self {
            module: String::new(),
            source,
            edge,
            target,
            cardinality_out: EdgeCardinality::List,
            cardinality_in: EdgeCardinality::List,
            step_name_out: None,
            step_doc_out: None,
            step_name_in: None,
            step_doc_in: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constant {
    pub name: String,
    pub value: String,
    pub value_type: ValueType,
    pub comment: Option<String>,
    pub module: String,
    pub proto_id: Option<i32>,
}
impl Constant {
    pub fn new(
        name: &str,
        value: &str,
        value_type: ValueType,
        comment: &str,
        module: &str,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            value_type,
            comment: nonempty(comment),
            module: module.into(),
            proto_id: None,
        }
    }
    pub fn with_proto_id(mut self, id: i32) -> Self {
        self.proto_id = Some(id);
        self
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantCategory {
    pub name: String,
    pub entries: Vec<Constant>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtoOptions {
    pub pkg: String,
    pub java_outer_classname: String,
    pub java_package: String,
    pub go_package: String,
    pub csharp_namespace: String,
    pub enum_names: BTreeMap<String, String>,
}
#[derive(Clone, Debug)]
pub struct Schema {
    pub domain_short_name: String,
    pub base_package: String,
    pub additional_traversal_packages: Vec<String>,
    pub modules: Vec<ModuleInfo>,
    pub properties: Vec<Property>,
    pub nodes: Vec<NodeType>,
    pub edges: Vec<EdgeType>,
    pub relations: Vec<Relation>,
    pub constants: Vec<ConstantCategory>,
    pub proto_options: Option<ProtoOptions>,
    pub duplicate_property_exemptions: BTreeSet<(NodeTypeId, PropertyId)>,
}
impl Schema {
    pub fn property(&self, id: PropertyId) -> Result<&Property> {
        self.properties
            .get(id.0)
            .ok_or_else(|| Error::new("property", "invalid handle"))
    }
    pub fn node(&self, id: NodeTypeId) -> Result<&NodeType> {
        self.nodes
            .get(id.0)
            .ok_or_else(|| Error::new("node", "invalid handle"))
    }
    pub fn edge(&self, id: EdgeTypeId) -> Result<&EdgeType> {
        self.edges
            .get(id.0)
            .ok_or_else(|| Error::new("edge", "invalid handle"))
    }
    pub fn find_node(&self, name: &str) -> Option<NodeTypeId> {
        self.nodes
            .iter()
            .position(|v| v.name == name)
            .map(NodeTypeId)
    }
    pub fn find_property(&self, name: &str) -> Option<PropertyId> {
        self.properties
            .iter()
            .position(|v| v.name == name)
            .map(PropertyId)
    }
    pub fn find_edge(&self, name: &str) -> Option<EdgeTypeId> {
        self.edges
            .iter()
            .position(|v| v.name == name)
            .map(EdgeTypeId)
    }
    pub fn ancestors(&self, node: NodeTypeId) -> Result<Vec<NodeTypeId>> {
        fn visit(
            s: &Schema,
            id: NodeTypeId,
            stack: &mut BTreeSet<NodeTypeId>,
            out: &mut Vec<NodeTypeId>,
        ) -> Result<()> {
            if !stack.insert(id) {
                return Err(Error::new("inheritance", "inheritance cycle"));
            }
            let mut bases = s.node(id)?.bases.clone();
            bases.sort_by_key(|v| s.nodes.get(v.0).map(|n| n.name.clone()));
            for b in &bases {
                if !out.contains(b) {
                    out.push(*b);
                }
            }
            for b in bases {
                visit(s, b, stack, out)?;
            }
            stack.remove(&id);
            Ok(())
        }
        let mut out = Vec::new();
        visit(self, node, &mut BTreeSet::new(), &mut out)?;
        Ok(out)
    }
    pub fn is_a(&self, node: NodeTypeId, parent: NodeTypeId) -> Result<bool> {
        Ok(node == parent
            || self.node(parent)?.kind == NodeKind::Any
            || self.ancestors(node)?.contains(&parent))
    }
    pub fn resolved_properties(&self, node: NodeTypeId) -> Result<Vec<PropertyId>> {
        let mut ids = self.node(node)?.properties.clone();
        for b in self.ancestors(node)? {
            for p in &self.node(b)?.properties {
                if !ids.contains(p) {
                    ids.push(*p);
                }
            }
        }
        for id in &ids {
            self.property(*id)?;
        }
        ids.sort_by_key(|p| self.properties[p.0].name.to_lowercase());
        Ok(ids)
    }
    pub fn resolved_contained_nodes(&self, node: NodeTypeId) -> Result<Vec<&ContainedNode>> {
        let mut v: Vec<_> = self.node(node)?.contained_nodes.iter().collect();
        v.sort_by_key(|c| c.local_name.to_lowercase());
        Ok(v)
    }
    pub fn concrete_nodes(&self) -> Vec<(NodeTypeId, &NodeType)> {
        let mut v: Vec<_> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.kind == NodeKind::Concrete)
            .map(|(i, n)| (NodeTypeId(i), n))
            .collect();
        v.sort_by_key(|(_, n)| n.name.to_lowercase());
        v
    }
    pub fn node_properties(&self) -> Result<Vec<PropertyId>> {
        let mut ids = BTreeSet::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if n.kind != NodeKind::Any {
                ids.extend(self.resolved_properties(NodeTypeId(i))?);
            }
        }
        let mut ids: Vec<_> = ids.into_iter().collect();
        ids.sort_by_key(|p| self.properties[p.0].name.to_lowercase());
        Ok(ids)
    }
    pub fn edge_properties(&self) -> Vec<PropertyId> {
        let mut ids: Vec<_> = self.edges.iter().filter_map(|e| e.property).collect();
        ids.sort();
        ids.dedup();
        ids.sort_by_key(|p| self.properties.get(p.0).map(|p| p.name.to_lowercase()));
        ids
    }
    pub fn verify_proto_ids_unique(&self) -> Result<()> {
        fn unique<'a>(
            values: impl IntoIterator<Item = (&'a str, Option<i32>)>,
            path: &str,
        ) -> Result<()> {
            let mut seen = BTreeMap::new();
            for (name, id) in values {
                if let Some(id) = id {
                    if let Some(other) = seen.insert(id, name) {
                        return Err(Error::new(
                            path,
                            format!("protoId {id} belongs to both {other} and {name}"),
                        ));
                    }
                }
            }
            Ok(())
        }
        unique(
            self.node_properties()?.iter().map(|id| {
                let p = &self.properties[id.0];
                (p.name.as_str(), p.proto_id)
            }),
            "node properties",
        )?;
        unique(
            self.edge_properties().iter().map(|id| {
                let p = &self.properties[id.0];
                (p.name.as_str(), p.proto_id)
            }),
            "edge properties",
        )?;
        unique(
            self.nodes
                .iter()
                .filter(|n| n.kind == NodeKind::Concrete)
                .map(|n| (n.name.as_str(), n.proto_id)),
            "node types",
        )?;
        unique(
            self.edges.iter().map(|e| (e.name.as_str(), e.proto_id)),
            "edge types",
        )?;
        for c in &self.constants {
            unique(
                c.entries.iter().map(|v| (v.name.as_str(), v.proto_id)),
                &c.name,
            )?;
        }
        Ok(())
    }
    /// Production schema validation. Separate from constructing and inspecting a schema.
    pub fn validate(&self) -> Result<()> {
        for (i, p) in self.properties.iter().enumerate() {
            if let Cardinality::One(Some(v)) = &p.cardinality {
                if v.value_type() != p.value_type {
                    return Err(Error::new(
                        format!("properties/{i}"),
                        "default type does not match property",
                    ));
                }
            }
        }
        let mut modules = BTreeSet::new();
        for m in &self.modules {
            for d in &m.dependencies {
                if !modules.contains(d) {
                    return Err(Error::new(
                        &m.name,
                        format!("missing earlier dependency {d}"),
                    ));
                }
            }
            if !modules.insert(m.name.clone()) {
                return Err(Error::new(&m.name, "duplicate module"));
            }
        }
        for (i, n) in self.nodes.iter().enumerate() {
            let properties = self.resolved_properties(NodeTypeId(i))?;
            if let Some(key) = n.primary_key {
                self.property(key)?;
                if !properties.contains(&key) {
                    return Err(Error::new(
                        &n.name,
                        "primary key is not a resolved property",
                    ));
                }
            }
            for c in &n.contained_nodes {
                self.node(c.node_type)?;
            }
        }
        for e in &self.edges {
            if let Some(p) = e.property {
                self.property(p)?;
            }
        }
        for r in &self.relations {
            self.node(r.source)?;
            self.node(r.target)?;
            self.edge(r.edge)?;
        }
        self.verify_proto_ids_unique()?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct SchemaBuilder {
    schema: Schema,
}
impl SchemaBuilder {
    pub fn new(domain: &str, package: &str, traversals: &[&str]) -> Self {
        let any = NodeType {
            name: "AnyNode".into(),
            comment: Some(
                "generic node base trait - use if you want to be explicitly unspecific".into(),
            ),
            module: String::new(),
            kind: NodeKind::Any,
            proto_id: None,
            properties: vec![],
            bases: vec![],
            primary_key: None,
            starter_name: Some("anyNode".into()),
            contained_nodes: vec![],
            marker_traits: vec![],
        };
        Self {
            schema: Schema {
                domain_short_name: domain.into(),
                base_package: package.into(),
                additional_traversal_packages: traversals.iter().map(|s| (*s).into()).collect(),
                modules: vec![],
                properties: vec![],
                nodes: vec![any],
                edges: vec![],
                relations: vec![],
                constants: vec![],
                proto_options: None,
                duplicate_property_exemptions: BTreeSet::new(),
            },
        }
    }
    pub fn any_node(&self) -> NodeTypeId {
        NodeTypeId(0)
    }
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
    pub fn add_module(&mut self, info: ModuleInfo) -> Result<()> {
        if self.schema.modules.iter().any(|m| m.name == info.name) {
            return Err(Error::new(&info.name, "module already constructed"));
        }
        for d in &info.dependencies {
            if !self.schema.modules.iter().any(|m| &m.name == d) {
                return Err(Error::new(
                    &info.name,
                    format!("dependency {d} not constructed"),
                ));
            }
        }
        self.schema.modules.push(info);
        Ok(())
    }
    pub fn add_property(
        &mut self,
        module: &str,
        name: &str,
        value_type: ValueType,
        comment: &str,
    ) -> Result<PropertyId> {
        if self.schema.properties.iter().any(|p| p.name == name) {
            return Err(Error::new(name, "duplicate property name"));
        }
        let id = PropertyId(self.schema.properties.len());
        self.schema.properties.push(Property {
            name: name.into(),
            value_type,
            comment: nonempty(comment),
            module: module.into(),
            proto_id: None,
            cardinality: Cardinality::ZeroOrOne,
        });
        Ok(id)
    }
    pub fn add_node(
        &mut self,
        module: &str,
        name: &str,
        kind: NodeKind,
        comment: &str,
    ) -> Result<NodeTypeId> {
        if self.schema.nodes.iter().any(|n| n.name == name) {
            return Err(Error::new(name, "duplicate node name"));
        }
        let id = NodeTypeId(self.schema.nodes.len());
        self.schema.nodes.push(NodeType {
            name: name.into(),
            comment: nonempty(comment),
            module: module.into(),
            kind,
            proto_id: None,
            properties: vec![],
            bases: vec![],
            primary_key: None,
            starter_name: Some(camel_case(name)),
            contained_nodes: vec![],
            marker_traits: vec![],
        });
        Ok(id)
    }
    pub fn add_edge(&mut self, module: &str, name: &str, comment: &str) -> Result<EdgeTypeId> {
        if self.schema.edges.iter().any(|e| e.name == name) {
            return Err(Error::new(name, "duplicate edge name"));
        }
        let id = EdgeTypeId(self.schema.edges.len());
        self.schema.edges.push(EdgeType {
            name: name.into(),
            comment: nonempty(comment),
            module: module.into(),
            proto_id: None,
            property: None,
            default_accessor_name: None,
        });
        Ok(id)
    }
    pub fn property_mut(&mut self, id: PropertyId) -> Result<&mut Property> {
        self.schema
            .properties
            .get_mut(id.0)
            .ok_or_else(|| Error::new("property", "invalid handle"))
    }
    pub fn node_mut(&mut self, id: NodeTypeId) -> Result<&mut NodeType> {
        self.schema
            .nodes
            .get_mut(id.0)
            .ok_or_else(|| Error::new("node", "invalid handle"))
    }
    pub fn edge_mut(&mut self, id: EdgeTypeId) -> Result<&mut EdgeType> {
        self.schema
            .edges
            .get_mut(id.0)
            .ok_or_else(|| Error::new("edge", "invalid handle"))
    }
    pub fn add_properties(&mut self, node: NodeTypeId, properties: &[PropertyId]) -> Result<()> {
        for p in properties {
            self.schema.property(*p)?;
        }
        let n = self.node_mut(node)?;
        for p in properties {
            if !n.properties.contains(p) {
                n.properties.push(*p);
            }
        }
        Ok(())
    }
    pub fn extend(&mut self, node: NodeTypeId, bases: &[NodeTypeId]) -> Result<()> {
        for b in bases {
            if self.schema.node(*b)?.kind == NodeKind::Concrete {
                return Err(Error::new("inheritance", "concrete node cannot be base"));
            }
        }
        let n = self.node_mut(node)?;
        for b in bases {
            if !n.bases.contains(b) {
                n.bases.push(*b);
            }
        }
        Ok(())
    }
    pub fn add_relation(&mut self, r: Relation) -> Result<()> {
        self.schema.node(r.source)?;
        self.schema.node(r.target)?;
        self.schema.edge(r.edge)?;
        if !self.schema.relations.contains(&r) {
            self.schema.relations.push(r);
        }
        Ok(())
    }
    pub fn add_contained(&mut self, node: NodeTypeId, c: ContainedNode) -> Result<()> {
        self.schema.node(c.node_type)?;
        let n = self.node_mut(node)?;
        if !n.contained_nodes.contains(&c) {
            n.contained_nodes.push(c);
        }
        Ok(())
    }
    pub fn add_constants(
        &mut self,
        name: &str,
        entries: Vec<Constant>,
    ) -> Result<ConstantCategoryId> {
        if let Some(i) = self.schema.constants.iter().position(|c| c.name == name) {
            self.schema.constants[i].entries.extend(entries);
            Ok(ConstantCategoryId(i))
        } else {
            let id = ConstantCategoryId(self.schema.constants.len());
            self.schema.constants.push(ConstantCategory {
                name: name.into(),
                entries,
            });
            Ok(id)
        }
    }
    pub fn set_proto_options(&mut self, options: ProtoOptions) {
        self.schema.proto_options = Some(options);
    }
    pub fn dont_warn_for_duplicate_property(
        &mut self,
        node: NodeTypeId,
        property: PropertyId,
    ) -> Result<()> {
        self.schema.node(node)?;
        self.schema.property(property)?;
        self.schema
            .duplicate_property_exemptions
            .insert((node, property));
        Ok(())
    }
    pub fn build(self) -> Result<Schema> {
        self.schema.verify_proto_ids_unique()?;
        Ok(self.schema)
    }
}
pub fn nonempty(text: &str) -> Option<String> {
    if text.is_empty() {
        None
    } else {
        Some(text.into())
    }
}
pub fn camel_case(text: &str) -> String {
    let mut out = String::new();
    for (i, w) in text.split('_').enumerate() {
        if i == 0 {
            out.push_str(&w.to_lowercase());
        } else {
            let mut c = w.chars();
            if let Some(h) = c.next() {
                out.extend(h.to_uppercase());
                out.push_str(&c.as_str().to_lowercase());
            }
        }
    }
    out
}

/// Adjacency metadata follows the source's independent incoming/outgoing sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdjacentNode {
    pub edge: EdgeTypeId,
    pub neighbor: NodeTypeId,
    pub cardinality: EdgeCardinality,
    pub custom_step_name: Option<String>,
    pub custom_step_doc: Option<String>,
}
impl Schema {
    pub fn properties(&self) -> Vec<(PropertyId, &Property)> {
        let mut out: Vec<_> = self
            .properties
            .iter()
            .enumerate()
            .map(|(i, p)| (PropertyId(i), p))
            .collect();
        out.sort_by_key(|(_, p)| p.name.to_lowercase());
        out
    }
    pub fn node_base_types(&self) -> Vec<(NodeTypeId, &NodeType)> {
        let mut out: Vec<_> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.kind == NodeKind::Base)
            .map(|(i, n)| (NodeTypeId(i), n))
            .collect();
        out.sort_by_key(|(_, n)| n.name.to_lowercase());
        out
    }
    pub fn edge_types(&self) -> Vec<(EdgeTypeId, &EdgeType)> {
        let mut out: Vec<_> = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| (EdgeTypeId(i), e))
            .collect();
        out.sort_by_key(|(_, e)| e.name.to_lowercase());
        out
    }
    pub fn adjacent_nodes(
        &self,
        node: NodeTypeId,
        direction: Direction,
        include_inherited: bool,
    ) -> Result<Vec<AdjacentNode>> {
        self.node(node)?;
        let mut owners = vec![node];
        if include_inherited {
            owners.extend(self.ancestors(node)?);
        }
        let mut result = vec![];
        for relation in &self.relations {
            let (owner, neighbor, cardinality, name, doc) = match direction {
                Direction::Out => (
                    relation.source,
                    relation.target,
                    relation.cardinality_out,
                    &relation.step_name_out,
                    &relation.step_doc_out,
                ),
                Direction::In => (
                    relation.target,
                    relation.source,
                    relation.cardinality_in,
                    &relation.step_name_in,
                    &relation.step_doc_in,
                ),
            };
            if owners.contains(&owner) {
                let adjacent = AdjacentNode {
                    edge: relation.edge,
                    neighbor,
                    cardinality,
                    custom_step_name: name.clone(),
                    custom_step_doc: doc.clone(),
                };
                if !result.contains(&adjacent) {
                    result.push(adjacent);
                }
            }
        }
        Ok(result)
    }
    pub fn subtypes(&self, node: NodeTypeId) -> Result<Vec<NodeTypeId>> {
        let def = self.node(node)?;
        if def.kind == NodeKind::Concrete {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for (i, n) in self.nodes.iter().enumerate() {
            let id = NodeTypeId(i);
            if n.kind != NodeKind::Any && self.is_a(id, node)? && id != node {
                out.push(id);
            }
        }
        out.sort_by_key(|id| self.nodes[id.0].name.clone());
        Ok(out)
    }
}
impl SchemaBuilder {
    pub fn add_marker_trait(&mut self, node: NodeTypeId, name: &str) -> Result<()> {
        let n = self.node_mut(node)?;
        if !n.marker_traits.iter().any(|s| s == name) {
            n.marker_traits.push(name.into());
        }
        Ok(())
    }
    pub fn without_starter(&mut self, node: NodeTypeId) -> Result<()> {
        self.node_mut(node)?.starter_name = None;
        Ok(())
    }
}
