//! Lossless graph instances and production validation. Validation never mutates input.
use crate::{
    model::{EdgeTypeId, Error, NodeKind, NodeTypeId, Result, Schema},
    value::{Cardinality, EdgeCardinality, JavaString, PropertyValue, Scalar},
};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeId(pub String);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainedValue {
    One(Option<NodeId>),
    List(Vec<NodeId>),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub properties: BTreeMap<String, PropertyValue>,
    pub contained: BTreeMap<String, ContainedValue>,
}
impl Node {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: NodeId(id.into()),
            label: label.into(),
            properties: BTreeMap::new(),
            contained: BTreeMap::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub label: String,
    pub properties: BTreeMap<String, PropertyValue>,
}
impl Edge {
    pub fn new(
        id: impl Into<String>,
        source: NodeId,
        target: NodeId,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: EdgeId(id.into()),
            source,
            target,
            label: label.into(),
            properties: BTreeMap::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    pub format_version: u32,
    pub schema_revision: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
impl Default for Graph {
    fn default() -> Self {
        Self {
            format_version: 1,
            schema_revision: crate::SOURCE_REVISION.into(),
            nodes: vec![],
            edges: vec![],
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstructionStage {
    Frontend,
    Materialized,
}
#[derive(Clone, Debug)]
pub struct ValidationOptions {
    pub stage: ConstructionStage,
    pub layers: BTreeSet<String>,
}
impl ValidationOptions {
    pub fn new(stage: ConstructionStage, layers: impl IntoIterator<Item = String>) -> Self {
        Self {
            stage,
            layers: layers.into_iter().collect(),
        }
    }
    pub fn all(schema: &Schema, stage: ConstructionStage) -> Self {
        Self::new(stage, schema.modules.iter().map(|m| m.name.clone()))
    }
    pub fn includes(&self, module: &str) -> bool {
        module.is_empty() || self.layers.contains(module)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    Format,
    Layer,
    Identity,
    Reference,
    NodeType,
    EdgeType,
    Property,
    Cardinality,
    ContainedNode,
    DerivedRelationship,
    SourceContract,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub path: String,
    pub entity: Option<String>,
    pub message: String,
}
fn report(
    out: &mut Vec<Diagnostic>,
    kind: DiagnosticKind,
    path: impl Into<String>,
    entity: Option<&str>,
    message: impl Into<String>,
) {
    out.push(Diagnostic {
        kind,
        path: path.into(),
        entity: entity.map(str::to_owned),
        message: message.into(),
    });
}
fn string<'a>(node: &'a Node, name: &str) -> Option<&'a JavaString> {
    match node.properties.get(name) {
        Some(PropertyValue::Scalar(Scalar::String(v))) => Some(v),
        _ => None,
    }
}
fn strings<'a>(node: &'a Node, name: &str) -> Vec<&'a JavaString> {
    match node.properties.get(name) {
        Some(PropertyValue::List(v)) => v
            .iter()
            .filter_map(|v| {
                if let Scalar::String(s) = v {
                    Some(s)
                } else {
                    None
                }
            })
            .collect(),
        _ => vec![],
    }
}
fn integer(node: &Node, name: &str) -> Option<i32> {
    match node.properties.get(name) {
        Some(PropertyValue::Scalar(Scalar::Int(v))) => Some(*v),
        _ => None,
    }
}
fn meaningful(s: &JavaString) -> bool {
    !s.0.is_empty() && s != &JavaString::from("<empty>") && s != &JavaString::from("<unknown>")
}
fn value_errors(
    out: &mut Vec<Diagnostic>,
    p: &crate::model::Property,
    value: Option<&PropertyValue>,
    path: &str,
    entity: &str,
) {
    match (&p.cardinality, value) {
        (Cardinality::One(_), None) => report(
            out,
            DiagnosticKind::Cardinality,
            path,
            Some(entity),
            "mandatory property is absent; applying defaults is an explicit operation",
        ),
        (Cardinality::List, Some(PropertyValue::Scalar(_)))
        | (Cardinality::ZeroOrOne | Cardinality::One(_), Some(PropertyValue::List(_))) => report(
            out,
            DiagnosticKind::Cardinality,
            path,
            Some(entity),
            "property shape does not match cardinality",
        ),
        _ => {}
    }
    if let Some(v) = value {
        for s in v.scalars() {
            if s.value_type() != p.value_type {
                report(
                    out,
                    DiagnosticKind::Property,
                    path,
                    Some(entity),
                    format!(
                        "expected {}, got {}",
                        p.value_type.name(),
                        s.value_type().name()
                    ),
                );
            }
        }
    }
}
/// Return every discoverable diagnostic; callers choose applicable layers and construction stage.
pub fn validate_graph(
    schema: &Schema,
    graph: &Graph,
    options: &ValidationOptions,
) -> Vec<Diagnostic> {
    let mut out = vec![];
    if graph.format_version != 1 {
        report(
            &mut out,
            DiagnosticKind::Format,
            "/formatVersion",
            None,
            "unsupported graph format version",
        );
    }
    if graph.schema_revision != crate::SOURCE_REVISION {
        report(
            &mut out,
            DiagnosticKind::Format,
            "/schemaRevision",
            None,
            "graph schema revision differs from this implementation",
        );
    }
    for layer in &options.layers {
        match schema.modules.iter().find(|m| &m.name == layer) {
            None => report(
                &mut out,
                DiagnosticKind::Layer,
                "/layers",
                None,
                format!("unknown layer {layer}"),
            ),
            Some(m) => {
                for dep in &m.dependencies {
                    if !options.layers.contains(dep) {
                        report(
                            &mut out,
                            DiagnosticKind::Layer,
                            "/layers",
                            None,
                            format!("{layer} requires {dep}"),
                        );
                    }
                }
            }
        }
    }
    let mut nodes = BTreeMap::new();
    let mut types = BTreeMap::new();
    for (i, n) in graph.nodes.iter().enumerate() {
        let path = format!("/nodes/{i}");
        if n.id.0.is_empty() || nodes.insert(n.id.clone(), n).is_some() {
            report(
                &mut out,
                DiagnosticKind::Identity,
                format!("{path}/id"),
                Some(&n.id.0),
                "empty or duplicate node id",
            );
        }
        let Some(ty) = schema.find_node(&n.label) else {
            report(
                &mut out,
                DiagnosticKind::NodeType,
                &path,
                Some(&n.id.0),
                "undeclared node type",
            );
            continue;
        };
        let Ok(def) = schema.node(ty) else { continue };
        if def.kind != NodeKind::Concrete {
            report(
                &mut out,
                DiagnosticKind::NodeType,
                &path,
                Some(&n.id.0),
                "abstract node type cannot be instantiated",
            );
        }
        if !options.includes(&def.module) {
            report(
                &mut out,
                DiagnosticKind::Layer,
                &path,
                Some(&n.id.0),
                format!("node belongs to inactive layer {}", def.module),
            );
        }
        types.insert(n.id.clone(), ty);
        match schema.resolved_properties(ty) {
            Err(e) => report(
                &mut out,
                DiagnosticKind::SourceContract,
                &path,
                Some(&n.id.0),
                e.to_string(),
            ),
            Ok(ids) => {
                let allowed: BTreeMap<_, _> = ids
                    .iter()
                    .filter_map(|id| schema.property(*id).ok())
                    .filter(|p| p.module != "ProtoSerialize")
                    .map(|p| (p.name.as_str(), p))
                    .collect();
                for (name, value) in &n.properties {
                    if let Some(p) = allowed.get(name.as_str()) {
                        value_errors(
                            &mut out,
                            p,
                            Some(value),
                            &format!("{path}/properties/{name}"),
                            &n.id.0,
                        );
                    } else {
                        report(
                            &mut out,
                            DiagnosticKind::Property,
                            format!("{path}/properties/{name}"),
                            Some(&n.id.0),
                            "property is not declared on this node",
                        );
                    }
                }
                for p in allowed.values() {
                    if !n.properties.contains_key(&p.name) {
                        value_errors(
                            &mut out,
                            p,
                            None,
                            &format!("{path}/properties/{}", p.name),
                            &n.id.0,
                        );
                    }
                }
            }
        }
        for name in n.contained.keys() {
            if !def.contained_nodes.iter().any(|c| &c.local_name == name) {
                report(
                    &mut out,
                    DiagnosticKind::ContainedNode,
                    format!("{path}/contained/{name}"),
                    Some(&n.id.0),
                    "undeclared contained-node field",
                );
            }
        }
        for c in &def.contained_nodes {
            let value = n.contained.get(&c.local_name);
            let cp = format!("{path}/contained/{}", c.local_name);
            match (&c.cardinality, value) {
                (Cardinality::One(_), None) => report(
                    &mut out,
                    DiagnosticKind::Cardinality,
                    &cp,
                    Some(&n.id.0),
                    "mandatory contained-node field is absent",
                ),
                (Cardinality::List, Some(ContainedValue::One(_)))
                | (Cardinality::One(_) | Cardinality::ZeroOrOne, Some(ContainedValue::List(_))) => {
                    report(
                        &mut out,
                        DiagnosticKind::Cardinality,
                        &cp,
                        Some(&n.id.0),
                        "contained-node cardinality mismatch",
                    )
                }
                (Cardinality::ZeroOrOne, Some(ContainedValue::One(None))) => report(
                    &mut out,
                    DiagnosticKind::ContainedNode,
                    &cp,
                    Some(&n.id.0),
                    "optional absent reference must be omitted",
                ),
                _ => {}
            }
        }
        if options.stage == ConstructionStage::Frontend
            && matches!(n.label.as_str(), "NAMESPACE" | "METHOD_PARAMETER_OUT")
        {
            report(
                &mut out,
                DiagnosticKind::SourceContract,
                &path,
                Some(&n.id.0),
                "source forbids this loader-created node in frontend output",
            );
        }
    }
    for (i, n) in graph.nodes.iter().enumerate() {
        if let Some(ty) = types.get(&n.id) {
            if let Ok(def) = schema.node(*ty) {
                for c in &def.contained_nodes {
                    let refs: Vec<_> = match n.contained.get(&c.local_name) {
                        Some(ContainedValue::One(Some(id))) => vec![id],
                        Some(ContainedValue::List(ids)) => ids.iter().collect(),
                        _ => vec![],
                    };
                    for id in refs {
                        match types.get(id) {
                            None => report(
                                &mut out,
                                DiagnosticKind::Reference,
                                format!("/nodes/{i}/contained/{}", c.local_name),
                                Some(&n.id.0),
                                format!("missing contained node {}", id.0),
                            ),
                            Some(target) => {
                                if !schema.is_a(*target, c.node_type).unwrap_or(false) {
                                    report(
                                        &mut out,
                                        DiagnosticKind::ContainedNode,
                                        format!("/nodes/{i}/contained/{}", c.local_name),
                                        Some(&n.id.0),
                                        "contained node has incompatible type",
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // NodeRef values are graph references, including values inside a scalar List.
    fn check_refs(
        value: &Scalar,
        nodes: &BTreeMap<NodeId, &Node>,
        path: &str,
        entity: &str,
        out: &mut Vec<Diagnostic>,
    ) {
        match value {
            Scalar::NodeRef(id) if !nodes.contains_key(&NodeId(id.clone())) => report(
                out,
                DiagnosticKind::Reference,
                path,
                Some(entity),
                format!("missing referenced node {id}"),
            ),
            Scalar::List(values) => {
                for (i, value) in values.iter().enumerate() {
                    check_refs(value, nodes, &format!("{path}/value/{i}"), entity, out);
                }
            }
            _ => {}
        }
    }
    for (i, node) in graph.nodes.iter().enumerate() {
        for (name, value) in &node.properties {
            for scalar in value.scalars() {
                check_refs(
                    scalar,
                    &nodes,
                    &format!("/nodes/{i}/properties/{name}"),
                    &node.id.0,
                    &mut out,
                );
            }
        }
    }
    for (i, edge) in graph.edges.iter().enumerate() {
        for (name, value) in &edge.properties {
            for scalar in value.scalars() {
                check_refs(
                    scalar,
                    &nodes,
                    &format!("/edges/{i}/properties/{name}"),
                    &edge.id.0,
                    &mut out,
                );
            }
        }
    }
    let mut edge_ids = BTreeSet::new();
    for (i, e) in graph.edges.iter().enumerate() {
        let path = format!("/edges/{i}");
        if e.id.0.is_empty() || !edge_ids.insert(e.id.clone()) {
            report(
                &mut out,
                DiagnosticKind::Identity,
                &path,
                Some(&e.id.0),
                "empty or duplicate edge id",
            );
        }
        let endpoints = types.get(&e.source).zip(types.get(&e.target));
        if endpoints.is_none() {
            report(
                &mut out,
                DiagnosticKind::Reference,
                &path,
                Some(&e.id.0),
                "missing source or target node",
            );
        }
        let Some(ety) = schema.find_edge(&e.label) else {
            report(
                &mut out,
                DiagnosticKind::EdgeType,
                &path,
                Some(&e.id.0),
                "undeclared edge type",
            );
            continue;
        };
        let Ok(def) = schema.edge(ety) else { continue };
        if !options.includes(&def.module) {
            report(
                &mut out,
                DiagnosticKind::Layer,
                &path,
                Some(&e.id.0),
                "edge belongs to inactive layer",
            );
        }
        if let Some((source, target)) = endpoints {
            if !schema.relations.iter().any(|r| {
                r.edge == ety
                    && options.includes(&r.module)
                    && schema.is_a(*source, r.source).unwrap_or(false)
                    && schema.is_a(*target, r.target).unwrap_or(false)
            }) {
                report(
                    &mut out,
                    DiagnosticKind::EdgeType,
                    &path,
                    Some(&e.id.0),
                    "source/edge/target combination is not declared",
                );
            }
        }
        for (name, v) in &e.properties {
            if let Some(p) = def
                .property
                .and_then(|p| schema.property(p).ok())
                .filter(|p| &p.name == name)
            {
                value_errors(
                    &mut out,
                    p,
                    Some(v),
                    &format!("{path}/properties/{name}"),
                    &e.id.0,
                );
            } else {
                report(
                    &mut out,
                    DiagnosticKind::Property,
                    format!("{path}/properties/{name}"),
                    Some(&e.id.0),
                    "undeclared edge property",
                );
            }
        }
        if let Some(p) = def.property.and_then(|p| schema.property(p).ok()) {
            if !e.properties.contains_key(&p.name) {
                value_errors(
                    &mut out,
                    p,
                    None,
                    &format!("{path}/properties/{}", p.name),
                    &e.id.0,
                );
            }
        }
        if options.stage == ConstructionStage::Frontend
            && (def.module == "Shortcuts"
                || matches!(
                    e.label.as_str(),
                    "SOURCE_FILE" | "ALIAS_OF" | "INHERITS_FROM"
                ))
        {
            report(
                &mut out,
                DiagnosticKind::SourceContract,
                &path,
                Some(&e.id.0),
                "source forbids this derived edge in frontend output",
            );
        }
    }
    for (i, n) in graph.nodes.iter().enumerate() {
        if let Some(ty) = types.get(&n.id) {
            for incoming in [false, true] {
                let mut groups: BTreeMap<(EdgeTypeId, NodeTypeId), EdgeCardinality> =
                    BTreeMap::new();
                for r in &schema.relations {
                    if !options.includes(&r.module) {
                        continue;
                    }
                    let (owner, neighbor, card) = if incoming {
                        (r.target, r.source, r.cardinality_in)
                    } else {
                        (r.source, r.target, r.cardinality_out)
                    };
                    if schema.is_a(*ty, owner).unwrap_or(false) {
                        groups
                            .entry((r.edge, neighbor))
                            .and_modify(|v| *v = v.union(card))
                            .or_insert(card);
                    }
                }
                for ((edge, neighbor), mut card) in groups {
                    for r in &schema.relations {
                        if r.edge != edge || !options.includes(&r.module) {
                            continue;
                        }
                        let (owner, other, c) = if incoming {
                            (r.target, r.source, r.cardinality_in)
                        } else {
                            (r.source, r.target, r.cardinality_out)
                        };
                        if schema.is_a(*ty, owner).unwrap_or(false)
                            && schema.is_a(neighbor, other).unwrap_or(false)
                        {
                            card = card.union(c);
                        }
                    }
                    if card == EdgeCardinality::List {
                        continue;
                    }
                    let count = graph
                        .edges
                        .iter()
                        .filter(|e| {
                            schema.find_edge(&e.label) == Some(edge)
                                && if incoming {
                                    e.target == n.id
                                        && types.get(&e.source).is_some_and(|t| {
                                            schema.is_a(*t, neighbor).unwrap_or(false)
                                        })
                                } else {
                                    e.source == n.id
                                        && types.get(&e.target).is_some_and(|t| {
                                            schema.is_a(*t, neighbor).unwrap_or(false)
                                        })
                                }
                        })
                        .count();
                    if !(if options.stage == ConstructionStage::Frontend {
                        count <= 1
                    } else {
                        card.accepts(count)
                    }) {
                        report(
                            &mut out,
                            DiagnosticKind::Cardinality,
                            format!("/nodes/{i}"),
                            Some(&n.id.0),
                            format!(
                                "{} {} adjacency to {} expects {card:?}, got {count}",
                                if incoming { "incoming" } else { "outgoing" },
                                schema.edges[edge.0].name,
                                schema.nodes[neighbor.0].name
                            ),
                        );
                    }
                }
            }
        }
    }
    source_contracts(schema, graph, options, &mut out);
    out
}
fn source_contracts(
    schema: &Schema,
    graph: &Graph,
    options: &ValidationOptions,
    out: &mut Vec<Diagnostic>,
) {
    if options.includes("MetaData") {
        let meta: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.label == "META_DATA")
            .collect();
        if meta.len() != 1 {
            report(
                out,
                DiagnosticKind::SourceContract,
                "/nodes",
                None,
                "exactly one META_DATA node is required",
            );
        }
        for n in meta {
            if string(n, "VERSION") != Some(&JavaString::from("1.1")) {
                report(
                    out,
                    DiagnosticKind::SourceContract,
                    "/nodes",
                    Some(&n.id.0),
                    "META_DATA.VERSION must be 1.1",
                );
            }
        }
    }
    if options.stage != ConstructionStage::Materialized {
        return;
    }
    let has_edge = |src: &Node, dst: &Node, label: &str| {
        graph
            .edges
            .iter()
            .any(|e| e.source == src.id && e.target == dst.id && e.label == label)
    };
    for (i, n) in graph.nodes.iter().enumerate() {
        let path = format!("/nodes/{i}");
        let mut link = |field: &str,
                        target_label: &str,
                        target_field: &str,
                        edge: Option<&str>,
                        minimum: usize,
                        maximum: Option<usize>,
                        layer: &str| {
            if !options.includes(layer) {
                return;
            }
            let mut vals = strings(n, field);
            if let Some(v) = string(n, field) {
                vals.push(v);
            }
            for value in vals {
                if !meaningful(value) {
                    continue;
                }
                let targets: Vec<_> = graph
                    .nodes
                    .iter()
                    .filter(|t| t.label == target_label && string(t, target_field) == Some(value))
                    .collect();
                if targets.len() < minimum || maximum.is_some_and(|max| targets.len() > max) {
                    report(
                        out,
                        DiagnosticKind::Reference,
                        format!("{path}/properties/{field}"),
                        Some(&n.id.0),
                        format!(
                            "{field} requires {minimum}..{maximum:?} {target_label} targets, got {}",
                            targets.len()
                        ),
                    );
                }
                if let Some(label) = edge {
                    for target in targets {
                        let allowed = schema
                            .find_node(&n.label)
                            .zip(schema.find_node(&target.label))
                            .zip(schema.find_edge(label))
                            .is_some_and(|((s, t), e)| {
                                schema.relations.iter().any(|r| {
                                    r.edge == e
                                        && schema.is_a(s, r.source).unwrap_or(false)
                                        && schema.is_a(t, r.target).unwrap_or(false)
                                })
                            });
                        if allowed && !has_edge(n, target, label) {
                            report(
                                out,
                                DiagnosticKind::DerivedRelationship,
                                &path,
                                Some(&n.id.0),
                                format!("missing derived {label} relationship to {}", target.id.0),
                            );
                        }
                    }
                }
            }
        };
        link(
            "TYPE_DECL_FULL_NAME",
            "TYPE_DECL",
            "FULL_NAME",
            Some("REF"),
            1,
            Some(1),
            "Type",
        );
        link(
            "INHERITS_FROM_TYPE_FULL_NAME",
            "TYPE",
            "FULL_NAME",
            Some("INHERITS_FROM"),
            1,
            None,
            "Type",
        );
        link(
            "ALIAS_TYPE_FULL_NAME",
            "TYPE",
            "FULL_NAME",
            Some("ALIAS_OF"),
            0,
            None,
            "Type",
        );
        link(
            "METHOD_FULL_NAME",
            "METHOD",
            "FULL_NAME",
            Some(if n.label == "CALL" { "CALL" } else { "REF" }),
            1,
            Some(1),
            "CallGraph",
        );
        if string(n, "FILENAME").is_some_and(meaningful) {
            link(
                "FILENAME",
                "FILE",
                "NAME",
                Some("SOURCE_FILE"),
                1,
                None,
                "FileSystem",
            );
        }
        if n.label == "NAMESPACE_BLOCK" {
            link(
                "NAME",
                "NAMESPACE",
                "NAME",
                Some("REF"),
                1,
                None,
                "Namespace",
            );
        }
        if options.includes("Ast") {
            if let (Some(label), Some(fullname)) = (
                string(n, "AST_PARENT_TYPE"),
                string(n, "AST_PARENT_FULL_NAME"),
            ) {
                if meaningful(label) && meaningful(fullname) {
                    let label = label.to_utf8().ok();
                    let parents: Vec<_> = graph
                        .nodes
                        .iter()
                        .filter(|p| {
                            Some(p.label.as_str()) == label.as_deref()
                                && string(p, "FULL_NAME") == Some(fullname)
                        })
                        .collect();
                    if parents.is_empty() {
                        report(
                            out,
                            DiagnosticKind::Reference,
                            &path,
                            Some(&n.id.0),
                            "AST parent identity does not resolve",
                        );
                    }
                    for parent in parents {
                        if !has_edge(parent, n, "AST") {
                            report(
                                out,
                                DiagnosticKind::DerivedRelationship,
                                &path,
                                Some(&n.id.0),
                                "missing AST edge from declared parent",
                            );
                        }
                    }
                }
            }
        }
        if options.includes("Method") && n.label == "METHOD" {
            let params: Vec<_> = graph
                .edges
                .iter()
                .filter(|e| e.source == n.id && e.label == "AST")
                .filter_map(|e| {
                    graph
                        .nodes
                        .iter()
                        .find(|p| p.id == e.target && p.label == "METHOD_PARAMETER_IN")
                })
                .collect();
            let variadic = params
                .iter()
                .filter(|p| {
                    p.properties.get("IS_VARIADIC")
                        == Some(&PropertyValue::Scalar(Scalar::Boolean(true)))
                })
                .count();
            if variadic > 1 {
                report(
                    out,
                    DiagnosticKind::SourceContract,
                    &path,
                    Some(&n.id.0),
                    "method has more than one variadic parameter",
                );
            }
            for input in params {
                let outputs: Vec<_> = graph
                    .edges
                    .iter()
                    .filter(|e| e.source == n.id && e.label == "AST")
                    .filter_map(|e| {
                        graph.nodes.iter().find(|p| {
                            p.id == e.target
                                && p.label == "METHOD_PARAMETER_OUT"
                                && integer(p, "INDEX") == integer(input, "INDEX")
                        })
                    })
                    .collect();
                if outputs.is_empty() {
                    report(
                        out,
                        DiagnosticKind::DerivedRelationship,
                        &path,
                        Some(&input.id.0),
                        "missing corresponding output parameter",
                    );
                }
                if options.includes("Shortcuts") {
                    for output in outputs {
                        if !has_edge(input, output, "PARAMETER_LINK") {
                            report(
                                out,
                                DiagnosticKind::DerivedRelationship,
                                &path,
                                Some(&input.id.0),
                                "missing PARAMETER_LINK",
                            );
                        }
                    }
                }
            }
        }
    }
}
/// Explicit default application returns a copy and preserves the original graph.
pub fn materialize_defaults(schema: &Schema, graph: &Graph) -> Result<Graph> {
    let mut result = graph.clone();
    for n in &mut result.nodes {
        let ty = schema
            .find_node(&n.label)
            .ok_or_else(|| Error::new(&n.id.0, "unknown node type"))?;
        for id in schema.resolved_properties(ty)? {
            let p = schema.property(id)?;
            if p.module == "ProtoSerialize" {
                continue;
            }
            if !n.properties.contains_key(&p.name) {
                match &p.cardinality {
                    Cardinality::One(Some(v)) => {
                        n.properties
                            .insert(p.name.clone(), PropertyValue::Scalar(v.clone()));
                    }
                    Cardinality::List => {
                        n.properties
                            .insert(p.name.clone(), PropertyValue::List(vec![]));
                    }
                    _ => {}
                }
            }
        }
        for c in &schema.node(ty)?.contained_nodes {
            if !n.contained.contains_key(&c.local_name) {
                match &c.cardinality {
                    Cardinality::One(None) => {
                        n.contained
                            .insert(c.local_name.clone(), ContainedValue::One(None));
                    }
                    Cardinality::List => {
                        n.contained
                            .insert(c.local_name.clone(), ContainedValue::List(vec![]));
                    }
                    _ => {}
                }
            }
        }
    }
    for e in &mut result.edges {
        if let Some(p) = schema
            .find_edge(&e.label)
            .and_then(|id| schema.edges[id.0].property)
        {
            let p = schema.property(p)?;
            if let Cardinality::One(Some(v)) = &p.cardinality {
                e.properties
                    .entry(p.name.clone())
                    .or_insert_with(|| PropertyValue::Scalar(v.clone()));
            }
        }
    }
    Ok(result)
}
