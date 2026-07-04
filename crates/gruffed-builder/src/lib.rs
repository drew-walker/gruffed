use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gruffed_core::graph::{EdgeKind, Graph, NodeId, NodeKind, Value};
use gruffed_core::interner::PathInterner;
use gruffed_core::report::GraphStats;
use rayon::prelude::*;

pub mod discover;
pub mod parse;
pub mod resolve;

use discover::{discover_files, SourceType};
use parse::extract_imports;
use resolve::ImportResolver;

#[derive(Debug)]
pub struct ModuleGraphBuilder {
    root: PathBuf,
    extensions: Vec<String>,
    exclude: Vec<String>,
    entrypoints: Vec<String>,
}

#[derive(Debug)]
pub struct BuildResult {
    pub graph: Graph,
    pub warnings: Vec<BuildWarning>,
    pub stats: BuildStats,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct BuildStats {
    pub files_scanned: usize,
    pub parse_time_ms: u64,
    pub resolve_time_ms: u64,
    pub total_time_ms: u64,
}

impl BuildStats {
    pub fn to_graph_stats(&self, node_count: usize, edge_count: usize) -> GraphStats {
        GraphStats {
            node_count,
            edge_count,
            build_time_ms: self.total_time_ms,
            analyze_time_ms: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildWarning {
    UnresolvedImport {
        source: PathBuf,
        specifier: String,
        line: u32,
    },
    ParseFailure {
        file: PathBuf,
        error: String,
    },
}

#[derive(Debug)]
pub enum BuildError {
    IoError(std::io::Error),
}

/// Result of parsing and resolving a single file in parallel.
struct ParseOutput {
    #[allow(dead_code)]
    source_node: NodeId,
    edges: Vec<(NodeId, NodeId)>,
    warnings: Vec<BuildWarning>,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::IoError(e) => write!(f, "build io error: {}", e),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::IoError(e)
    }
}

impl ModuleGraphBuilder {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            extensions: vec![
                ".ts".to_string(),
                ".tsx".to_string(),
                ".js".to_string(),
                ".jsx".to_string(),
                ".mjs".to_string(),
                ".cjs".to_string(),
            ],
            exclude: vec![],
            entrypoints: vec![],
        }
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn with_exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = exclude;
        self
    }

    pub fn with_entrypoints(mut self, entrypoints: Vec<String>) -> Self {
        self.entrypoints = entrypoints;
        self
    }

    pub fn build(self) -> Result<BuildResult, BuildError> {
        let start = Instant::now();

        let files = discover_files(&self.root, &self.extensions, &self.exclude);

        let resolver = ImportResolver::new(&self.root);
        let mut interner = PathInterner::new();
        let mut graph = Graph::new();
        let mut warnings = Vec::new();

        // First pass: create nodes for all discovered files
        let mut file_to_node: HashMap<PathBuf, NodeId> = HashMap::new();
        for file in &files {
            let canonical = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
            let label = canonical.to_string_lossy().to_string();
            let node_id = graph.add_node(
                NodeKind::Module,
                label.clone(),
                build_node_properties(file.source_type, &self.entrypoints, &file.path, &self.root),
            );
            let _ = interner.intern(canonical.clone());
            file_to_node.insert(canonical, node_id);
        }

        // Second pass: parse imports and resolve edges in parallel.
        // Graph mutation stays sequential — each file produces a list of
        // (source_node, target_node) edges and warnings that we apply after.
        let parse_start = Instant::now();

        let source_nodes: Vec<NodeId> = files
            .iter()
            .map(|file| {
                let canonical = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
                *file_to_node.get(&canonical).unwrap()
            })
            .collect();

        let outputs: Vec<ParseOutput> = files
            .par_iter()
            .enumerate()
            .map(|(idx, file)| {
                let source_node = source_nodes[idx];
                let source_text = match std::fs::read_to_string(&file.path) {
                    Ok(text) => text,
                    Err(e) => {
                        return ParseOutput {
                            source_node,
                            edges: Vec::new(),
                            warnings: vec![BuildWarning::ParseFailure {
                                file: file.path.clone(),
                                error: e.to_string(),
                            }],
                        };
                    }
                };

                let imports = extract_imports(&source_text, &file.path);

                let mut edges = Vec::with_capacity(imports.len());
                let mut warnings = Vec::new();

                for import in imports {
                    match resolver.resolve(&file.path, &import.specifier) {
                        Some(resolved) => {
                            let resolved_canonical =
                                resolved.canonicalize().unwrap_or(resolved.clone());
                            if let Some(&target_node) = file_to_node.get(&resolved_canonical) {
                                edges.push((source_node, target_node));
                            }
                        }
                        None => {
                            warnings.push(BuildWarning::UnresolvedImport {
                                source: file.path.clone(),
                                specifier: import.specifier.clone(),
                                line: import.line,
                            });
                        }
                    }
                }

                ParseOutput {
                    source_node,
                    edges,
                    warnings,
                }
            })
            .collect();

        let parse_elapsed = parse_start.elapsed();

        // Apply edges and warnings sequentially
        for output in &outputs {
            for &(source, target) in &output.edges {
                graph.add_edge(source, target, EdgeKind::Imports, HashMap::new());
            }
            warnings.extend(output.warnings.iter().cloned());
        }

        let total_elapsed = start.elapsed();

        Ok(BuildResult {
            graph,
            warnings,
            stats: BuildStats {
                files_scanned: files.len(),
                parse_time_ms: parse_elapsed.as_millis() as u64,
                resolve_time_ms: 0,
                total_time_ms: total_elapsed.as_millis() as u64,
            },
        })
    }
}

fn build_node_properties(
    source_type: SourceType,
    entrypoints: &[String],
    file_path: &Path,
    root: &Path,
) -> HashMap<String, Value> {
    let mut props = HashMap::new();
    let type_str = match source_type {
        SourceType::TypeScript => "ts",
        SourceType::TypeScriptTsx => "tsx",
        SourceType::JavaScript => "js",
        SourceType::JavaScriptJsx => "jsx",
        SourceType::ESModule => "mjs",
        SourceType::CommonJS => "cjs",
    };
    props.insert("source_type".to_string(), Value::String(type_str.to_string()));

    // Match entrypoint patterns against the file path relative to root,
    // falling back to suffix matching for robustness.
    let rel = file_path
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let is_entrypoint = entrypoints.iter().any(|e| {
        if glob::Pattern::new(e)
            .map(|p| p.matches(&rel))
            .unwrap_or(false)
        {
            return true;
        }
        file_path
            .to_str()
            .map(|s| s.ends_with(e))
            .unwrap_or(false)
    });
    props.insert("is_entrypoint".to_string(), Value::Bool(is_entrypoint));
    props
}
