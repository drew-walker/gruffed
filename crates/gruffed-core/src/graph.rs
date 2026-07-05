use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub(crate) u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub(crate) u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NodeKind {
    Module,
    Package,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EdgeKind {
    Imports,
    DependsOn,
    References,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub label: String,
    pub properties: Arc<HashMap<String, Value>>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub kind: EdgeKind,
    pub properties: Arc<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
    out_edges: HashMap<NodeId, SmallVec<[EdgeId; 8]>>,
    in_edges: HashMap<NodeId, SmallVec<[EdgeId; 8]>>,
    next_node_id: u64,
    next_edge_id: u64,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(
        &mut self,
        kind: NodeKind,
        label: impl Into<String>,
        properties: HashMap<String, Value>,
    ) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        let node = Node {
            id,
            kind,
            label: label.into(),
            properties: Arc::new(properties),
        };
        self.nodes.insert(id, node);
        self.out_edges.insert(id, SmallVec::new());
        self.in_edges.insert(id, SmallVec::new());
        id
    }

    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: EdgeKind,
        properties: HashMap<String, Value>,
    ) -> EdgeId {
        let id = EdgeId(self.next_edge_id);
        self.next_edge_id += 1;
        let edge = Edge {
            id,
            from,
            to,
            kind: kind.clone(),
            properties: Arc::new(properties),
        };
        self.edges.insert(id, edge);
        self.out_edges
            .get_mut(&from)
            .expect("from node exists")
            .push(id);
        self.in_edges.get_mut(&to).expect("to node exists").push(id);
        id
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn out_edges(&self, id: NodeId) -> &[EdgeId] {
        self.out_edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn in_edges(&self, id: NodeId) -> &[EdgeId] {
        self.in_edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn find_node_by_label(&self, label: &str) -> Option<NodeId> {
        self.nodes
            .iter()
            .find(|(_, n)| n.label == label)
            .map(|(id, _)| *id)
    }
}
