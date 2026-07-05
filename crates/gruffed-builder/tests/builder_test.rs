use std::path::PathBuf;

use gruffed_builder::ModuleGraphBuilder;

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
