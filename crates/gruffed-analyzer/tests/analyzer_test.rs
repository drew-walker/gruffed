use std::collections::HashMap;
use std::path::PathBuf;

use gruffed_analyzer::Analyzer;
use gruffed_builder::ModuleGraphBuilder;
use gruffed_config::{GruffedConfig, RuleSetting};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("fixtures")
        .join(name)
        .join("src")
}

fn load_fixture_config(name: &str) -> HashMap<String, RuleSetting> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("fixtures")
        .join(name)
        .join("gruffed.jsonc");
    let config = GruffedConfig::from_file(&config_path).unwrap();
    config.rules
}

#[test]
fn detects_cycle_in_cycle_fixture() {
    let rules = load_fixture_config("cycle");
    let build_result = ModuleGraphBuilder::new(fixture_path("cycle"))
        .with_entrypoints(vec!["src/a.ts".to_string()])
        .build()
        .unwrap();

    let analyzer = Analyzer::from_config(&rules);
    let report = analyzer.run(&build_result.graph, &build_result.warnings);

    let cycle_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == "no-cycles")
        .collect();
    assert!(!cycle_findings.is_empty());
    assert!(cycle_findings[0].message.contains("Circular dependency"));
}

#[test]
fn detects_unresolved_in_unresolved_fixture() {
    let rules = load_fixture_config("unresolved");
    let build_result = ModuleGraphBuilder::new(fixture_path("unresolved"))
        .build()
        .unwrap();

    let analyzer = Analyzer::from_config(&rules);
    let report = analyzer.run(&build_result.graph, &build_result.warnings);

    let unresolved_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == "no-unresolved")
        .collect();
    assert_eq!(unresolved_findings.len(), 1);
    assert!(unresolved_findings[0].message.contains("Cannot resolve"));
}

#[test]
fn detects_long_chain_in_long_chain_fixture() {
    let rules = load_fixture_config("long-chain");
    let build_result = ModuleGraphBuilder::new(fixture_path("long-chain"))
        .with_entrypoints(vec!["src/index.ts".to_string()])
        .build()
        .unwrap();

    let analyzer = Analyzer::from_config(&rules);
    let report = analyzer.run(&build_result.graph, &build_result.warnings);

    let chain_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == "no-long-chains")
        .collect();
    assert!(!chain_findings.is_empty());
    assert!(chain_findings[0].message.contains("exceeds max depth"));
}

#[test]
fn simple_fixture_has_no_findings() {
    let rules = load_fixture_config("simple");
    let build_result = ModuleGraphBuilder::new(fixture_path("simple"))
        .with_entrypoints(vec!["src/index.ts".to_string()])
        .build()
        .unwrap();

    let analyzer = Analyzer::from_config(&rules);
    let report = analyzer.run(&build_result.graph, &build_result.warnings);

    assert!(report.findings.is_empty());
}

#[test]
fn report_stats_contain_correct_counts() {
    let rules = load_fixture_config("simple");
    let build_result = ModuleGraphBuilder::new(fixture_path("simple"))
        .with_entrypoints(vec!["src/index.ts".to_string()])
        .build()
        .unwrap();

    let analyzer = Analyzer::from_config(&rules);
    let report = analyzer.run(&build_result.graph, &build_result.warnings);

    assert_eq!(report.stats.node_count, 3);
    assert_eq!(report.stats.edge_count, 3);
}
