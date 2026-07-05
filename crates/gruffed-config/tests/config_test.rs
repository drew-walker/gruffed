use gruffed_config::{GruffedConfig, RuleSetting};
use gruffed_core::report::Severity;

#[test]
fn parses_minimal_config() {
    let jsonc = r#"{ "root": "./src" }"#;
    let config = GruffedConfig::from_jsonc(jsonc).unwrap();
    assert_eq!(config.root, Some("./src".into()));
}

#[test]
fn parses_jsonc_with_comments() {
    let jsonc = r#"
    // this is a comment
    {
      "root": "./src",
      // another comment
      "rules": {
        "no-cycles": "error"
      }
    }
    "#;
    let config = GruffedConfig::from_jsonc(jsonc).unwrap();
    assert_eq!(config.root, Some("./src".into()));
    assert_eq!(
        config.rules.get("no-cycles"),
        Some(&RuleSetting::Severity(Severity::Error))
    );
}

#[test]
fn parses_rule_with_options() {
    let jsonc = r#"
    {
      "rules": {
        "no-long-chains": ["warning", { "max": 10 }]
      }
    }
    "#;
    let config = GruffedConfig::from_jsonc(jsonc).unwrap();
    let setting = config.rules.get("no-long-chains").unwrap();
    assert_eq!(setting.severity(), Some(Severity::Warning));
    let opts = setting.options().unwrap();
    assert_eq!(opts.get("max").unwrap(), 10);
}

#[test]
fn parses_rule_off() {
    let jsonc = r#"
    {
      "rules": {
        "no-cycles": "off"
      }
    }
    "#;
    let config = GruffedConfig::from_jsonc(jsonc).unwrap();
    let setting = config.rules.get("no-cycles").unwrap();
    assert!(!setting.is_enabled());
}

#[test]
fn default_config_has_expected_extensions() {
    let config = GruffedConfig::default();
    assert_eq!(config.extensions.len(), 6);
    assert!(config.extensions.contains(&".ts".to_string()));
}

#[test]
fn config_from_file_works() {
    use tempfile::NamedTempFile;
    let file = NamedTempFile::new().unwrap();
    std::fs::write(
        &file,
        r#"{
          // config
          "root": "./lib"
        }"#,
    )
    .unwrap();
    let config = GruffedConfig::from_file(file.path()).unwrap();
    assert_eq!(config.root, Some("./lib".into()));
}

#[test]
fn discover_finds_jsonc_in_directory() {
    use tempfile::TempDir;
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("gruffed.jsonc"), r#"{ "root": "./found" }"#).unwrap();
    let config = GruffedConfig::discover_or_default(dir.path()).unwrap();
    assert_eq!(config.root, Some("./found".into()));
}

#[test]
fn discover_returns_default_when_no_config() {
    use tempfile::TempDir;
    let dir = TempDir::new().unwrap();
    let config = GruffedConfig::discover_or_default(dir.path()).unwrap();
    assert_eq!(config.root, None);
}
