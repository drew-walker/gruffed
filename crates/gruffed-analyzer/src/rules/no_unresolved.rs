use gruffed_builder::BuildWarning;
use gruffed_core::graph::Graph;
use gruffed_core::report::{Finding, Location, Severity};

use crate::Rule;

pub struct NoUnresolvedRule {
    severity: Severity,
}

impl NoUnresolvedRule {
    pub fn new(severity: Severity) -> Self {
        Self { severity }
    }
}

impl Rule for NoUnresolvedRule {
    fn id(&self) -> &'static str {
        "no-unresolved"
    }

    fn analyze(&self, _graph: &Graph, warnings: &[BuildWarning]) -> Vec<Finding> {
        warnings
            .iter()
            .filter_map(|w| match w {
                BuildWarning::UnresolvedImport {
                    source,
                    specifier,
                    line,
                } => Some(Finding {
                    rule_id: self.id().to_string(),
                    severity: self.severity.clone(),
                    message: format!(
                        "Cannot resolve import \"{}\" in {}:{}",
                        specifier,
                        source.display(),
                        line
                    ),
                    location: Location {
                        file: source.to_string_lossy().to_string(),
                        line: Some(*line),
                        column: None,
                    },
                    context: vec![],
                }),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn reports_unresolved_imports() {
        let graph = Graph::new();
        let warnings = vec![
            BuildWarning::UnresolvedImport {
                source: PathBuf::from("src/index.ts"),
                specifier: "./missing".to_string(),
                line: 5,
            },
            BuildWarning::UnresolvedImport {
                source: PathBuf::from("src/app.ts"),
                specifier: "./nonexistent".to_string(),
                line: 12,
            },
        ];

        let rule = NoUnresolvedRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &warnings);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "no-unresolved");
        assert!(findings[0].message.contains("./missing"));
        assert_eq!(findings[0].location.line, Some(5));
    }

    #[test]
    fn ignores_parse_failures() {
        let graph = Graph::new();
        let warnings = vec![BuildWarning::ParseFailure {
            file: PathBuf::from("src/broken.ts"),
            error: "syntax error".to_string(),
        }];

        let rule = NoUnresolvedRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &warnings);
        assert!(findings.is_empty());
    }

    #[test]
    fn no_warnings_produces_no_findings() {
        let graph = Graph::new();
        let rule = NoUnresolvedRule::new(Severity::Error);
        let findings = rule.analyze(&graph, &[]);
        assert!(findings.is_empty());
    }
}
