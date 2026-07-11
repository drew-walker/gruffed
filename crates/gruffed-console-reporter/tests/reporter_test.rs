use std::collections::HashMap;

use gruffed_console_reporter::ConsoleReporter;
use gruffed_core::graph::{EdgeKind, Graph, NodeKind};
use gruffed_core::report::{Finding, GraphStats, Location, Report, Severity};

fn make_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.add_node(NodeKind::Module, "src/a.ts", HashMap::new());
    let b = graph.add_node(NodeKind::Module, "src/b.ts", HashMap::new());
    graph.add_edge(a, b, EdgeKind::Imports, HashMap::new());
    graph
}

#[test]
fn renders_clean_report() {
    let graph = make_graph();
    let report = Report {
        findings: vec![],
        stats: GraphStats {
            node_count: 2,
            edge_count: 1,
            build_time_ms: 10,
            analyze_time_ms: 5,
        },
    };

    let reporter = ConsoleReporter::new(false);
    let output = reporter.render(&report, &graph);
    assert!(output.contains("No problems found"));
    assert!(output.contains("Nodes:"));
}

#[test]
fn renders_error_finding() {
    let graph = make_graph();
    let report = Report {
        findings: vec![Finding {
            rule_id: "no-cycles".to_string(),
            severity: Severity::Error,
            message: "Circular dependency (2 nodes)".to_string(),
            location: Location {
                file: "src/a.ts".to_string(),
                line: None,
                column: None,
            },
            context: vec![],
        }],
        stats: GraphStats {
            node_count: 2,
            edge_count: 1,
            build_time_ms: 10,
            analyze_time_ms: 5,
        },
    };

    let reporter = ConsoleReporter::new(false);
    let output = reporter.render(&report, &graph);
    assert!(output.contains("error"));
    assert!(output.contains("no-cycles"));
    assert!(output.contains("Circular dependency"));
}

#[test]
fn renders_warning_finding() {
    let graph = make_graph();
    let report = Report {
        findings: vec![Finding {
            rule_id: "no-long-chains".to_string(),
            severity: Severity::Warning,
            message: "Import chain exceeds max depth (12 > 10)".to_string(),
            location: Location {
                file: "src/index.ts".to_string(),
                line: None,
                column: None,
            },
            context: vec![],
        }],
        stats: GraphStats {
            node_count: 2,
            edge_count: 1,
            build_time_ms: 10,
            analyze_time_ms: 5,
        },
    };

    let reporter = ConsoleReporter::new(false);
    let output = reporter.render(&report, &graph);
    assert!(output.contains("warning"));
    assert!(output.contains("no-long-chains"));
}

#[test]
fn json_output_is_valid_json() {
    let report = Report {
        findings: vec![],
        stats: GraphStats {
            node_count: 1,
            edge_count: 0,
            build_time_ms: 5,
            analyze_time_ms: 1,
        },
    };

    let json = gruffed_console_reporter::render_json(&report).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_object());
}

#[test]
fn trace_output_is_bounded_and_has_valid_edges() {
    let mut graph = Graph::new();
    let compiler = graph.add_node(
        NodeKind::Module,
        "/repo/src/compiler/checker.ts",
        HashMap::new(),
    );
    let parser = graph.add_node(
        NodeKind::Module,
        "/repo/src/parser/index.ts",
        HashMap::new(),
    );
    let cli = graph.add_node(
        NodeKind::Module,
        "/repo/packages/cli/index.ts",
        HashMap::new(),
    );
    graph.add_edge(compiler, parser, EdgeKind::Imports, HashMap::new());
    graph.add_edge(cli, compiler, EdgeKind::Imports, HashMap::new());

    let report = Report {
        findings: vec![Finding {
            rule_id: "no-cycles".to_string(),
            severity: Severity::Error,
            message: "cycle".to_string(),
            location: Location {
                file: "/repo/src/compiler/checker.ts".to_string(),
                line: None,
                column: None,
            },
            context: vec![compiler, parser],
        }],
        stats: GraphStats {
            node_count: 3,
            edge_count: 2,
            build_time_ms: 4,
            analyze_time_ms: 1,
        },
    };

    let json = gruffed_console_reporter::render_trace_json(
        &report,
        &graph,
        std::path::Path::new("/repo"),
        2,
    )
    .unwrap();
    let trace: serde_json::Value = serde_json::from_str(&json).unwrap();
    let nodes = trace["nodes"].as_array().unwrap();
    let edges = trace["edges"].as_array().unwrap();
    let ids: std::collections::HashSet<_> = nodes
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect();

    assert_eq!(nodes.len(), 2);
    assert_eq!(trace["stats"]["omittedClusters"], 0);
    assert!(edges.iter().all(|edge| {
        ids.contains(edge["source"].as_str().unwrap())
            && ids.contains(edge["target"].as_str().unwrap())
    }));
    assert_eq!(trace["findings"][0]["file"], "src/compiler/checker.ts");
    assert!(!trace["findings"][0]["message"]
        .as_str()
        .unwrap()
        .contains("/repo"));
}
