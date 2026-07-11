use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::path::{Component, Path};

use gruffed_core::graph::{EdgeKind, Graph, NodeId, Value};
use gruffed_core::report::{Report, Severity};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceArtifact {
    pub schema_version: u32,
    pub projection: &'static str,
    pub root: String,
    pub stats: TraceStats,
    pub nodes: Vec<TraceNode>,
    pub edges: Vec<TraceEdge>,
    pub findings: Vec<TraceFinding>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub projected_nodes: usize,
    pub projected_edges: usize,
    pub omitted_clusters: usize,
    pub build_time_ms: u64,
    pub analyze_time_ms: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceNode {
    pub id: String,
    pub label: String,
    pub module_count: usize,
    pub finding_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub import_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceFinding {
    pub rule_id: String,
    pub severity: &'static str,
    pub message: String,
    pub file: String,
    pub path: Vec<String>,
}

#[derive(Default)]
struct ClusterStats {
    module_count: usize,
    finding_count: usize,
}

pub fn render_trace_json(
    report: &Report,
    graph: &Graph,
    root: &Path,
    max_nodes: usize,
) -> Result<String, serde_json::Error> {
    let max_nodes = max_nodes.max(1);
    let node_parts: HashMap<NodeId, Vec<String>> = graph
        .nodes()
        .map(|node| (node.id, path_parts(&node.label, root)))
        .collect();
    let frontier = adaptive_frontier(node_parts.values(), max_nodes);
    let mut clusters: HashMap<String, ClusterStats> = HashMap::new();
    let mut node_clusters: HashMap<NodeId, String> = HashMap::with_capacity(graph.node_count());

    for node in graph.nodes() {
        let cluster = cluster_for_parts(&node_parts[&node.id], &frontier);
        clusters.entry(cluster.clone()).or_default().module_count += 1;
        node_clusters.insert(node.id, cluster);
    }

    for finding in &report.findings {
        let mut touched = HashSet::new();
        for id in &finding.context {
            if let Some(cluster) = node_clusters.get(id) {
                touched.insert(cluster.clone());
            }
        }
        if touched.is_empty() {
            touched.insert(cluster_for_parts(
                &path_parts(&finding.location.file, root),
                &frontier,
            ));
        }
        for cluster in touched {
            clusters.entry(cluster).or_default().finding_count += 1;
        }
    }

    let mut selected: Vec<_> = clusters.iter().collect();
    selected.sort_by(|(label_a, a), (label_b, b)| {
        b.module_count
            .cmp(&a.module_count)
            .then_with(|| label_a.cmp(label_b))
    });
    selected.truncate(max_nodes);
    selected.sort_by_key(|(label, _)| *label);

    let selected_labels: HashSet<_> = selected.iter().map(|(label, _)| (*label).clone()).collect();
    let ids: HashMap<_, _> = selected
        .iter()
        .enumerate()
        .map(|(index, (label, _))| ((*label).clone(), format!("cluster:{index}")))
        .collect();

    let nodes = selected
        .into_iter()
        .map(|(label, stats)| TraceNode {
            id: ids[label].clone(),
            label: label.clone(),
            module_count: stats.module_count,
            finding_count: stats.finding_count,
        })
        .collect::<Vec<_>>();

    let mut edge_counts: HashMap<(String, String), usize> = HashMap::new();
    for edge in graph.edges() {
        let Some(source) = node_clusters.get(&edge.from) else {
            continue;
        };
        let Some(target) = node_clusters.get(&edge.to) else {
            continue;
        };
        if source != target && selected_labels.contains(source) && selected_labels.contains(target)
        {
            *edge_counts
                .entry((source.clone(), target.clone()))
                .or_default() += 1;
        }
    }

    let mut aggregate_edges: Vec<_> = edge_counts.into_iter().collect();
    aggregate_edges.sort_by(|((source_a, target_a), _), ((source_b, target_b), _)| {
        source_a.cmp(source_b).then_with(|| target_a.cmp(target_b))
    });
    let edges = aggregate_edges
        .into_iter()
        .enumerate()
        .map(|(index, ((source, target), import_count))| TraceEdge {
            id: format!("edge:{index}"),
            source: ids[&source].clone(),
            target: ids[&target].clone(),
            import_count,
        })
        .collect::<Vec<_>>();

    let findings = report
        .findings
        .iter()
        .map(|finding| TraceFinding {
            rule_id: finding.rule_id.clone(),
            severity: match finding.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
            },
            message: relative_message(&finding.message, root),
            file: relative_label(&finding.location.file, root),
            path: finding_path(graph, finding)
                .iter()
                .filter_map(|id| graph.node(*id))
                .map(|node| relative_label(&node.label, root))
                .collect(),
        })
        .collect();

    let artifact = TraceArtifact {
        schema_version: 1,
        projection: "directory-clusters",
        root: root
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string()),
        stats: TraceStats {
            total_nodes: graph.node_count(),
            total_edges: graph.edge_count(),
            projected_nodes: nodes.len(),
            projected_edges: edges.len(),
            omitted_clusters: clusters.len().saturating_sub(nodes.len()),
            build_time_ms: report.stats.build_time_ms,
            analyze_time_ms: report.stats.analyze_time_ms,
        },
        nodes,
        edges,
        findings,
    };

    serde_json::to_string_pretty(&artifact)
}

fn relative_label(label: &str, root: &Path) -> String {
    Path::new(label)
        .strip_prefix(root)
        .unwrap_or_else(|_| Path::new(label))
        .to_string_lossy()
        .replace('\\', "/")
}

fn relative_message(message: &str, root: &Path) -> String {
    let root = root.to_string_lossy();
    let root = root.trim_end_matches(['/', '\\']);
    message
        .replace(&format!("{root}/"), "")
        .replace(&format!("{root}\\"), "")
}

fn finding_path(graph: &Graph, finding: &gruffed_core::report::Finding) -> Vec<NodeId> {
    if finding.rule_id == "no-cycles" {
        ordered_cycle_path(graph, &finding.context).unwrap_or_else(|| finding.context.clone())
    } else {
        finding.context.clone()
    }
}

fn ordered_cycle_path(graph: &Graph, members: &[NodeId]) -> Option<Vec<NodeId>> {
    fn visit(
        graph: &Graph,
        node: NodeId,
        allowed: &HashSet<NodeId>,
        state: &mut HashMap<NodeId, u8>,
        parent: &mut HashMap<NodeId, NodeId>,
    ) -> Option<Vec<NodeId>> {
        state.insert(node, 1);
        for edge_id in graph.out_edges(node) {
            let Some(edge) = graph.edge(*edge_id) else {
                continue;
            };
            if edge.kind != EdgeKind::Imports
                || matches!(edge.properties.get("import_kind"), Some(Value::String(kind)) if kind == "type")
                || !allowed.contains(&edge.to)
            {
                continue;
            }
            match state.get(&edge.to).copied().unwrap_or(0) {
                0 => {
                    parent.insert(edge.to, node);
                    if let Some(path) = visit(graph, edge.to, allowed, state, parent) {
                        return Some(path);
                    }
                }
                1 => {
                    let mut path = vec![node];
                    let mut current = node;
                    while current != edge.to {
                        current = *parent.get(&current)?;
                        path.push(current);
                    }
                    path.reverse();
                    return Some(path);
                }
                _ => {}
            }
        }
        state.insert(node, 2);
        None
    }

    let allowed: HashSet<_> = members.iter().copied().collect();
    let mut state = HashMap::new();
    let mut parent = HashMap::new();
    for node in members {
        if state.get(node).copied().unwrap_or(0) == 0 {
            if let Some(path) = visit(graph, *node, &allowed, &mut state, &mut parent) {
                return Some(path);
            }
        }
    }
    None
}

fn path_parts(label: &str, root: &Path) -> Vec<String> {
    let relative = relative_label(label, root);
    let parts = Path::new(&relative)
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        vec!["(root)".to_string()]
    } else {
        parts
    }
}

#[derive(Default)]
struct PrefixTrie {
    member_count: usize,
    children: BTreeMap<String, PrefixTrie>,
}

impl PrefixTrie {
    fn insert(&mut self, parts: &[String]) {
        self.member_count += 1;
        if let Some((head, tail)) = parts.split_first() {
            self.children.entry(head.clone()).or_default().insert(tail);
        }
    }

    fn frontier_nodes(&self, prefix: &[String]) -> Vec<FrontierNode> {
        self.children
            .iter()
            .map(|(part, child)| {
                let mut child_prefix = prefix.to_vec();
                child_prefix.push(part.clone());
                FrontierNode {
                    prefix: child_prefix.clone(),
                    member_count: child.member_count,
                    children: child.frontier_nodes(&child_prefix),
                }
            })
            .collect()
    }
}

#[derive(Clone)]
struct FrontierNode {
    prefix: Vec<String>,
    member_count: usize,
    children: Vec<FrontierNode>,
}

#[derive(Clone)]
struct HeapEntry(FrontierNode);

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.member_count == other.0.member_count && self.0.prefix == other.0.prefix
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .member_count
            .cmp(&other.0.member_count)
            .then_with(|| other.0.prefix.cmp(&self.0.prefix))
    }
}

fn adaptive_frontier<'a>(
    paths: impl Iterator<Item = &'a Vec<String>>,
    max_nodes: usize,
) -> Vec<Vec<String>> {
    let mut trie = PrefixTrie::default();
    for path in paths {
        trie.insert(path);
    }

    let mut roots = trie.frontier_nodes(&[]);
    roots.truncate(max_nodes);
    let mut active: BTreeMap<_, _> = roots
        .iter()
        .cloned()
        .map(|node| (node.prefix.clone(), node))
        .collect();
    let mut candidates: BinaryHeap<_> = roots
        .into_iter()
        .filter(|node| !node.children.is_empty())
        .map(HeapEntry)
        .collect();

    while let Some(HeapEntry(candidate)) = candidates.pop() {
        if !active.contains_key(&candidate.prefix) || candidate.children.is_empty() {
            continue;
        }
        if active.len() + candidate.children.len() - 1 > max_nodes {
            continue;
        }
        active.remove(&candidate.prefix);
        for child in candidate.children {
            if !child.children.is_empty() {
                candidates.push(HeapEntry(child.clone()));
            }
            active.insert(child.prefix.clone(), child);
        }
    }

    active.into_keys().collect()
}

fn cluster_for_parts(parts: &[String], frontier: &[Vec<String>]) -> String {
    let best = frontier
        .iter()
        .filter(|prefix| parts.starts_with(prefix))
        .max_by_key(|prefix| prefix.len());

    best.map(|prefix| prefix.join("/"))
        .unwrap_or_else(|| "(other)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gruffed_core::graph::NodeKind;
    use std::collections::HashMap;

    #[test]
    fn sanitizes_windows_root_in_messages() {
        let message = r#"Cannot resolve import \"./missing\" in C:\Users\drew\repo\src\index.ts:1"#;
        let sanitized = relative_message(message, Path::new(r"C:\Users\drew\repo"));
        assert_eq!(
            sanitized,
            r#"Cannot resolve import \"./missing\" in src\index.ts:1"#
        );
    }

    #[test]
    fn extracts_an_edge_ordered_cycle_from_scc_members() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeKind::Module, "a.ts", HashMap::new());
        let b = graph.add_node(NodeKind::Module, "b.ts", HashMap::new());
        let c = graph.add_node(NodeKind::Module, "c.ts", HashMap::new());
        let branch = graph.add_node(NodeKind::Module, "branch.ts", HashMap::new());
        graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, branch, EdgeKind::Imports, HashMap::new());
        graph.add_edge(branch, b, EdgeKind::Imports, HashMap::new());
        graph.add_edge(b, c, EdgeKind::Imports, HashMap::new());
        graph.add_edge(c, a, EdgeKind::Imports, HashMap::new());

        let path = ordered_cycle_path(&graph, &[c, branch, a, b]).unwrap();
        assert!(!path.is_empty());
        for (index, node) in path.iter().enumerate() {
            let next = path[(index + 1) % path.len()];
            assert!(graph.out_edges(*node).iter().any(|edge_id| {
                graph
                    .edge(*edge_id)
                    .is_some_and(|edge| edge.kind == EdgeKind::Imports && edge.to == next)
            }));
        }
    }
}
