use std::collections::BTreeMap;

use crate::{Binary, Value};

pub mod options;
pub use options::Options;
use options::{BinaryEncoding, QuoteStyle};

pub fn to_string(value: &Value) -> String {
    format_with_opts(value, &Options::compact(), 0)
}

pub fn to_string_pretty(value: &Value) -> String {
    format_with_opts(value, &Options::pretty(), 0)
}

/// Formats a JASN value with custom formatting options.
pub fn to_string_opts(value: &Value, opts: &Options) -> String {
    format_with_opts(value, opts, 0)
}

fn format_with_opts(value: &Value, opts: &Options, depth: usize) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => format_int(*i, opts),
        Value::Float(f) => format_float(*f, opts),
        Value::String(s) => {
            let quote = match opts.quote_style {
                QuoteStyle::Double => '"',
                QuoteStyle::Single => '\'',
                QuoteStyle::PreferDouble => {
                    if s.contains('"') && !s.contains('\'') {
                        '\''
                    } else {
                        '"'
                    }
                }
            };
            format_string(s, quote, opts.escape_unicode)
        }
        Value::Binary(b) => format_binary(b, opts.binary_encoding),
        Value::List(items) => {
            if opts.indent.is_empty() {
                format_list_compact(items, opts)
            } else {
                format_list_pretty(items, opts, depth)
            }
        }
        Value::Map(map) => {
            if opts.indent.is_empty() {
                format_map_compact(map, opts)
            } else {
                format_map_pretty(map, opts, depth)
            }
        }
    }
}

fn format_int(i: i64, opts: &Options) -> String {
    if opts.leading_plus && i >= 0 {
        format!("+{}", i)
    } else {
        i.to_string()
    }
}

fn format_float(f: f64, opts: &Options) -> String {
    let base_string = if f.is_infinite() {
        if f.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        }
    } else if f.is_nan() {
        "nan".to_string()
    } else if f.fract() == 0.0 && f.abs() < 1e15 {
        // Ensure we always have a decimal point to distinguish from integers
        format!("{:.1}", f)
    } else {
        f.to_string()
    };

    // Add leading plus for positive numbers (including +inf, but not nan)
    if opts.leading_plus && !f.is_nan() && !base_string.starts_with('-') {
        format!("+{}", base_string)
    } else {
        base_string
    }
}

fn format_string(s: &str, quote: char, escape_unicode: bool) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    result.push(quote);

    for ch in s.chars() {
        match ch {
            '"' if quote == '"' => result.push_str("\\\""),
            '\'' if quote == '\'' => result.push_str("\\'"),
            '\\' => result.push_str("\\\\"),
            '/' => result.push_str("\\/"),
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            c if c.is_control() => {
                use std::fmt::Write;
                write!(&mut result, "\\u{:04x}", c as u32).unwrap();
            }
            c if escape_unicode && !c.is_ascii() => {
                use std::fmt::Write;
                let code = c as u32;
                if code <= 0xFFFF {
                    // BMP character - single escape sequence
                    write!(&mut result, "\\u{:04x}", code).unwrap();
                } else {
                    // Non-BMP character - use UTF-16 surrogate pair
                    let adjusted = code - 0x10000;
                    let high = 0xD800 + (adjusted >> 10);
                    let low = 0xDC00 + (adjusted & 0x3FF);
                    write!(&mut result, "\\u{:04x}\\u{:04x}", high, low).unwrap();
                }
            }
            c => result.push(c),
        }
    }

    result.push(quote);
    result
}

fn format_binary(binary: &Binary, encoding: BinaryEncoding) -> String {
    match encoding {
        BinaryEncoding::Base64 => {
            use base64::{Engine as _, engine::general_purpose};
            let encoded = general_purpose::STANDARD.encode(&binary.0);
            format!("b64\"{}\"", encoded)
        }
        BinaryEncoding::Hex => {
            let hex: String = binary.0.iter().map(|b| format!("{:02x}", b)).collect();
            format!("h\"{}\"", hex)
        }
    }
}

fn format_list_compact(items: &[Value], opts: &Options) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }

    let formatted: Vec<String> = items
        .iter()
        .map(|item| format_with_opts(item, opts, 0))
        .collect();
    format!("[{}]", formatted.join(","))
}

fn format_list_pretty(items: &[Value], opts: &Options, depth: usize) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }

    let indent = opts.indent.repeat(depth);
    let item_indent = opts.indent.repeat(depth + 1);
    let mut result = String::from("[\n");

    for (i, item) in items.iter().enumerate() {
        result.push_str(&item_indent);
        result.push_str(&format_with_opts(item, opts, depth + 1));
        if i < items.len() - 1 || opts.trailing_commas {
            result.push(',');
        }
        result.push('\n');
    }

    result.push_str(&indent);
    result.push(']');
    result
}

fn format_map_compact(map: &BTreeMap<String, Value>, opts: &Options) -> String {
    if map.is_empty() {
        return "{}".to_string();
    }

    let entries: Vec<_> = if opts.sort_keys {
        let mut sorted: Vec<_> = map.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        sorted
    } else {
        map.iter().collect()
    };

    let formatted: Vec<String> = entries
        .iter()
        .map(|(k, v)| {
            let key_str = if opts.unquoted_keys && can_be_unquoted(k) {
                k.to_string()
            } else {
                let quote = match opts.quote_style {
                    QuoteStyle::Double => '"',
                    QuoteStyle::Single => '\'',
                    QuoteStyle::PreferDouble => {
                        if k.contains('"') && !k.contains('\'') {
                            '\''
                        } else {
                            '"'
                        }
                    }
                };
                format_string(k, quote, opts.escape_unicode)
            };
            format!("{}:{}", key_str, format_with_opts(v, opts, 0))
        })
        .collect();
    format!("{{{}}}", formatted.join(","))
}

fn format_map_pretty(map: &BTreeMap<String, Value>, opts: &Options, depth: usize) -> String {
    if map.is_empty() {
        return "{}".to_string();
    }

    let indent = opts.indent.repeat(depth);
    let item_indent = opts.indent.repeat(depth + 1);
    let mut result = String::from("{\n");

    let entries: Vec<_> = if opts.sort_keys {
        let mut sorted: Vec<_> = map.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        sorted
    } else {
        map.iter().collect()
    };
    for (i, (key, value)) in entries.iter().enumerate() {
        result.push_str(&item_indent);

        // Format key (possibly unquoted)
        if opts.unquoted_keys && can_be_unquoted(key) {
            result.push_str(key);
        } else {
            let quote = match opts.quote_style {
                QuoteStyle::Double => '"',
                QuoteStyle::Single => '\'',
                QuoteStyle::PreferDouble => {
                    if key.contains('"') && !key.contains('\'') {
                        '\''
                    } else {
                        '"'
                    }
                }
            };
            result.push_str(&format_string(key, quote, opts.escape_unicode));
        }

        result.push_str(": ");
        result.push_str(&format_with_opts(value, opts, depth + 1));

        if i < entries.len() - 1 || opts.trailing_commas {
            result.push(',');
        }
        result.push('\n');
    }

    result.push_str(&indent);
    result.push('}');
    result
}

fn can_be_unquoted(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    // Reserved keywords cannot be unquoted
    if matches!(key, "null" | "true" | "false" | "inf" | "nan") {
        return false;
    }

    let mut chars = key.chars();
    let first = chars.next().unwrap();

    // Must start with letter or underscore
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_format_primitives() {
        assert_eq!(to_string(&Value::Null), "null");
        assert_eq!(to_string(&Value::Bool(true)), "true");
        assert_eq!(to_string(&Value::Bool(false)), "false");
        assert_eq!(to_string(&Value::Int(42)), "42");
        assert_eq!(to_string(&Value::Int(-123)), "-123");
    }

    #[test]
    fn test_format_float() {
        assert_eq!(to_string(&Value::Float(3.0)), "3.0");
        assert_eq!(to_string(&Value::Float(2.5)), "2.5");
        assert_eq!(to_string(&Value::Float(f64::INFINITY)), "inf");
        assert_eq!(to_string(&Value::Float(f64::NEG_INFINITY)), "-inf");
        assert!(to_string(&Value::Float(f64::NAN)).contains("nan"));
    }

    #[test]
    fn test_format_string() {
        assert_eq!(to_string(&Value::String("hello".to_string())), "\"hello\"");
        assert_eq!(
            to_string(&Value::String("hello\nworld".to_string())),
            "\"hello\\nworld\""
        );
        assert_eq!(
            to_string(&Value::String("tab\there".to_string())),
            "\"tab\\there\""
        );
    }

    #[test]
    fn test_format_binary() {
        let binary = Binary(vec![72, 101, 108, 108, 111]); // "Hello"
        assert_eq!(to_string(&Value::Binary(binary)), "b64\"SGVsbG8=\"");
    }

    #[test]
    fn test_format_list() {
        let list = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
        assert_eq!(to_string(&Value::List(list)), "[1,2,3]");

        assert_eq!(to_string(&Value::List(vec![])), "[]");
    }

    #[test]
    fn test_format_map() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Int(30));

        let formatted = to_string(&Value::Map(map));
        // Compact format uses unquoted keys to save bytes
        assert!(formatted.contains("age:30"));
        assert!(formatted.contains("name:\"Alice\""));
    }

    #[test]
    fn test_round_trip() {
        // Null
        let null = Value::Null;
        assert_eq!(parse(&to_string(&null)).unwrap(), null);

        // Bool
        let bool_val = Value::Bool(true);
        assert_eq!(parse(&to_string(&bool_val)).unwrap(), bool_val);

        // Int
        let int_val = Value::Int(42);
        assert_eq!(parse(&to_string(&int_val)).unwrap(), int_val);

        // Float
        let float_val = Value::Float(2.5);
        assert_eq!(parse(&to_string(&float_val)).unwrap(), float_val);

        // String
        let string_val = Value::String("hello world".to_string());
        assert_eq!(parse(&to_string(&string_val)).unwrap(), string_val);

        // List
        let list_val = Value::List(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(parse(&to_string(&list_val)).unwrap(), list_val);

        // Map
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Value::Int(42));
        let map_val = Value::Map(map);
        assert_eq!(parse(&to_string(&map_val)).unwrap(), map_val);
    }

    #[test]
    fn test_pretty_format() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Int(30));

        let pretty = to_string_pretty(&Value::Map(map));
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  "));
    }

    #[test]
    fn test_can_be_unquoted() {
        assert!(can_be_unquoted("hello"));
        assert!(can_be_unquoted("_private"));
        assert!(can_be_unquoted("key123"));
        assert!(can_be_unquoted("_"));

        assert!(!can_be_unquoted(""));
        assert!(!can_be_unquoted("123"));
        assert!(!can_be_unquoted("null"));
        assert!(!can_be_unquoted("true"));
        assert!(!can_be_unquoted("false"));
        assert!(!can_be_unquoted("kebab-case"));
    }

    #[test]
    fn test_leading_plus() {
        let opts = Options::compact().with_leading_plus(true);

        // Positive integers get a plus sign
        assert_eq!(to_string_opts(&Value::Int(42), &opts), "+42");

        // Zero gets a plus sign
        assert_eq!(to_string_opts(&Value::Int(0), &opts), "+0");

        // Negative integers keep their minus sign
        assert_eq!(to_string_opts(&Value::Int(-42), &opts), "-42");

        // Positive floats get a plus sign
        assert_eq!(to_string_opts(&Value::Float(2.5), &opts), "+2.5");

        // Negative floats keep their minus sign
        assert_eq!(to_string_opts(&Value::Float(-2.5), &opts), "-2.5");

        // Positive infinity gets a plus sign
        assert_eq!(to_string_opts(&Value::Float(f64::INFINITY), &opts), "+inf");

        // Negative infinity keeps its minus
        assert_eq!(
            to_string_opts(&Value::Float(f64::NEG_INFINITY), &opts),
            "-inf"
        );

        // NaN doesn't get a plus sign
        assert_eq!(to_string_opts(&Value::Float(f64::NAN), &opts), "nan");

        // Default (no leading_plus) should not add plus signs
        let default_opts = Options::compact();
        assert_eq!(to_string_opts(&Value::Int(42), &default_opts), "42");
        assert_eq!(to_string_opts(&Value::Float(2.5), &default_opts), "2.5");
    }

    #[test]
    fn test_sort_keys() {
        let mut map = BTreeMap::new();
        map.insert("zebra".to_string(), Value::Int(1));
        map.insert("apple".to_string(), Value::Int(2));
        map.insert("banana".to_string(), Value::Int(3));

        // With sort_keys enabled
        let sorted_opts = Options::compact().with_sort_keys(true);
        let sorted = to_string_opts(&Value::Map(map), &sorted_opts);

        // Should be alphabetically ordered
        assert_eq!(sorted, "{apple:2,banana:3,zebra:1}");

        // Pretty mode with sort_keys
        let pretty_sorted = Options::pretty().with_sort_keys(true);
        let mut map2 = BTreeMap::new();
        map2.insert("z".to_string(), Value::Int(1));
        map2.insert("a".to_string(), Value::Int(2));
        let result = to_string_opts(&Value::Map(map2), &pretty_sorted);
        assert!(result.find("a").unwrap() < result.find("z").unwrap());
    }

    #[test]
    fn test_escape_unicode() {
        let opts = Options::compact().with_escape_unicode(true);

        // ASCII characters should not be escaped
        let ascii = Value::String("hello".to_string());
        assert_eq!(to_string_opts(&ascii, &opts), "\"hello\"");

        // Non-ASCII characters should be escaped
        let unicode = Value::String("café".to_string());
        assert_eq!(to_string_opts(&unicode, &opts), "\"caf\\u00e9\"");

        // Emoji should be escaped using UTF-16 surrogate pairs (U+1F30D => D83C DF0D)
        let emoji = Value::String("Hello 🌍".to_string());
        assert_eq!(to_string_opts(&emoji, &opts), "\"Hello \\ud83c\\udf0d\"");

        // Chinese characters
        let chinese = Value::String("你好".to_string());
        assert_eq!(to_string_opts(&chinese, &opts), "\"\\u4f60\\u597d\"");

        // Without escape_unicode should keep Unicode literal
        let no_escape = Options::compact().with_escape_unicode(false);
        let result = to_string_opts(&unicode, &no_escape);
        assert_eq!(result, "\"café\"");
    }

    #[test]
    fn test_surrogate_pair_encoding() {
        let opts = Options::compact().with_escape_unicode(true);

        // Various emoji that require surrogate pairs
        let grinning = Value::String("😀".to_string());
        assert_eq!(to_string_opts(&grinning, &opts), "\"\\ud83d\\ude00\"");

        let thumbs_up = Value::String("👍".to_string());
        assert_eq!(to_string_opts(&thumbs_up, &opts), "\"\\ud83d\\udc4d\"");

        // Musical note (U+1D11E)
        let music = Value::String("𝄞".to_string());
        assert_eq!(to_string_opts(&music, &opts), "\"\\ud834\\udd1e\"");

        // Multiple emoji
        let multiple = Value::String("😀😁😂".to_string());
        assert_eq!(
            to_string_opts(&multiple, &opts),
            "\"\\ud83d\\ude00\\ud83d\\ude01\\ud83d\\ude02\""
        );

        // Mixed ASCII and emoji
        let mixed = Value::String("Hello 😀 World".to_string());
        assert_eq!(
            to_string_opts(&mixed, &opts),
            "\"Hello \\ud83d\\ude00 World\""
        );

        // BMP characters should not use surrogate pairs
        let bmp = Value::String("中文".to_string());
        assert_eq!(to_string_opts(&bmp, &opts), "\"\\u4e2d\\u6587\"");
    }

    #[test]
    fn test_surrogate_pair_round_trip() {
        // Test that encoding and parsing gives us back the original
        let opts = Options::compact().with_escape_unicode(true);

        let test_cases = vec!["😀", "🌍", "👍", "𝄞", "Hello 😀 World", "😀😁😂"];

        for original in test_cases {
            let value = Value::String(original.to_string());
            let formatted = to_string_opts(&value, &opts);
            let parsed = crate::parse(&formatted).expect("Failed to parse");

            if let Value::String(s) = parsed {
                assert_eq!(s, original, "Round-trip failed for: {}", original);
            } else {
                panic!("Expected String value");
            }
        }
    }
}
