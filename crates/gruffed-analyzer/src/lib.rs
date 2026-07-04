use std::collections::HashMap;

use gruffed_builder::BuildWarning;
use gruffed_core::graph::Graph;
use gruffed_core::report::{Finding, Report};
use rayon::prelude::*;

pub mod rules;
pub mod scc;

use gruffed_config::RuleSetting;

/// A rule that analyzes a graph for problems.
pub trait Rule: Send + Sync {
    /// The rule identifier, e.g. "no-cycles".
    fn id(&self) -> &'static str;

    /// Analyze the graph and return findings.
    fn analyze(&self, graph: &Graph, warnings: &[BuildWarning]) -> Vec<Finding>;
}

pub struct Analyzer {
    rules: Vec<Box<dyn Rule>>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn from_config(rules_config: &HashMap<String, RuleSetting>) -> Self {
        let mut rules: Vec<Box<dyn Rule>> = Vec::new();

        for (rule_id, setting) in rules_config {
            if !setting.is_enabled() {
                continue;
            }
            let severity = setting
                .severity()
                .unwrap_or(gruffed_core::report::Severity::Error);

            match rule_id.as_str() {
                "no-cycles" => {
                    rules.push(Box::new(rules::no_cycles::NoCyclesRule::new(severity)));
                }
                "no-unresolved" => {
                    rules.push(Box::new(rules::no_unresolved::NoUnresolvedRule::new(
                        severity,
                    )));
                }
                "no-long-chains" => {
                    let max = setting
                        .options()
                        .and_then(|o| o.get("max"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(10) as usize;
                    rules.push(Box::new(rules::no_long_chains::NoLongChainsRule::new(
                        severity, max,
                    )));
                }
                _ => {}
            }
        }

        Self { rules }
    }

    pub fn run(&self, graph: &Graph, warnings: &[BuildWarning]) -> Report {
        let findings: Vec<Finding> = self
            .rules
            .par_iter()
            .flat_map(|rule| rule.analyze(graph, warnings))
            .collect();

        Report {
            findings,
            stats: gruffed_core::report::GraphStats {
                node_count: graph.node_count(),
                edge_count: graph.edge_count(),
                build_time_ms: 0,
                analyze_time_ms: 0,
            },
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
