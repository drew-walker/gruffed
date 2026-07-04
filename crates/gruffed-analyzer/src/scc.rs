use std::collections::HashMap;

use gruffed_core::graph::{Graph, NodeId};

/// A strongly connected component found by Tarjan's algorithm.
#[derive(Debug, Clone)]
pub struct Scc {
    pub nodes: Vec<NodeId>,
}

impl Scc {
    pub fn is_cycle(&self) -> bool {
        self.nodes.len() > 1
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

/// Find all strongly connected components in the graph using Tarjan's algorithm.
/// Runs in O(V + E).
pub fn find_sccs(graph: &Graph) -> Vec<Scc> {
    let node_count = graph.node_count();
    let mut state = TarjanState::new(node_count);

    for node_id in graph.nodes().map(|n| n.id) {
        if !state.index.contains_key(&node_id) {
            strongconnect(graph, node_id, &mut state);
        }
    }

    state.sccs
}

struct TarjanState {
    index: HashMap<NodeId, usize>,
    lowlink: HashMap<NodeId, usize>,
    on_stack: HashMap<NodeId, bool>,
    stack: Vec<NodeId>,
    next_index: usize,
    sccs: Vec<Scc>,
}

impl TarjanState {
    fn new(_node_count: usize) -> Self {
        Self {
            index: HashMap::new(),
            lowlink: HashMap::new(),
            on_stack: HashMap::new(),
            stack: Vec::new(),
            next_index: 0,
            sccs: Vec::new(),
        }
    }
}

fn strongconnect(graph: &Graph, v: NodeId, state: &mut TarjanState) {
    let v_index = state.next_index;
    state.index.insert(v, v_index);
    state.lowlink.insert(v, v_index);
    state.next_index += 1;
    state.stack.push(v);
    state.on_stack.insert(v, true);

    for edge_id in graph.out_edges(v) {
        if let Some(edge) = graph.edge(*edge_id) {
            let w = edge.to;
            if !state.index.contains_key(&w) {
                strongconnect(graph, w, state);
                let w_low = *state.lowlink.get(&w).unwrap();
                let v_low = *state.lowlink.get(&v).unwrap();
                state.lowlink.insert(v, v_low.min(w_low));
            } else if *state.on_stack.get(&w).unwrap_or(&false) {
                let w_index = *state.index.get(&w).unwrap();
                let v_low = *state.lowlink.get(&v).unwrap();
                state.lowlink.insert(v, v_low.min(w_index));
            }
        }
    }

    if state.lowlink.get(&v) == state.index.get(&v) {
        let mut component = Vec::new();
        loop {
            let w = state.stack.pop().unwrap();
            state.on_stack.insert(w, false);
            component.push(w);
            if w == v {
                break;
            }
        }
        state.sccs.push(Scc { nodes: component });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use gruffed_core::graph::{EdgeKind, Graph, NodeKind};

    #[test]
    fn acyclic_graph_has_no_cycles() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

        let sccs = find_sccs(&graph);
        assert_eq!(sccs.len(), 2);
        assert!(sccs.iter().all(|s| !s.is_cycle()));
    }

    #[test]
    fn simple_cycle_detected() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, a, EdgeKind::Imports, HashMap::new());

        let sccs = find_sccs(&graph);
        let cycles: Vec<_> = sccs.iter().filter(|s| s.is_cycle()).collect();
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].size(), 2);
    }

    #[test]
    fn three_node_cycle_detected() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b", HashMap::new());
        let c = graph.add_node(NodeKind::Module, "c", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, c, EdgeKind::Imports, HashMap::new());
        graph.add_edge(c, a, EdgeKind::Imports, HashMap::new());

        let sccs = find_sccs(&graph);
        let cycles: Vec<_> = sccs.iter().filter(|s| s.is_cycle()).collect();
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].size(), 3);
    }

    #[test]
    fn disconnected_components() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b", HashMap::new());
        let c = graph.add_node(NodeKind::Module, "c", HashMap::new());
        let d = graph.add_node(NodeKind::Module, "d", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(c, d, EdgeKind::Imports, HashMap::new());

        let sccs = find_sccs(&graph);
        assert_eq!(sccs.len(), 4);
    }

    #[test]
    fn self_loop_is_cycle() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a", HashMap::new());
        graph.add_edge(a, a, EdgeKind::Imports, HashMap::new());

        let sccs = find_sccs(&graph);
        assert_eq!(sccs.len(), 1);
        assert_eq!(sccs[0].size(), 1);
    }
}
