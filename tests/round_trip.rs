use std::fs;

use jasn::{parse, to_string, to_string_pretty};

#[test]
fn test_round_trip_valid_examples() {
    // Get all valid example files
    let valid_dir = "examples/valid";
    let entries = fs::read_dir(valid_dir).expect("Failed to read valid examples dir");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("jasn") {
            let filename = path.file_name().unwrap().to_str().unwrap();
            println!("Testing round-trip for: {}", filename);

            let content = fs::read_to_string(&path).expect("Failed to read file");

            // Parse the original
            let value = parse(&content).expect("Failed to parse original");

            // Format and re-parse (compact)
            let formatted = to_string(&value);
            let reparsed = parse(&formatted).expect("Failed to parse formatted (compact)");
            assert_values_equal(
                &value,
                &reparsed,
                &format!("compact format of {}", filename),
            );

            // Format and re-parse (pretty)
            let formatted_pretty = to_string_pretty(&value);
            let reparsed_pretty =
                parse(&formatted_pretty).expect("Failed to parse formatted (pretty)");
            assert_values_equal(
                &value,
                &reparsed_pretty,
                &format!("pretty format of {}", filename),
            );
        }
    }
}

// Helper to compare values, treating NaN as equal to NaN
fn assert_values_equal(left: &jasn::Value, right: &jasn::Value, context: &str) {
    use jasn::Value;

    match (left, right) {
        (Value::Float(l), Value::Float(r)) if l.is_nan() && r.is_nan() => {
            // Both NaN, consider them equal
        }
        (Value::List(l), Value::List(r)) => {
            assert_eq!(l.len(), r.len(), "List length mismatch in {}", context);
            for (i, (lv, rv)) in l.iter().zip(r.iter()).enumerate() {
                assert_values_equal(lv, rv, &format!("{}[{}]", context, i));
            }
        }
        (Value::Map(l), Value::Map(r)) => {
            assert_eq!(l.len(), r.len(), "Map length mismatch in {}", context);
            for (key, lv) in l.iter() {
                let rv = r
                    .get(key)
                    .expect(&format!("Missing key '{}' in {}", key, context));
                assert_values_equal(lv, rv, &format!("{}.{}", context, key));
            }
        }
        _ => {
            assert_eq!(left, right, "Value mismatch in {}", context);
        }
    }
}

#[test]
fn test_format_preserve_exact_values() {
    // Test that specific values format and parse exactly
    let test_cases = vec![
        (r#"null"#, "null"),
        (r#"true"#, "true"),
        (r#"false"#, "false"),
        (r#"42"#, "42"),
        (r#"-123"#, "-123"),
        (r#"0x10"#, "16"),   // Hex parses to decimal
        (r#"0b1010"#, "10"), // Binary parses to decimal
        (r#"inf"#, "inf"),
        (r#"-inf"#, "-inf"),
        (r#"3.0"#, "3.0"), // Float with .0
        (r#"[]"#, "[]"),
        (r#"{}"#, "{}"),
    ];

    for (input, expected_compact) in test_cases {
        let value = parse(input).expect("Parse failed");
        let formatted = to_string(&value);
        assert_eq!(
            formatted, expected_compact,
            "Format mismatch for input: {}",
            input
        );

        // Ensure round-trip
        let reparsed = parse(&formatted).expect("Reparse failed");
        assert_values_equal(&value, &reparsed, &format!("round-trip of {}", input));
    }

    // Special case for NaN (can't compare with ==)
    let nan_value = parse("nan").expect("Parse failed");
    let formatted_nan = to_string(&nan_value);
    assert_eq!(formatted_nan, "nan");
    let reparsed_nan = parse(&formatted_nan).expect("Reparse failed");
    assert!(
        matches!(reparsed_nan, jasn::Value::Float(f) if f.is_nan()),
        "NaN round-trip failed"
    );
}

#[test]
fn test_string_escaping() {
    // Test various string escapes
    let test_strings = vec![
        ("hello", r#""hello""#),
        ("hello\nworld", r#""hello\nworld""#),
        ("tab\there", r#""tab\there""#),
        ("quote\"test", r#""quote\"test""#),
        ("slash\\test", r#""slash\\test""#),
    ];

    for (original, expected) in test_strings {
        let value = parse(&format!(
            r#""{}""#,
            original
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\t", "\\t")
        ))
        .unwrap();
        let formatted = to_string(&value);
        assert_eq!(formatted, expected);
    }
}
