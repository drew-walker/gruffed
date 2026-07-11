use gruffed_core::graph::Graph;
use gruffed_core::report::{Report, Severity};

mod trace;

pub use trace::{render_trace_json, TraceArtifact};

pub struct ConsoleReporter {
    pub use_color: bool,
    pub max_context_nodes: usize,
}

impl Default for ConsoleReporter {
    fn default() -> Self {
        Self {
            use_color: true,
            max_context_nodes: 5,
        }
    }
}

impl ConsoleReporter {
    pub fn new(use_color: bool) -> Self {
        Self {
            use_color,
            max_context_nodes: 5,
        }
    }

    pub fn render(&self, report: &Report, graph: &Graph) -> String {
        let mut output = String::new();

        output.push_str("gruffed v0.1.0 — module graph analysis\n\n");

        output.push_str(&format!(
            "  Nodes: {:>6}    Edges: {:>6}    Build: {}ms    Analyze: {}ms\n\n",
            report.stats.node_count,
            report.stats.edge_count,
            report.stats.build_time_ms,
            report.stats.analyze_time_ms,
        ));

        if report.findings.is_empty() {
            output.push_str("  No problems found.\n");
            return output;
        }

        let errors = report.error_count();
        let warnings = report.warning_count();
        let summary = if errors > 0 && warnings > 0 {
            format!(
                "  {} {}, {} {}\n\n",
                red("✖", self.use_color),
                pluralize(errors, "error"),
                yellow("⚠", self.use_color),
                pluralize(warnings, "warning")
            )
        } else if errors > 0 {
            format!(
                "  {} {}\n\n",
                red("✖", self.use_color),
                pluralize(errors, "error")
            )
        } else {
            format!(
                "  {} {}\n\n",
                yellow("⚠", self.use_color),
                pluralize(warnings, "warning")
            )
        };
        output.push_str(&summary);

        let mut rule_ids: Vec<&String> = report.findings.iter().map(|f| &f.rule_id).collect();
        rule_ids.sort();
        rule_ids.dedup();

        for rule_id in rule_ids {
            for finding in report.findings.iter().filter(|f| &f.rule_id == rule_id) {
                let severity_str = match finding.severity {
                    Severity::Error => {
                        if self.use_color {
                            "\x1b[31m  error\x1b[0m"
                        } else {
                            "  error"
                        }
                    }
                    Severity::Warning => {
                        if self.use_color {
                            "\x1b[33m  warning\x1b[0m"
                        } else {
                            "  warning"
                        }
                    }
                };

                output.push_str(&format!("{}  {}\n", severity_str, finding.rule_id));

                let message = self.format_finding_message(finding, graph);
                for line in message.lines() {
                    output.push_str(&format!("         {}\n", line));
                }
                output.push('\n');
            }
        }

        output
    }

    fn format_finding_message(
        &self,
        finding: &gruffed_core::report::Finding,
        graph: &Graph,
    ) -> String {
        let mut msg = finding.message.clone();

        if !finding.context.is_empty() {
            let labels: Vec<String> = finding
                .context
                .iter()
                .take(self.max_context_nodes)
                .filter_map(|id| graph.node(*id))
                .map(|n| n.label.clone())
                .collect();

            if labels.len() < finding.context.len() {
                msg.push_str(&format!(
                    "\n           {} → ... ({} more)",
                    labels.join(" → "),
                    finding.context.len() - labels.len()
                ));
            }
        }

        msg
    }
}

fn pluralize(count: usize, word: &str) -> String {
    if count == 1 {
        format!("1 {}", word)
    } else {
        format!("{} {}s", count, word)
    }
}

fn red(text: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[31m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn yellow(text: &str, use_color: bool) -> String {
    if use_color {
        format!("\x1b[33m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

pub fn render_json(report: &Report) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}
