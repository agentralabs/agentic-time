//! Integration tests for the CLI binary.
//!
//! Verifies that the `atime` binary exists and responds to basic flags.
//!
//! This test is registered as a [[test]] in the agentic-time-cli crate
//! so that CARGO_BIN_EXE_atime is available.

use std::process::Command;
use tempfile::tempdir;

/// Get a Command pointing to the `atime` binary.
fn atime_binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_atime"))
}

#[test]
fn cli_responds_to_help() {
    let output = atime_binary()
        .arg("--help")
        .output()
        .expect("failed to execute atime --help");

    assert!(
        output.status.success(),
        "atime --help should exit with success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("atime") || stdout.contains("AgenticTime") || stdout.contains("Usage"),
        "atime --help output should contain usage information, got: {stdout}"
    );
}

#[test]
fn cli_responds_to_version() {
    let output = atime_binary()
        .arg("--version")
        .output()
        .expect("failed to execute atime --version");

    assert!(
        output.status.success(),
        "atime --version should exit with success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0.") || stdout.contains("agentic-time"),
        "atime --version output should contain a version number, got: {stdout}"
    );
}

#[test]
fn cli_decay_value_invalid_id_exits_nonzero() {
    let dir = tempdir().expect("failed to create temp dir");
    let atime_path = dir.path().join("test.atime");
    let atime_path_str = atime_path.to_string_lossy().to_string();

    let create = atime_binary()
        .args([
            "--path",
            &atime_path_str,
            "decay",
            "create",
            "freshness",
            "--initial",
            "1.0",
        ])
        .output()
        .expect("failed to execute atime decay create");

    assert!(
        create.status.success(),
        "atime decay create should succeed, stderr: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let invalid = atime_binary()
        .args(["--path", &atime_path_str, "decay", "value", "freshness"])
        .output()
        .expect("failed to execute atime decay value with invalid id");

    assert!(
        !invalid.status.success(),
        "atime decay value with invalid id should exit non-zero"
    );

    let stderr = String::from_utf8_lossy(&invalid.stderr);
    assert!(
        stderr.contains("Invalid ID"),
        "expected invalid ID error, got: {stderr}"
    );
}
