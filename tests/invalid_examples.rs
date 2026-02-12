use std::{fs, path::Path};

use jasn::parse;

#[test]
fn test_all_invalid_examples() {
    let invalid_dir = Path::new("examples/invalid");

    let mut examples: Vec<_> = fs::read_dir(invalid_dir)
        .expect("Failed to read invalid examples directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "jasn" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    examples.sort();

    assert!(!examples.is_empty(), "No invalid example files found");

    for example in examples {
        let content = fs::read_to_string(&example)
            .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", example, e));

        assert!(
            parse(&content).is_err(),
            "Invalid example {:?} should have failed to parse but succeeded",
            example
        );
    }
}

#[test]
fn test_double_underscore() {
    assert!(parse("1__000").is_err());
}

#[test]
fn test_trailing_underscore() {
    assert!(parse("1000_").is_err());
}

#[test]
fn test_leading_underscore_number() {
    assert!(parse("_1000").is_err());
}

#[test]
fn test_unquoted_keyword_key() {
    assert!(parse("{null: 1}").is_err());
    assert!(parse("{true: 1}").is_err());
    assert!(parse("{false: 1}").is_err());
}

#[test]
fn test_invalid_escape() {
    assert!(parse(r#""\x""#).is_err());
}

#[test]
fn test_unterminated_string() {
    assert!(parse(r#""unterminated"#).is_err());
}

#[test]
fn test_missing_comma() {
    assert!(parse("[1, 2 3]").is_err());
}

#[test]
fn test_odd_hex_binary() {
    assert!(parse(r#"h"ABC""#).is_err());
}

#[test]
fn test_invalid_base64() {
    assert!(parse(r#"b64"Hello!""#).is_err());
}

#[test]
fn test_bare_decimal_point() {
    assert!(parse(".").is_err());
}

#[test]
fn test_unquoted_dash_key() {
    assert!(parse("{kebab-case: 1}").is_err());
}

#[test]
fn test_unterminated_comment() {
    assert!(parse("/* unterminated").is_err());
}

#[test]
fn test_integer_overflow() {
    // Beyond i64::MAX
    assert!(parse("9223372036854775808").is_err());
    // Below i64::MIN
    assert!(parse("-9223372036854775809").is_err());
}
