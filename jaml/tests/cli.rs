use std::{fs, path::PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;

fn example_path(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(relative);
    path
}

fn jaml_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("jaml"))
}

#[test]
fn test_format_stdin() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_file() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg(example_path("examples/valid/basic.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("numeric"));
}

#[test]
fn test_format_stdin_explicit() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("-")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_check_valid() {
    let mut cmd = jaml_cmd();
    // First format a string to get valid output
    let formatted = jaml_cmd()
        .arg("format")
        .write_stdin("test: 123")
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
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--check-format")
        .write_stdin("# Comment\ntest: 123\n") // Missing trailing newline in expected output
        .assert()
        .failure()
        .stderr(predicate::str::contains("not formatted correctly"));
}

#[test]
fn test_check_valid_file() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg(example_path("examples/valid/basic.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_check_invalid_file() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg(example_path("examples/invalid/invalid_escape.jaml"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_check_multiple_files() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg(example_path("examples/valid/simple.jaml"))
        .arg(example_path("examples/valid/nested.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("All 2 file(s) are valid"));
}

#[test]
fn test_check_stdin() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid JAML"));
}

#[test]
fn test_check_stdin_explicit() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg("-")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_check_quiet_success() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg("--quiet")
        .arg(example_path("examples/valid/basic.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_check_quiet_failure() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg("--quiet")
        .arg(example_path("examples/invalid/invalid_escape.jaml"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_check_verbose() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg("--verbose")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid JAML:"));
}

#[test]
fn test_completions_bash() {
    let mut cmd = jaml_cmd();
    cmd.arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("_jaml"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = jaml_cmd();
    cmd.arg("completions")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef jaml"));
}

#[test]
fn test_format_output_file() {
    let temp_file = "/tmp/jaml_test_output.jaml";

    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("-o")
        .arg(temp_file)
        .write_stdin("test: 123")
        .assert()
        .success();

    let content = fs::read_to_string(temp_file).unwrap();
    assert!(content.contains("test: 123"));

    fs::remove_file(temp_file).ok();
}

#[test]
fn test_format_output_stdout_explicit() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("-o")
        .arg("-")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_format_custom_options() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--quotes")
        .arg("single")
        .arg("--binary")
        .arg("hex")
        .arg("--quote-keys")
        .write_stdin(r#"test: "value"
data: b64"SGVsbG8=""#)
        .assert()
        .success()
        .stdout(predicate::str::contains("'test'"))
        .stdout(predicate::str::contains("hex\""));
}

#[test]
fn test_format_leading_plus() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--leading-plus")
        .write_stdin("number: 42\npi: 3.14")
        .assert()
        .success()
        .stdout(predicate::str::contains("+42"))
        .stdout(predicate::str::contains("+3.14"));
}

#[test]
fn test_format_no_zulu() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--no-zulu")
        .write_stdin("time: ts\"2024-01-15T12:30:45Z\"")
        .assert()
        .success()
        .stdout(predicate::str::contains("+00:00"));
}

#[test]
fn test_format_timestamp_precision() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--timestamp-precision")
        .arg("milliseconds")
        .write_stdin("time: ts\"2024-01-15T12:30:45.123456789Z\"")
        .assert()
        .success()
        .stdout(predicate::str::contains(".123Z"))
        .stdout(predicate::str::contains(".123456").not());
}

#[test]
fn test_invalid_jaml_parse_error() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .write_stdin("test: invalid syntax ][")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_help_message() {
    let mut cmd = jaml_cmd();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("JAML"))
        .stdout(predicate::str::contains("Usage: jaml"));
}

#[test]
fn test_version() {
    let mut cmd = jaml_cmd();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("jaml"));
}

#[test]
fn test_format_alias() {
    let mut cmd = jaml_cmd();
    cmd.arg("fmt")
        .write_stdin("test: 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("test: 123"));
}

#[test]
fn test_check_alias() {
    let mut cmd = jaml_cmd();
    cmd.arg("chk")
        .arg(example_path("examples/valid/basic.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_format_nonexistent_file() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("/nonexistent/file.jaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
}

#[test]
fn test_check_nonexistent_file() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg("/nonexistent/file.jaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
}

#[test]
fn test_format_sort_keys_default() {
    let mut cmd = jaml_cmd();
    let output = cmd
        .arg("format")
        .write_stdin("zebra: 1\napple: 2\nbanana: 3")
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    // Output should be sorted alphabetically (apple, banana, zebra)
    let output_str = String::from_utf8_lossy(&output.stdout);
    let apple_pos = output_str.find("apple").unwrap();
    let banana_pos = output_str.find("banana").unwrap();
    let zebra_pos = output_str.find("zebra").unwrap();
    
    assert!(apple_pos < banana_pos);
    assert!(banana_pos < zebra_pos);
}

#[test]
fn test_format_escape_unicode() {
    let mut cmd = jaml_cmd();
    cmd.arg("format")
        .arg("--escape-unicode")
        .write_stdin("emoji: \"🎉\"")
        .assert()
        .success()
        .stdout(predicate::str::contains("\\u"));
}

#[test]
fn test_check_all_valid_examples() {
    let mut cmd = jaml_cmd();
    cmd.arg("check")
        .arg(example_path("examples/valid/basic.jaml"))
        .arg(example_path("examples/valid/simple.jaml"))
        .arg(example_path("examples/valid/nested.jaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("All 3 file(s) are valid"));
}
