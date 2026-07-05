# Releasing gruffed

gruffed uses synchronized versions across Rust crates and npm packages. For an alpha release, use `0.1.0-alpha.1`; for the first public 0.1 release, use `0.1.0`.

## Package Names

- Rust CLI crate: `gruffed`
- Rust support crates: `gruffed-core`, `gruffed-config`, `gruffed-builder`, `gruffed-analyzer`, `gruffed-console-reporter`
- Rust napi crate: `gruffed-node`
- npm wrapper package: `gruffed`
- npm native package: `@gruffed/node`
- CLI binary: `gruffed`

## Preflight

Run this before publishing:

```sh
pnpm release:preflight
```

The preflight script runs Rust build/tests, Node native build/tests, packs both npm packages, installs the packed npm packages into a temporary project, runs the installed npm CLI, installs the Rust CLI into a temporary Cargo root, and runs the installed Rust CLI.

## Rust Publish Order

Publish Rust crates in dependency order:

```sh
cargo publish -p gruffed-core
cargo publish -p gruffed-config
cargo publish -p gruffed-builder
cargo publish -p gruffed-analyzer
cargo publish -p gruffed-console-reporter
cargo publish -p gruffed
```

`gruffed-node` is used for napi-rs builds and npm packaging. Do not publish it to crates.io unless there is a clear Rust-side consumer.

For the first publish, dependent crates cannot be fully packaged against crates.io until their internal dependencies have been published and the crates.io index has updated. This is expected.

## npm Publish Order

Build the native binding first:

```sh
pnpm node:build
```

Then publish:

```sh
pnpm --filter @gruffed/node publish --access public
pnpm --filter gruffed publish --access public
```

`pnpm pack` rewrites `workspace:*` dependencies to the synchronized package version, so the packed `gruffed` package depends on `@gruffed/node` at the same version.

## Native npm Strategy

The current alpha packaging strategy is a single `@gruffed/node` package containing the native `.node` file for the platform that built it. This is enough for local install verification and a narrow alpha on the build platform.

Before broad npm distribution, move to platform-specific native packages or a full napi-rs release workflow so macOS, Linux, and Windows users install the matching binary. The public `gruffed` wrapper should stay ESM-only.

## Versioning

- Use synchronized versions across all packages.
- Use prereleases for rehearsal builds, for example `0.1.0-alpha.1`.
- Before 1.0, breaking changes should bump the minor version.
- Patch releases are for bug fixes and packaging repairs.
