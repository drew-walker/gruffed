# gruffed

> Fast module-graph analysis for JavaScript/TypeScript codebases.

gruffed builds a dependency graph from your source files and detects structural problems — circular imports, unresolvable imports, and excessive import chains. It's written in Rust for speed and ships Node.js bindings for programmatic use.

## What it does

- **Builds a module graph** from your TS/JS source files using [oxc](https://oxc.rs/) for parsing and resolution
- **Detects cycles** using Tarjan's strongly connected components algorithm (O(V+E))
- **Finds unresolvable imports** — imports that don't resolve to a real file
- **Flags excessive import chains** — modules whose import depth from an entrypoint exceeds a threshold

## Quick start

### CLI

```bash
# Install
cargo install gruffed-cli

# Run in your project (create a gruffed.jsonc to enable rules)
gruffed --root ./src

# JSON output for CI
gruffed --root ./src --format json
```

### Node.js

```bash
npm install gruffed
```

```ts
import { buildModuleGraph, analyzeGraph, renderReport } from 'gruffed';

const result = buildModuleGraph('./src', undefined);
const report = analyzeGraph(result.graph, result.warnings, JSON.stringify({
  rules: {
    'no-cycles': 'error',
    'no-unresolved': 'error',
  }
}));
console.log(renderReport(report, result.graph, true));
```

## Configuration

Create a `gruffed.jsonc` file in your project root:

```jsonc
{
  "root": "./src",
  "entrypoints": ["src/index.ts"],
  "exclude": ["**/*.test.ts", "node_modules/**"],
  "extensions": [".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"],
  "rules": {
    "no-cycles": "error",
    "no-unresolved": "error",
    "no-long-chains": ["warning", { "max": 10 }]
  }
}
```

### Rules

| Rule | Description | Default |
|------|-------------|---------|
| `no-cycles` | Detects circular dependencies | disabled |
| `no-unresolved` | Detects imports that can't be resolved | disabled |
| `no-long-chains` | Flags import chains exceeding a depth threshold | disabled |

All rules are disabled unless explicitly enabled in `gruffed.jsonc`. To enable the built-in rules, add a `rules` block:

Rule values follow ESLint convention:
- `"off"` — disabled
- `"error"` or `"warning"` — enabled with severity
- `["error", { "max": 10 }]` — enabled with custom options

## Roadmap

| Milestone | Scope | Status |
|-----------|-------|--------|
| 0.1 | Alpha foundation: CLI + Rust API, module graph, 3 rules, docs site, product site | Done |
| 0.1 release hardening | Installable packages, release workflows, license/security docs, CI quality gates | Planned |
| 0.2 | Package graph + TS project reference graph builders | Planned |
| 0.3 | Graph cache + incremental updates | Planned |
| 0.4 | Custom rule API + rule plugins | Planned |
| 0.5 | Additional output formats + CI integrations | Planned |
| 1.0 | API stabilization, performance hardening, full docs | Planned |

## Architecture

gruffed is a Cargo workspace with focused crates:

- `gruffed-core` — graph data model and report types
- `gruffed-config` — JSONC config schema and parsing
- `gruffed-builder` — module graph builder (oxc parser + resolver)
- `gruffed-analyzer` — rules engine with Tarjan's SCC for cycle detection
- `gruffed-console-reporter` — terminal output formatting
- `gruffed-cli` — the `gruffed` binary
- `gruffed-node` — napi-rs bindings for Node.js

## Development

```bash
cargo build          # build all crates
cargo test           # run all tests
cargo bench          # run benchmarks
cargo run -- --root fixtures/simple  # test against a fixture
```

## License

MIT. See [LICENSE](LICENSE).
