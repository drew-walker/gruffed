use gruffed_core::report::{Finding, GraphStats, Location, Report, Severity};

#[test]
fn empty_report_has_no_errors_or_warnings() {
    let report = Report::new();
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warning_count(), 0);
    assert!(!report.has_errors());
}

#[test]
fn error_count_counts_only_errors() {
    let report = Report {
        findings: vec![
            test_finding("rule-a", Severity::Error),
            test_finding("rule-a", Severity::Error),
            test_finding("rule-b", Severity::Warning),
        ],
        stats: GraphStats::default(),
    };
    assert_eq!(report.error_count(), 2);
    assert_eq!(report.warning_count(), 1);
    assert!(report.has_errors());
}

#[test]
fn report_serializes_to_json() {
    let report = Report {
        findings: vec![test_finding("no-cycles", Severity::Error)],
        stats: GraphStats {
            node_count: 10,
            edge_count: 5,
            build_time_ms: 100,
            analyze_time_ms: 20,
        },
    };
    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("no-cycles"));
    assert!(json.contains("\"node_count\":10"));
}

fn test_finding(rule_id: &str, severity: Severity) -> Finding {
    Finding {
        rule_id: rule_id.to_string(),
        severity,
        message: "test".to_string(),
        location: Location {
            file: "test.ts".to_string(),
            line: None,
            column: None,
        },
        context: vec![],
    }
}
