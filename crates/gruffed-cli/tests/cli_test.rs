use std::path::PathBuf;
use std::process::Command;

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_gruffed"))
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("fixtures")
        .join(name)
}

#[test]
fn simple_fixture_exits_zero() {
    let output = Command::new(binary_path())
        .args(["--root", fixture_path("simple").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "should exit zero, stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn cycle_fixture_detects_cycle() {
    let output = Command::new(binary_path())
        .args(["--root", fixture_path("cycle").to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "should exit non-zero for errors");
    assert!(stdout.contains("no-cycles"));
    assert!(stdout.contains("Circular dependency"));
}

#[test]
fn unresolved_fixture_detects_unresolved() {
    let output = Command::new(binary_path())
        .args(["--root", fixture_path("unresolved").to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success());
    assert!(stdout.contains("no-unresolved"));
    assert!(stdout.contains("Cannot resolve"));
}

#[test]
fn json_output_is_valid_json() {
    let output = Command::new(binary_path())
        .args([
            "--root",
            fixture_path("simple").to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("invalid JSON: {}\noutput: {}", e, stdout));
    assert!(parsed.is_object());
    assert!(parsed["findings"].is_array());
}
