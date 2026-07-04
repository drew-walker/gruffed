# gruffed - Current Unfinished Work

**Status:** Active planning

This document tracks remaining product and engineering work only.

## Current Roadmap

| Milestone | Scope | Status |
| --- | --- | --- |
| 0.2 | Package graph + TypeScript project reference graph builders | Planned |
| 0.3 | Graph cache + incremental updates | Planned |
| 1.0 | API stabilization, performance hardening, full docs, release readiness | Planned |

## 0.2 - Package And Project Graphs

Add graph builders beyond the current source-module graph.

### Package graph

- Discover workspace packages from common package manager layouts.
- Model package-level dependency edges separately from source-module import edges.
- Detect package cycles and unresolved package references.
- Keep package graph types in `gruffed-core`; builder-specific discovery stays in
  `gruffed-builder`.
- Do not serialize package graphs through JavaScript. Node callers should receive
  opaque handles the same way module graph callers do.

### TypeScript project reference graph

- Discover `tsconfig.json` files and `references` entries.
- Resolve project references to concrete config paths.
- Model project-to-project edges independently from source imports.
- Report missing, circular, or unexpectedly deep project reference chains.
- Avoid becoming a TypeScript type checker; this is structural graph analysis
  only.

### Expected acceptance

- Fixture workspaces cover npm/pnpm-style packages and TS project references.
- Analyzer rules can run against package and project graphs without mutating
  graph state.
- CLI and Node APIs expose the new graph builders without changing the existing
  module graph hot path.

## 0.3 - Cache And Incremental Updates

Add persistent graph cache support for faster repeated analysis.

### Cache contents

- File path, size, modified time, and content hash.
- Parsed import specifiers and source locations.
- Resolution results and unresolved import warnings.
- Enough version metadata to invalidate incompatible cache files safely.

### Incremental rebuild behavior

- Re-parse changed files only.
- Re-resolve affected imports when changed files alter exports, paths, or
  resolver inputs.
- Reuse unchanged graph nodes and edges where IDs can remain stable.
- Preserve correctness over clever partial reuse when invalidation is uncertain.

### Expected acceptance

- Cold builds match current full rebuild behavior.
- Warm builds skip unchanged parsing work.
- Fixture tests cover changed, deleted, added, and moved files.
- Benchmarks show meaningful warm-build improvement without regressing cold
  builds.

## Release Readiness Toward 1.0

The 1.0 line should focus on making the current public surface dependable rather
than expanding product scope.

- Stabilize Rust and Node public APIs.
- Document supported config fields, rule settings, and CLI behavior.
- Replace sample benchmark data with repeatable release benchmark snapshots.
- Configure deployment for `packages/docs` and `packages/site`.
- Keep coverage and benchmark gates in CI.
- Audit hot paths for avoidable clones, allocations, and accidental quadratic
  behavior.

## Constraints

- Keep the build -> analyze -> report separation intact.
- Keep analyzers read-only over graph data.
- Keep JavaScript out of the computation hot path.
- Treat config as configuration, not custom logic.
- Detect problems; do not auto-fix them.
