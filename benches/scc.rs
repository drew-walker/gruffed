use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use gruffed_analyzer::scc::find_sccs;
use gruffed_core::graph::{EdgeKind, Graph, NodeKind};

fn build_large_acyclic_graph(size: usize) -> Graph {
    let mut graph = Graph::new();
    let mut nodes = Vec::new();
    for i in 0..size {
        let node = graph.add_node(NodeKind::Module, format!("node_{}", i), HashMap::new());
        nodes.push(node);
    }
    for i in 0..(size - 1) {
        graph.add_edge(nodes[i], nodes[i + 1], EdgeKind::Imports, HashMap::new());
    }
    graph
}

fn bench_scc_acyclic(c: &mut Criterion) {
    let graph = build_large_acyclic_graph(1000);
    c.bench_function("scc_acyclic_1000", |b| {
        b.iter(|| black_box(find_sccs(black_box(&graph))))
    });
}

fn bench_scc_with_cycles(c: &mut Criterion) {
    let mut graph = build_large_acyclic_graph(1000);
    // Add a cycle back to the start
    let nodes: Vec<_> = graph.nodes().map(|n| n.id).collect();
    if nodes.len() >= 2 {
        graph.add_edge(nodes[1], nodes[0], EdgeKind::Imports, HashMap::new());
    }
    c.bench_function("scc_with_cycle_1000", |b| {
        b.iter(|| black_box(find_sccs(black_box(&graph))))
    });
}

criterion_group!(benches, bench_scc_acyclic, bench_scc_with_cycles);
criterion_main!(benches);
