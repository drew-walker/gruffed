use std::path::PathBuf;

use gruffed_builder::{BuildWarning, ModuleGraphBuilder};
use gruffed_core::graph::Value;

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

#[test]
fn builds_simple_graph() {
    let result = ModuleGraphBuilder::new(fixture_path("simple"))
        .build()
        .unwrap();

    assert_eq!(result.graph.node_count(), 3);
    // index.ts imports a.ts and b.ts; b.ts imports a.ts
    assert_eq!(result.graph.edge_count(), 3);
    assert!(result.warnings.is_empty());
}

#[test]
fn detects_unresolved_imports() {
    let result = ModuleGraphBuilder::new(fixture_path("unresolved"))
        .build()
        .unwrap();

    assert_eq!(result.graph.node_count(), 1);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn ignores_unresolved_bare_imports() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("index.ts");
    std::fs::write(
        &source,
        r#"
        import fs from "node:fs";
        import react from "react";
        import helper from "@/helper";
        import component from "./Component.svelte";
        import type { Thing } from "./types.d.ts";
        import missing from "./missing";
        "#,
    )
    .unwrap();

    let result = ModuleGraphBuilder::new(dir.path()).build().unwrap();

    assert_eq!(result.warnings.len(), 1);
    let warning = &result.warnings[0];
    assert!(format!("{warning:?}").contains("./missing"));
}

#[test]
fn builds_cycle_graph() {
    let result = ModuleGraphBuilder::new(fixture_path("cycle"))
        .build()
        .unwrap();

    assert_eq!(result.graph.node_count(), 3);
    assert_eq!(result.graph.edge_count(), 3);
}

#[test]
fn builds_long_chain_graph() {
    let result = ModuleGraphBuilder::new(fixture_path("long-chain"))
        .build()
        .unwrap();

    assert_eq!(result.graph.node_count(), 12);
    assert_eq!(result.graph.edge_count(), 11);
}

#[test]
fn stats_track_files_scanned() {
    let result = ModuleGraphBuilder::new(fixture_path("simple"))
        .build()
        .unwrap();

    assert_eq!(result.stats.files_scanned, 3);
}

#[test]
fn resolves_js_imports_to_existing_js_files() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("index.js"),
        r#"import { helper } from "./helper.js"; helper();"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("helper.js"), "export function helper() {}").unwrap();

    let result = ModuleGraphBuilder::new(dir.path()).build().unwrap();

    assert_eq!(result.graph.node_count(), 2);
    assert_eq!(result.graph.edge_count(), 1);
    assert!(result.warnings.is_empty());
}

#[test]
fn records_side_effect_import_edges() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("index.ts"), r#"import "./setup.js";"#).unwrap();
    std::fs::write(dir.path().join("setup.ts"), "export const ready = true;").unwrap();

    let result = ModuleGraphBuilder::new(dir.path()).build().unwrap();

    assert_eq!(result.graph.edge_count(), 1);
    let edge = result.graph.edges().next().unwrap();
    assert_eq!(
        edge.properties.get("import_kind"),
        Some(&Value::String("side-effect".to_string()))
    );
}

#[test]
fn records_type_only_import_edges() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("index.ts"),
        r#"import type { Thing } from "./types.js";"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("types.ts"), "export interface Thing {}").unwrap();

    let result = ModuleGraphBuilder::new(dir.path()).build().unwrap();

    assert_eq!(result.graph.edge_count(), 1);
    let edge = result.graph.edges().next().unwrap();
    assert_eq!(
        edge.properties.get("import_kind"),
        Some(&Value::String("type".to_string()))
    );
}

#[test]
fn warns_when_import_resolves_to_untracked_file() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("index.ts"),
        r#"import data from "./data.json";"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("data.json"), "{}").unwrap();

    let result = ModuleGraphBuilder::new(dir.path()).build().unwrap();

    assert_eq!(result.graph.edge_count(), 0);
    assert_eq!(result.warnings.len(), 1);
    assert!(matches!(
        &result.warnings[0],
        BuildWarning::ResolvedImportNotInGraph { specifier, .. } if specifier == "./data.json"
    ));
}
