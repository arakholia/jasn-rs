use std::{fs, path::Path};

use jasn::parse;

#[test]
fn test_all_valid_examples() {
    let valid_dir = Path::new("examples/valid");

    let mut examples: Vec<_> = fs::read_dir(valid_dir)
        .expect("Failed to read valid examples directory")
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

    assert!(!examples.is_empty(), "No valid example files found");

    for example in examples {
        let content = fs::read_to_string(&example)
            .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", example, e));

        parse(&content)
            .unwrap_or_else(|e| panic!("Failed to parse valid example {:?}: {}", example, e));
    }
}

#[test]
fn test_minimal() {
    let result = parse("null").unwrap();
    assert!(matches!(result, jasn::Value::Null));
}

#[test]
fn test_integers() {
    assert!(matches!(parse("42").unwrap(), jasn::Value::Int(42)));
    assert!(matches!(parse("-123").unwrap(), jasn::Value::Int(-123)));
    assert!(matches!(parse("0xFF").unwrap(), jasn::Value::Int(255)));
    assert!(matches!(parse("0b1010").unwrap(), jasn::Value::Int(10)));
    assert!(matches!(parse("0o755").unwrap(), jasn::Value::Int(493)));
    assert!(matches!(
        parse("1_000_000").unwrap(),
        jasn::Value::Int(1_000_000)
    ));
}

#[test]
fn test_floats() {
    assert!(matches!(parse("3.14").unwrap(), jasn::Value::Float(_)));
    assert!(matches!(parse("1e10").unwrap(), jasn::Value::Float(_)));
    assert!(matches!(parse("inf").unwrap(), jasn::Value::Float(f) if f.is_infinite()));
    assert!(
        matches!(parse("-inf").unwrap(), jasn::Value::Float(f) if f.is_infinite() && f.is_sign_negative())
    );
    assert!(matches!(parse("nan").unwrap(), jasn::Value::Float(f) if f.is_nan()));
}

#[test]
fn test_strings() {
    assert!(matches!(parse(r#""hello""#).unwrap(), jasn::Value::String(s) if s == "hello"));
    assert!(matches!(parse(r#"'world'"#).unwrap(), jasn::Value::String(s) if s == "world"));
}

#[test]
fn test_binary() {
    let result = parse(r#"b64"SGVsbG8=""#).unwrap();
    assert!(matches!(result, jasn::Value::Binary(_)));

    let result = parse(r#"h"48656c6c6f""#).unwrap();
    assert!(matches!(result, jasn::Value::Binary(_)));
}

#[test]
fn test_lists() {
    let result = parse("[1, 2, 3]").unwrap();
    assert!(matches!(result, jasn::Value::List(ref v) if v.len() == 3));

    let result = parse("[1, 2, 3,]").unwrap();
    assert!(matches!(result, jasn::Value::List(ref v) if v.len() == 3));
}

#[test]
fn test_maps() {
    let result = parse(r#"{"key": "value"}"#).unwrap();
    assert!(matches!(result, jasn::Value::Map(ref m) if m.len() == 1));

    let result = parse(r#"{unquoted: "value"}"#).unwrap();
    assert!(matches!(result, jasn::Value::Map(ref m) if m.len() == 1));
}

#[test]
fn test_comments() {
    let result = parse("// comment\n42").unwrap();
    assert!(matches!(result, jasn::Value::Int(42)));

    let result = parse("/* block comment */ 42").unwrap();
    assert!(matches!(result, jasn::Value::Int(42)));
}
