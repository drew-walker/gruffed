use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

#[derive(Debug)]
pub struct DiscoveredFile {
    pub path: PathBuf,
    pub source_type: SourceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    TypeScript,
    TypeScriptTsx,
    JavaScript,
    JavaScriptJsx,
    ESModule,
    CommonJS,
}

impl SourceType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            ".ts" => Some(Self::TypeScript),
            ".tsx" => Some(Self::TypeScriptTsx),
            ".js" | ".jsx" => Some(Self::JavaScript),
            ".mjs" => Some(Self::ESModule),
            ".cjs" => Some(Self::CommonJS),
            _ => None,
        }
    }
}

pub fn discover_files(
    root: &Path,
    extensions: &[String],
    exclude: &[String],
) -> Vec<DiscoveredFile> {
    let mut builder = WalkBuilder::new(root);
    builder.hidden(true);
    builder.git_ignore(true);
    builder.git_exclude(true);

    if !exclude.is_empty() {
        let mut overrides = ignore::overrides::OverrideBuilder::new(root);
        for pattern in exclude {
            let _ = overrides.add(&format!("!{}", pattern));
        }
        if let Ok(built) = overrides.build() {
            builder.overrides(built);
        }
    }

    builder
        .build()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.into_path();
            let ext = path.extension()?.to_str()?;
            let dot_ext = format!(".{}", ext);
            if !extensions.contains(&dot_ext) {
                return None;
            }
            let source_type = SourceType::from_extension(&dot_ext)?;
            Some(DiscoveredFile { path, source_type })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn discovers_ts_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("a.ts"), "").unwrap();
        fs::write(dir.path().join("b.ts"), "").unwrap();
        fs::write(dir.path().join("readme.md"), "").unwrap();

        let files = discover_files(dir.path(), &[".ts".to_string()], &[]);
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn respects_gitignore() {
        let dir = TempDir::new().unwrap();
        // The `ignore` crate honors `.gitignore` only inside git repositories.
        let _ = std::process::Command::new("git")
            .args(["init", dir.path().to_str().unwrap()])
            .output();
        fs::write(dir.path().join(".gitignore"), "*.test.ts\n").unwrap();
        fs::write(dir.path().join("a.ts"), "").unwrap();
        fs::write(dir.path().join("a.test.ts"), "").unwrap();

        let files = discover_files(dir.path(), &[".ts".to_string()], &[]);
        assert_eq!(files.len(), 1);
        assert!(files[0].path.file_name().unwrap() == "a.ts");
    }

    #[test]
    fn excludes_patterns() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("node_modules")).unwrap();
        fs::write(dir.path().join("node_modules/lib.js"), "").unwrap();
        fs::write(dir.path().join("src.ts"), "").unwrap();

        let files = discover_files(
            dir.path(),
            &[".ts".to_string(), ".js".to_string()],
            &["node_modules/**".to_string()],
        );
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn source_type_detection() {
        assert_eq!(
            SourceType::from_extension(".ts"),
            Some(SourceType::TypeScript)
        );
        assert_eq!(
            SourceType::from_extension(".tsx"),
            Some(SourceType::TypeScriptTsx)
        );
        assert_eq!(
            SourceType::from_extension(".mjs"),
            Some(SourceType::ESModule)
        );
        assert_eq!(SourceType::from_extension(".unknown"), None);
    }
}
