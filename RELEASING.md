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

Build and test the local native binding first:

```sh
pnpm node:build
pnpm node:test
```

For broad npm distribution, use the `npm Release` GitHub Actions workflow. It builds the native bindings on Linux x64, macOS x64, macOS arm64, and Windows x64, assembles the `@gruffed/node-*` optional native packages, then publishes those packages before publishing `@gruffed/node` and `gruffed`.

Manual publishing still follows the same order:

```sh
pnpm --filter @gruffed/node npm-dirs
pnpm --filter @gruffed/node exec napi artifacts --output-dir ../../native-artifacts
pnpm --filter @gruffed/node pre-publish
pnpm --filter @gruffed/node publish --access public --no-git-checks
pnpm --filter gruffed publish --access public --no-git-checks
```

`pnpm pack` rewrites `workspace:*` dependencies to the synchronized package version, so the packed `gruffed` package depends on `@gruffed/node` at the same version.

## Native npm Strategy

`@gruffed/node` is the public native binding entrypoint. It loads a local `gruffed-node.<platform>.node` file for contributor builds, then falls back to platform optional packages such as `@gruffed/node-darwin-arm64` or `@gruffed/node-linux-x64-gnu` for published installs.

The public `gruffed` wrapper stays ESM-only.

## Versioning

- Use synchronized versions across all packages.
- Use prereleases for rehearsal builds, for example `0.1.0-alpha.1`.
- Before 1.0, breaking changes should bump the minor version.
- Patch releases are for bug fixes and packaging repairs.
