use std::collections::{HashMap, HashSet, VecDeque};

use gruffed_builder::BuildWarning;
use gruffed_core::graph::{EdgeKind, Graph, NodeId, Value};
use gruffed_core::report::{Finding, Location, Severity};

use crate::Rule;

pub struct NoLongChainsRule {
    severity: Severity,
    max_depth: usize,
}

impl NoLongChainsRule {
    pub fn new(severity: Severity, max_depth: usize) -> Self {
        Self { severity, max_depth }
    }
}

impl Rule for NoLongChainsRule {
    fn id(&self) -> &'static str {
        "no-long-chains"
    }

    fn analyze(&self, graph: &Graph, _warnings: &[BuildWarning]) -> Vec<Finding> {
        let entrypoints: Vec<NodeId> = graph
            .nodes()
            .filter(|n| {
                n.properties
                    .get("is_entrypoint")
                    .and_then(|v| match v {
                        Value::Bool(b) => Some(*b),
                        _ => None,
                    })
                    .unwrap_or(false)
            })
            .map(|n| n.id)
            .collect();

        if entrypoints.is_empty() {
            // No entrypoints — can't compute depth
            return vec![];
        }

        let depths = bfs_depths(graph, &entrypoints);

        let mut findings = Vec::new();

        for (node_id, depth) in &depths {
            if *depth > self.max_depth {
                let node = graph.node(*node_id);
                let path = reconstruct_path(graph, *node_id, &depths);

                let labels: Vec<String> = path
                    .iter()
                    .filter_map(|id| graph.node(*id))
                    .map(|n| n.label.clone())
                    .collect();

                let location = node
                    .map(|n| Location {
                        file: n.label.clone(),
                        line: None,
                        column: None,
                    })
                    .unwrap_or(Location {
                        file: "unknown".to_string(),
                        line: None,
                        column: None,
                    });

                let truncated = if labels.len() > 5 {
                    format!(
                        "{} → ... ({} more)",
                        labels[..3].join(" → "),
                        labels.len() - 3
                    )
                } else {
                    labels.join(" → ")
                };

                findings.push(Finding {
                    rule_id: self.id().to_string(),
                    severity: self.severity.clone(),
                    message: format!(
                        "Import chain exceeds max depth ({} > {}): {}",
                        depth, self.max_depth, truncated
                    ),
                    location,
                    context: path,
                });
            }
        }

        findings
    }
}

/// BFS from entrypoints, computing the depth (longest path) to each node.
fn bfs_depths(graph: &Graph, entrypoints: &[NodeId]) -> HashMap<NodeId, usize> {
    let mut depths: HashMap<NodeId, usize> = HashMap::new();
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();

    for &entry in entrypoints {
        depths.insert(entry, 0);
        visited.insert(entry);
        queue.push_back(entry);
    }

    while let Some(current) = queue.pop_front() {
        let current_depth = *depths.get(&current).unwrap_or(&0);

        for edge_id in graph.out_edges(current) {
            if let Some(edge) = graph.edge(*edge_id) {
                if edge.kind != EdgeKind::Imports {
                    continue;
                }
                let neighbor = edge.to;
                let new_depth = current_depth + 1;

                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    depths.insert(neighbor, new_depth);
                    queue.push_back(neighbor);
                } else {
                    // Update if we found a longer path
                    let existing = *depths.get(&neighbor).unwrap_or(&0);
                    if new_depth > existing {
                        depths.insert(neighbor, new_depth);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    depths
}

/// Reconstruct the path from an entrypoint to the given node.
fn reconstruct_path(
    graph: &Graph,
    target: NodeId,
    depths: &HashMap<NodeId, usize>,
) -> Vec<NodeId> {
    let target_depth = *depths.get(&target).unwrap_or(&0);
    let mut path = vec![target];
    let mut current = target;

    for depth in (0..target_depth).rev() {
        // Find a predecessor with depth == current_depth - 1 that has an edge to current
        let mut found = false;
        for node in graph.nodes() {
            if depths.get(&node.id) != Some(&depth) {
                continue;
            }
            for edge_id in graph.out_edges(node.id) {
                if let Some(edge) = graph.edge(*edge_id) {
                    if edge.to == current && edge.kind == EdgeKind::Imports {
                        path.insert(0, node.id);
                        current = node.id;
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }
        if !found {
            break;
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use gruffed_core::graph::{EdgeKind, NodeKind};

    fn make_entrypoint_node(graph: &mut Graph, label: &str) -> NodeId {
        let mut props = HashMap::new();
        props.insert("is_entrypoint".to_string(), Value::Bool(true));
        graph.add_node(NodeKind::Module, label, props)
    }

    #[test]
    fn no_findings_for_short_chain() {
        let mut graph = Graph::new();
        let a = make_entrypoint_node(&mut graph, "a.ts");
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

        let rule = NoLongChainsRule::new(Severity::Warning, 3);
        let findings = rule.analyze(&graph, &[]);
        assert!(findings.is_empty());
    }

    #[test]
    fn detects_chain_exceeding_max() {
        let mut graph = Graph::new();
        let a = make_entrypoint_node(&mut graph, "a.ts");
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        let c = graph.add_node(NodeKind::Module, "c.ts", HashMap::new());
        let d = graph.add_node(NodeKind::Module, "d.ts", HashMap::new());
        let e = graph.add_node(NodeKind::Module, "e.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, c, EdgeKind::Imports, HashMap::new());
        graph.add_edge(c, d, EdgeKind::Imports, HashMap::new());
        graph.add_edge(d, e, EdgeKind::Imports, HashMap::new());

        let rule = NoLongChainsRule::new(Severity::Warning, 3);
        let findings = rule.analyze(&graph, &[]);
        assert!(!findings.is_empty());
        assert!(findings[0].message.contains("4 > 3"));
    }

    #[test]
    fn no_entrypoints_produces_no_findings() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

        let rule = NoLongChainsRule::new(Severity::Warning, 1);
        let findings = rule.analyze(&graph, &[]);
        assert!(findings.is_empty());
    }
}
