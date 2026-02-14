use std::{fs, path::PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;

fn example_path(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(relative);
    path
}

fn jasn_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("jasn"))
}

#[test]
fn test_format_stdin() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_compact() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("--compact")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("{test:123}"));
}

#[test]
fn test_format_file() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg(example_path("examples/valid/basic.jasn"))
        .assert()
        .success()
        .stdout(predicate::str::contains("{"));
}

#[test]
fn test_format_stdin_explicit() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("-")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_check_valid() {
    let mut cmd = jasn_cmd();
    // First format a file to get valid output
    let formatted = jasn_cmd()
        .arg("format")
        .write_stdin(r#"{"test": 123}"#)
        .output()
        .unwrap()
        .stdout;

    // Now check if it's formatted
    cmd.arg("format")
        .arg("--check-format")
        .write_stdin(formatted)
        .assert()
        .success();
}

#[test]
fn test_format_check_invalid() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("--check-format")
        .write_stdin(r#"{"test":123}"#) // Not formatted
        .assert()
        .failure()
        .stderr(predicate::str::contains("not formatted correctly"));
}

#[test]
fn test_check_valid_file() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg(example_path("examples/valid/basic.jasn"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_check_invalid_file() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg(example_path("examples/invalid/invalid_escape.jasn"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_check_multiple_files() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg(example_path("examples/valid/basic.jasn"))
        .arg(example_path("examples/valid/minimal.jasn"))
        .assert()
        .success()
        .stdout(predicate::str::contains("All 2 file(s) are valid"));
}

#[test]
fn test_check_stdin() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid JASN"));
}

#[test]
fn test_check_stdin_explicit() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg("-")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_check_quiet_success() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg("--quiet")
        .arg(example_path("examples/valid/basic.jasn"))
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_check_quiet_failure() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg("--quiet")
        .arg(example_path("examples/invalid/invalid_escape.jasn"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_check_verbose() {
    let mut cmd = jasn_cmd();
    cmd.arg("check")
        .arg("--verbose")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid JASN:"));
}

#[test]
fn test_completions_bash() {
    let mut cmd = jasn_cmd();
    cmd.arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("_jasn"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = jasn_cmd();
    cmd.arg("completions")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef jasn"));
}

#[test]
fn test_format_output_file() {
    let temp_file = "/tmp/jasn_test_output.jasn";

    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("-o")
        .arg(temp_file)
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success();

    let content = fs::read_to_string(temp_file).unwrap();
    assert!(content.contains("test: 123"));

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_format_output_stdout_explicit() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("-o")
        .arg("-")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_custom_options() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("--quotes")
        .arg("single")
        .arg("--binary")
        .arg("hex")
        .arg("--quote-keys")
        .write_stdin(r#"{"test": "value", "data": b64"SGVsbG8="}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("'test'"))
        .stdout(predicate::str::contains("hex\""));
}

#[test]
fn test_format_no_trailing_commas() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .arg("--no-trailing-commas")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("123\n}"));
}

#[test]
fn test_invalid_json_parse_error() {
    let mut cmd = jasn_cmd();
    cmd.arg("format")
        .write_stdin(r#"{"test": invalid}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_help_message() {
    let mut cmd = jasn_cmd();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("JASN"))
        .stdout(predicate::str::contains("Usage: jasn"));
}

#[test]
fn test_version() {
    let mut cmd = jasn_cmd();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("jasn"));
}

#[test]
fn test_format_alias() {
    let mut cmd = jasn_cmd();
    cmd.arg("fmt")
        .write_stdin(r#"{"test": 123}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_check_alias() {
    let mut cmd = jasn_cmd();
    cmd.arg("chk")
        .arg(example_path("examples/valid/basic.jasn"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}
