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
