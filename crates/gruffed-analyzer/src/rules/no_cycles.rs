use gruffed_builder::BuildWarning;
use gruffed_core::graph::{Edge, EdgeKind, Graph, Value};
use gruffed_core::report::{Finding, Location, Severity};

use crate::scc::find_sccs_with_edge_filter;
use crate::Rule;

pub struct NoCyclesRule {
    severity: Severity,
}

impl NoCyclesRule {
    pub fn new(severity: Severity) -> Self {
        Self { severity }
    }
}

impl Rule for NoCyclesRule {
    fn id(&self) -> &'static str {
        "no-cycles"
    }

    fn analyze(&self, graph: &Graph, _warnings: &[BuildWarning]) -> Vec<Finding> {
        let sccs = find_sccs_with_edge_filter(graph, includes_runtime_cycle_edge);

        sccs.into_iter()
            .filter(|scc| scc.is_cycle())
            .map(|scc| {
                let node_labels: Vec<String> = scc
                    .nodes
                    .iter()
                    .filter_map(|id| graph.node(*id))
                    .map(|n| n.label.clone())
                    .collect();

                let first_node = scc.nodes.first().copied();
                let location = first_node
                    .and_then(|id| graph.node(id))
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

                Finding {
                    rule_id: self.id().to_string(),
                    severity: self.severity.clone(),
                    message: format!(
                        "Circular dependency detected ({} nodes in cycle): {}",
                        scc.nodes.len(),
                        node_labels.join(" → ")
                    ),
                    location,
                    context: scc.nodes,
                }
            })
            .collect()
    }
}

fn includes_runtime_cycle_edge(edge: &Edge) -> bool {
    if edge.kind != EdgeKind::Imports {
        return false;
    }

    !matches!(
        edge.properties.get("import_kind"),
        Some(Value::String(kind)) if kind == "type"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use gruffed_core::graph::NodeKind;
    use std::collections::HashMap;

    #[test]
    fn detects_simple_cycle() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, a, EdgeKind::Imports, HashMap::new());

        let rule = NoCyclesRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &[]);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "no-cycles");
        assert!(findings[0].message.contains("Circular dependency"));
    }

    #[test]
    fn no_findings_for_acyclic_graph() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());

        let rule = NoCyclesRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &[]);
        assert!(findings.is_empty());
    }

    #[test]
    fn detects_three_node_cycle() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        let c = graph.add_node(NodeKind::Module, "c.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, c, EdgeKind::Imports, HashMap::new());
        graph.add_edge(c, a, EdgeKind::Imports, HashMap::new());

        let rule = NoCyclesRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &[]);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("3 nodes"));
    }

    #[test]
    fn ignores_type_only_cycles_by_default() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        let mut type_props = HashMap::new();
        type_props.insert(
            "import_kind".to_string(),
            gruffed_core::graph::Value::String("type".to_string()),
        );

        graph.add_edge(a, b, EdgeKind::Imports, type_props.clone());
        graph.add_edge(b, a, EdgeKind::Imports, type_props);

        let rule = NoCyclesRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &[]);
        assert!(findings.is_empty());
    }
}
