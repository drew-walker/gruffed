use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gruffed_core::report::Severity;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GruffedConfig {
    #[serde(default)]
    pub root: Option<PathBuf>,
    #[serde(default)]
    pub entrypoints: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub rules: HashMap<String, RuleSetting>,
}

fn default_extensions() -> Vec<String> {
    vec![
        ".ts".to_string(),
        ".tsx".to_string(),
        ".js".to_string(),
        ".jsx".to_string(),
        ".mjs".to_string(),
        ".cjs".to_string(),
    ]
}

impl Default for GruffedConfig {
    fn default() -> Self {
        Self {
            root: None,
            entrypoints: vec![],
            exclude: vec![],
            extensions: default_extensions(),
            rules: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleSetting {
    Severity(Severity),
    WithOptions(Severity, HashMap<String, Value>),
    Off(String),
}

impl RuleSetting {
    pub fn is_enabled(&self) -> bool {
        match self {
            RuleSetting::Off(s) => s != "off",
            RuleSetting::Severity(_) => true,
            RuleSetting::WithOptions(_, _) => true,
        }
    }

    pub fn severity(&self) -> Option<Severity> {
        match self {
            RuleSetting::Off(_) => None,
            RuleSetting::Severity(s) | RuleSetting::WithOptions(s, _) => Some(s.clone()),
        }
    }

    pub fn options(&self) -> Option<&HashMap<String, Value>> {
        match self {
            RuleSetting::Off(_) | RuleSetting::Severity(_) => None,
            RuleSetting::WithOptions(_, opts) => Some(opts),
        }
    }
}

/// Strip JSONC comments and trailing commas, then parse as JSON.
fn strip_jsonc(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;

    while let Some(c) = chars.next() {
        if in_string {
            result.push(c);
            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                }
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' => {
                in_string = true;
                result.push(c);
            }
            '/' if chars.peek() == Some(&'/') => {
                // line comment
                for c in chars.by_ref() {
                    if c == '\n' {
                        result.push('\n');
                        break;
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                // block comment
                chars.next();
                let mut prev = '\0';
                for c in chars.by_ref() {
                    if prev == '*' && c == '/' {
                        break;
                    }
                    prev = c;
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

impl GruffedConfig {
    pub fn from_jsonc(jsonc: &str) -> Result<Self, ConfigError> {
        let stripped = strip_jsonc(jsonc);
        let config: Self = serde_json::from_str(&stripped)?;
        Ok(config)
    }

    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_jsonc(&content)
    }

    pub fn discover(start_dir: &Path) -> Result<Option<Self>, ConfigError> {
        let candidates = ["gruffed.jsonc", "gruffed.json"];
        for candidate in &candidates {
            let path = start_dir.join(candidate);
            if path.exists() {
                return Ok(Some(Self::from_file(&path)?));
            }
        }
        Ok(None)
    }

    pub fn discover_or_default(start_dir: &Path) -> Result<Self, ConfigError> {
        Ok(Self::discover(start_dir)?.unwrap_or_default())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Json(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Json(e) => write!(f, "config parse error: {}", e),
            ConfigError::Io(e) => write!(f, "config io error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::Json(e)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}
