use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use gruffed_analyzer::Analyzer;
use gruffed_builder::{BuildWarning, ModuleGraphBuilder};
use gruffed_config::GruffedConfig;
use gruffed_console_reporter::ConsoleReporter;
use gruffed_core::graph::Graph;
use gruffed_core::report::Report;

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Internal registry mapping opaque handles to graphs living in Rust memory.
/// This keeps the graph out of JS — JS only ever holds a `u32` handle.
struct GraphRegistry {
    next_id: u32,
    graphs: HashMap<u32, Box<Graph>>,
}

static GRAPH_REGISTRY: OnceLock<Mutex<GraphRegistry>> = OnceLock::new();

fn registry() -> &'static Mutex<GraphRegistry> {
    GRAPH_REGISTRY.get_or_init(|| {
        Mutex::new(GraphRegistry {
            next_id: 1,
            graphs: HashMap::new(),
        })
    })
}

/// Build a module graph from a root directory.
///
/// Returns an opaque graph handle and warnings. The graph stays in Rust
/// memory; JS never sees the graph internals directly. Free the handle with
/// `freeGraph` when finished.
#[napi]
pub fn build_module_graph(root: String, config_json: Option<String>) -> Result<BuildResultJs> {
    let config = if let Some(json) = config_json {
        GruffedConfig::from_jsonc(&json)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?
    } else {
        GruffedConfig::default()
    };

    let build_result = ModuleGraphBuilder::new(&root)
        .with_extensions(config.extensions.clone())
        .with_exclude(config.exclude.clone())
        .with_entrypoints(config.entrypoints.clone())
        .build()
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    let warnings_js: Vec<JsBuildWarning> =
        build_result.warnings.iter().map(warning_to_js).collect();

    let handle = {
        let mut reg = registry()
            .lock()
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let id = reg.next_id;
        reg.next_id += 1;
        reg.graphs.insert(id, Box::new(build_result.graph));
        id
    };

    Ok(BuildResultJs {
        graph_handle: handle,
        warnings: warnings_js,
        stats: JsBuildStats {
            files_scanned: build_result.stats.files_scanned as u32,
            build_time_ms: build_result.stats.total_time_ms as u32,
        },
    })
}

/// Analyze a graph with the given rules config JSON.
/// Pass the `graphHandle` from `buildModuleGraph` and the `warnings` array it returned.
#[napi]
pub fn analyze_graph(
    graph_handle: u32,
    warnings: Vec<JsBuildWarning>,
    rules_json: String,
) -> Result<JsReport> {
    let config = GruffedConfig::from_jsonc(&rules_json)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

    let warnings: Vec<BuildWarning> = warnings.into_iter().map(js_to_warning).collect();

    let report = {
        let reg = registry()
            .lock()
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let graph = reg.graphs.get(&graph_handle).ok_or_else(|| {
            Error::new(
                Status::InvalidArg,
                format!("unknown graph handle {}", graph_handle),
            )
        })?;
        let analyzer = Analyzer::from_config(&config.rules);
        analyzer.run(graph.as_ref(), &warnings)
    };

    Ok(JsReport {
        findings: report
            .findings
            .iter()
            .map(|f| JsFinding {
                rule_id: f.rule_id.clone(),
                severity: format!("{:?}", f.severity),
                message: f.message.clone(),
                file: f.location.file.clone(),
                line: f.line_for_js(),
            })
            .collect(),
        stats: JsGraphStats {
            node_count: report.stats.node_count as u32,
            edge_count: report.stats.edge_count as u32,
        },
    })
}

/// Render a report (given as the JS report object) as terminal output.
#[napi]
pub fn render_report(report: JsReport, graph_handle: u32, use_color: bool) -> Result<String> {
    let report = js_report_to_report(&report);
    let reg = registry()
        .lock()
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    let graph = reg.graphs.get(&graph_handle).ok_or_else(|| {
        Error::new(
            Status::InvalidArg,
            format!("unknown graph handle {}", graph_handle),
        )
    })?;
    let reporter = ConsoleReporter::new(use_color);
    Ok(reporter.render(&report, graph.as_ref()))
}

/// Free a graph handle previously returned by `build_module_graph`.
#[napi]
pub fn free_graph(graph_handle: u32) -> Result<()> {
    let mut reg = registry()
        .lock()
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    reg.graphs.remove(&graph_handle);
    Ok(())
}

fn warning_to_js(w: &BuildWarning) -> JsBuildWarning {
    match w {
        BuildWarning::UnresolvedImport {
            source,
            specifier,
            line,
        } => JsBuildWarning {
            kind: "unresolved".to_string(),
            source: source.to_string_lossy().to_string(),
            specifier: Some(specifier.clone()),
            resolved: None,
            line: Some(*line),
            error: None,
        },
        BuildWarning::ResolvedImportNotInGraph {
            source,
            specifier,
            resolved,
            line,
        } => JsBuildWarning {
            kind: "resolved_not_in_graph".to_string(),
            source: source.to_string_lossy().to_string(),
            specifier: Some(specifier.clone()),
            resolved: Some(resolved.to_string_lossy().to_string()),
            line: Some(*line),
            error: None,
        },
        BuildWarning::ParseFailure { file, error } => JsBuildWarning {
            kind: "parse_failure".to_string(),
            source: file.to_string_lossy().to_string(),
            specifier: None,
            resolved: None,
            line: None,
            error: Some(error.clone()),
        },
        BuildWarning::ResolveFailure {
            source,
            specifier,
            line,
            error,
        } => JsBuildWarning {
            kind: "resolve_failure".to_string(),
            source: source.to_string_lossy().to_string(),
            specifier: Some(specifier.clone()),
            resolved: None,
            line: Some(*line),
            error: Some(error.clone()),
        },
    }
}

fn js_to_warning(w: JsBuildWarning) -> BuildWarning {
    match w.kind.as_str() {
        "parse_failure" => BuildWarning::ParseFailure {
            file: w.source.into(),
            error: w.error.unwrap_or_default(),
        },
        "resolved_not_in_graph" => BuildWarning::ResolvedImportNotInGraph {
            source: w.source.into(),
            specifier: w.specifier.unwrap_or_default(),
            resolved: w.resolved.unwrap_or_default().into(),
            line: w.line.unwrap_or(0),
        },
        "resolve_failure" => BuildWarning::ResolveFailure {
            source: w.source.into(),
            specifier: w.specifier.unwrap_or_default(),
            line: w.line.unwrap_or(0),
            error: w.error.unwrap_or_default(),
        },
        _ => BuildWarning::UnresolvedImport {
            source: w.source.into(),
            specifier: w.specifier.unwrap_or_default(),
            line: w.line.unwrap_or(0),
        },
    }
}

fn js_report_to_report(js: &JsReport) -> Report {
    use gruffed_core::report::{Finding, GraphStats, Location, Severity};

    let findings = js
        .findings
        .iter()
        .map(|f| {
            let severity = match f.severity.as_str() {
                "Warning" => Severity::Warning,
                _ => Severity::Error,
            };
            Finding {
                rule_id: f.rule_id.clone(),
                severity,
                message: f.message.clone(),
                location: Location {
                    file: f.file.clone(),
                    line: f.line,
                    column: None,
                },
                context: vec![],
            }
        })
        .collect();

    Report {
        findings,
        stats: GraphStats {
            node_count: js.stats.node_count as usize,
            edge_count: js.stats.edge_count as usize,
            build_time_ms: 0,
            analyze_time_ms: 0,
        },
    }
}

/// Helper trait to read the line from a Finding for JS.
trait FindingLineExt {
    fn line_for_js(&self) -> Option<u32>;
}

impl FindingLineExt for gruffed_core::report::Finding {
    fn line_for_js(&self) -> Option<u32> {
        self.location.line
    }
}

#[napi(object)]
pub struct BuildResultJs {
    /// Opaque handle to the graph in Rust memory. Pass to `analyzeGraph` / `renderReport`,
    /// then free with `freeGraph` when done.
    pub graph_handle: u32,
    pub warnings: Vec<JsBuildWarning>,
    pub stats: JsBuildStats,
}

#[napi(object)]
pub struct JsBuildStats {
    pub files_scanned: u32,
    pub build_time_ms: u32,
}

#[napi(object)]
pub struct JsBuildWarning {
    pub kind: String,
    pub source: String,
    pub specifier: Option<String>,
    pub resolved: Option<String>,
    pub line: Option<u32>,
    pub error: Option<String>,
}

#[napi(object)]
pub struct JsReport {
    pub findings: Vec<JsFinding>,
    pub stats: JsGraphStats,
}

#[napi(object)]
pub struct JsFinding {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
    pub file: String,
    pub line: Option<u32>,
}

#[napi(object)]
pub struct JsGraphStats {
    pub node_count: u32,
    pub edge_count: u32,
}
