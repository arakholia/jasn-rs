use std::{fs, path::Path};

use jasn::parse;
use rstest::rstest;

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

#[rstest]
#[case("1__000")]
#[case("1000_")]
#[case("_1000")]
fn test_invalid_underscore_in_numbers(#[case] input: &str) {
    assert!(parse(input).is_err());
}

#[rstest]
#[case(r#"{a: 1, a: 2}"#)]
#[case(r#"{"key": 1, "key": 2}"#)]
#[case(r#"{null: 1, null: 2}"#)]
fn test_duplicate_keys(#[case] input: &str) {
    assert!(parse(input).is_err());
}

#[rstest]
#[case(r#""\x""#)]
#[case(r#""unterminated"#)]
#[case("[1, 2 3]")]
#[case(r#"h"ABC""#)]
#[case(r#"b64"Hello!""#)]
#[case(".")]
#[case("{kebab-case: 1}")]
#[case("/* unterminated")]
fn test_various_parse_errors(#[case] input: &str) {
    assert!(parse(input).is_err());
}

#[rstest]
// Beyond i64::MAX
#[case("9223372036854775808")]
// Below i64::MIN
#[case("-9223372036854775809")]
fn test_integer_overflow(#[case] input: &str) {
    assert!(parse(input).is_err());
}
