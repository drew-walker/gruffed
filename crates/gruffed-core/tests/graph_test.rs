use std::collections::HashMap;

use gruffed_core::graph::{EdgeKind, Graph, NodeKind};

#[test]
fn add_node_returns_unique_ids() {
    let mut graph = Graph::new();
    let id1 = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
    let id2 = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
    assert_ne!(id1, id2);
}

#[test]
fn add_edge_connects_nodes() {
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
    let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
    let edge_id = graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

    assert_eq!(graph.out_edges(a), &[edge_id]);
    assert_eq!(graph.in_edges(b), &[edge_id]);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn bidirectional_adjacency_works() {
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
    let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
    let _ = graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

    assert_eq!(graph.out_edges(a).len(), 1);
    assert_eq!(graph.in_edges(a).len(), 0);
    assert_eq!(graph.out_edges(b).len(), 0);
    assert_eq!(graph.in_edges(b).len(), 1);
}

#[test]
fn find_node_by_label_returns_correct_id() {
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "src/a.ts", HashMap::new());
    let _ = graph.add_node(NodeKind::Module, "src/b.ts", HashMap::new());

    assert_eq!(graph.find_node_by_label("src/a.ts"), Some(a));
    assert_eq!(graph.find_node_by_label("nonexistent.ts"), None);
}

#[test]
fn node_count_and_edge_count_are_accurate() {
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
    let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
    let c = graph.add_node(NodeKind::Module, "c.ts", HashMap::new());
    let _ = graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
    let _ = graph.add_edge(b, c, EdgeKind::Imports, HashMap::new());

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);
}

#[test]
fn node_properties_are_shared_via_arc() {
    use gruffed_core::graph::Value;

    let mut props = HashMap::new();
    props.insert("source_type".to_string(), Value::String("ts".to_string()));
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "a.ts", props.clone());
    let node = graph.node(a).unwrap();
    assert_eq!(
        node.properties.get("source_type"),
        Some(&Value::String("ts".to_string()))
    );
}
