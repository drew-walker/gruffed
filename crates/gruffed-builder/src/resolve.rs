use std::path::{Path, PathBuf};

use oxc_resolver::{ResolveOptions, Resolver};

pub struct ImportResolver {
    resolver: Resolver,
}

impl ImportResolver {
    pub fn new(_root: &Path) -> Self {
        let resolver = Resolver::new(ResolveOptions {
            extensions: vec![
                ".ts".into(),
                ".tsx".into(),
                ".js".into(),
                ".jsx".into(),
                ".mjs".into(),
                ".cjs".into(),
                ".json".into(),
            ],
            // TypeScript convention: import specifiers use `.js` (and `.jsx`)
            // extensions even when the source file is `.ts`/`.tsx`. Resolve
            // those aliases while preserving same-extension JS/MJS/CJS targets.
            extension_alias: vec![
                (
                    ".js".into(),
                    vec![".js".into(), ".ts".into(), ".tsx".into()],
                ),
                (".jsx".into(), vec![".jsx".into(), ".tsx".into()]),
                (".mjs".into(), vec![".mjs".into(), ".mts".into()]),
                (".cjs".into(), vec![".cjs".into(), ".cts".into()]),
            ],
            ..ResolveOptions::default()
        });
        Self { resolver }
    }

    pub fn resolve(&self, source_file: &Path, specifier: &str) -> Option<PathBuf> {
        self.resolver
            .resolve(source_file.parent()?, specifier)
            .ok()
            .map(|info| info.path().to_path_buf())
    }
}
