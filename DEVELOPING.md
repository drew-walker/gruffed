# Developing gruffed

This repo contains the Rust core, napi-rs bindings, the npm wrapper, and two
Next.js apps for docs and product marketing.

## Requirements

- Rust stable
- Node.js 24 (see `.nvmrc`)
- pnpm 11.9.0

Version management does not use Corepack. Node is pinned via `.nvmrc`; pnpm is
enforced via `engines` and `devEngines.packageManager` in the root
`package.json`.

Install JavaScript dependencies from the repo root:

```sh
nvm use
npm install -g pnpm@11.9.0   # one-time bootstrap via npm
pnpm install
```

If you prefer not to install pnpm globally, bootstrap with:

```sh
nvm use
npm exec pnpm@11.9.0 install
```

## Rust Core

Run these from the repo root:

```sh
cargo build
cargo test
cargo bench
cargo tarpaulin
```

Useful local CLI smoke test:

```sh
cargo run -- --root fixtures/simple
```

The core flow is:

```text
gruffed-builder -> Graph -> gruffed-analyzer -> Report -> reporter
```

Keep analyzer code read-only over `&Graph`, and keep JS out of the hot path.

## JavaScript Workspace

The JavaScript projects are managed with pnpm workspaces:

```text
packages/gruffed  npm wrapper
packages/docs     docs website
packages/site     product site
```

Root quality scripts are Node-side only:

```sh
pnpm format:check
pnpm format
pnpm lint
pnpm typecheck
pnpm test
pnpm coverage
```

Package and site scripts:

```sh
pnpm build:sites
pnpm docs:typecheck
pnpm docs:build
pnpm site:typecheck
pnpm site:build
```

Default dev servers:

```sh
pnpm docs:dev
pnpm site:dev
```

To run both apps at once, use explicit ports:

```sh
cd packages/docs
pnpm exec next dev -p 3002

cd packages/site
pnpm exec next dev -p 3001
```

Then open:

- Docs: `http://localhost:3002/docs/cli`
- Product site: `http://localhost:3001`

## Quality Boundaries

Keep Rust and Node quality gates walled off.

- pnpm scripts are for JavaScript, TypeScript, npm packages, and Next.js apps.
- Cargo commands are for Rust crates, benchmarks, and Rust coverage.
- CI may run both ecosystems, but it should do so with separate commands rather
  than hiding Rust work behind pnpm scripts.

Current Node-side gates:

```sh
pnpm format:check
pnpm lint
pnpm typecheck
pnpm test
pnpm coverage
```

Current Rust-side gates:

```sh
cargo fmt --all -- --check
cargo build
cargo test
cargo tarpaulin --fail-under 90
```

The Rust coverage gate uses `tarpaulin.toml` to exclude FFI/CLI/config/parser
adapter surfaces from the threshold. Those areas are still tested through Rust
integration tests or Node/Vitest smoke coverage where appropriate.

## Docs App

The docs app lives in `packages/docs`.

- App Router routes live under `app/`.
- Prose-heavy pages use MDX.
- Interactive or data-driven pages use TSX.
- `mdx-components.tsx` maps MDX primitives to local styled components.
- Search is client-side and uses static metadata in `lib/nav.ts`.
- Do not add `globals.css`; use CSS Modules for layout, pages, and components.

## Product Site

The product site lives in `packages/site`.

- Pages are TSX only.
- Benchmark data is static JSON in `content/benchmarks.json`.
- Terminal samples live in `content/samples/`.
- The hero image lives in `public/images/gruffed-hero.png`.
- `NEXT_PUBLIC_GRUFFED_DOCS_URL` can override the docs link at deploy time.
- Do not add backend routes or live benchmark execution.
- Do not add shared UI imports from the docs app.

## Styling Rules

- Use CSS Modules only.
- Do not add `app/globals.css`.
- Keep components self-contained.
- Avoid nested cards and decorative gradient blobs.
- Use stable dimensions for fixed UI elements such as nav, terminal frames,
  icon buttons, tabs, and charts.
- Long terminal output should scroll inside the terminal frame, not widen the
  page.

## Before Hand-Off

Run Node-side quality gates:

```sh
pnpm format:check
pnpm lint
pnpm typecheck
pnpm test
pnpm coverage
```

Run Rust-side quality gates:

```sh
cargo fmt --all -- --check
cargo build
cargo test
cargo tarpaulin --fail-under 90
```

For substantial Rust performance changes, also run `cargo bench`.
