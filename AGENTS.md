# AGENTS.md

Guidance for AI agents (and humans) working on gruffed.

## What gruffed is

gruffed is a fast, module-graph analysis tool for JavaScript/TypeScript codebases. It builds a dependency graph from source files, detects structural problems (cycles, unresolvable imports, excessive chain depth), and reports findings. The core is Rust; Node.js bindings via napi-rs.

**What it is NOT:** a visualization library, a database, a query language, or a type checker. It lints dependency graphs.

## Key tenets

### 1. Separation of concerns is non-negotiable

The build → analyze → report flow is enforced at the type level:
- `gruffed-builder` produces a `Graph`
- `gruffed-analyzer` consumes a `&Graph` (immutable)
- Reporters consume `&Report` + `&Graph`

The analyzer never mutates the graph. Reporters never touch graph internals. No crate depends on another's internals — they communicate through `gruffed-core` types.

### 2. Performance is a feature

This tool targets 10M+ edge graphs. Every design decision must hold up at that scale:
- u64 IDs, not string keys
- Bidirectional adjacency indexes
- `SmallVec` for adjacency lists (common case: <8 neighbors)
- Path interning (no duplication)
- `Cow<str>` / `Arc` to avoid clones
- Parallel file parsing via `rayon`
- Zero-copy parsing where possible

If a change adds clones, allocations, or O(n²) behavior, it needs a strong justification.

### 3. Type safety over convenience

- IDs are newtypes with private inner values — can't be faked
- Exhaustive pattern matching, no fallthroughs
- No `any` in TS bindings — types generated from Rust via napi-rs
- `Option` only where genuinely optional; not as a lazy escape hatch

### 4. No JS in the hot path

All computation happens in Rust. The napi layer is a thin shim. The graph is never serialized through JS — it stays in Rust memory as an opaque handle.

### 5. Config is configuration, not logic

JSONC config enables/configures built-in rules. No inline rule definitions, no custom logic in config. Custom rules (when added in v0.4) come via a plugin mechanism, not config.

### 6. Detection only, no auto-fix

v1 (and pre-1.0 generally) detects problems. It does not fix them. Findings are for humans to act on.

### 7. High test coverage is enforced

- Unit tests per crate
- Integration tests for cross-module behavior
- Fixture projects in `fixtures/` with known structures
- `proptest` for graph invariants
- `cargo tarpaulin` with 90%+ floor in CI
- `criterion` benchmarks for hot paths; regressions fail CI

### 8. Pre-1.0 means unstable

Nothing before 1.0 is a stable public API. Breaking changes can occur in any 0.x release. Don't over-engineer for backward compatibility yet — let the API stabilize through real usage first.

## Workspace layout

```
crates/
├── gruffed-core/              # shared atom: graph data model + report types
├── gruffed-builder/           # module graph builder (oxc parser + resolver)
├── gruffed-analyzer/          # rules engine + built-in rules
├── gruffed-config/            # JSONC config schema + parsing
├── gruffed-console-reporter/  # pretty terminal formatter
├── gruffed-cli/               # the "gruffed" binary
└── gruffed-node/              # napi-rs bindings
packages/
└── gruffed/                   # npm package: TS types + thin wrapper
```

Dependency graph is acyclic. See `docs/superpowers/specs/2026-06-24-gruffed-design.md` for the full design.

## Build & test commands

(Run these before considering work complete)

```sh
cargo build              # build all crates
cargo test               # run all tests
cargo bench              # run benchmarks
cargo tarpaulin          # coverage check (CI gate: 90%+)
```

## Roadmap

| Milestone | Scope |
|-----------|-------|
| 0.1 | Full rebuild, CLI + lib API, module graph, 3 rules |
| 0.2 | Package graph + TS project reference graph builders |
| 0.3 | Graph cache + incremental updates |
| 0.4 | Custom rule API + rule plugins |
| 0.5 | Additional output formats + CI integrations |
| 1.0 | API stabilization, performance hardening, full docs |
