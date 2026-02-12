use std::collections::BTreeMap;

use lazy_static::lazy_static;
use time::format_description;

use crate::{Binary, Value};

/// Formatting options and configuration.
pub mod options;
pub use options::Options;
use options::{BinaryEncoding, QuoteStyle, TimestampPrecision};

/// Formats a JASN value into a compact string (no unnecessary whitespace).
pub fn to_string(value: &Value) -> String {
    format_with_opts(value, &Options::compact(), 0)
}

/// Formats a JASN value into a pretty-printed string with indentation and newlines.
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
        Value::Timestamp(t) => format_timestamp(t, opts),
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

lazy_static! {
    static ref TIMESTAMP_FORMAT_SECONDS: Vec<format_description::FormatItem<'static>> = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
        ).unwrap();
}

lazy_static! {
    static ref TIMESTAMP_FORMAT_MILLIS: Vec<format_description::FormatItem<'static>> = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]"
        ).unwrap();
}

lazy_static! {
    static ref TIMESTAMP_FORMAT_MICROS: Vec<format_description::FormatItem<'static>> = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]"
        ).unwrap();
}

lazy_static! {
    static ref TIMESTAMP_FORMAT_NANOS: Vec<format_description::FormatItem<'static>> = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:9][offset_hour sign:mandatory]:[offset_minute]"
        ).unwrap();
}

fn format_timestamp(t: &crate::Timestamp, opts: &Options) -> String {
    // Select format descriptor based on precision
    let format: &[format_description::FormatItem<'_>] = match opts.timestamp_precision {
        TimestampPrecision::Auto => {
            // Use RFC3339 which includes fractional seconds when present
            let formatted = t
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| t.to_string());

            // RFC3339 uses Z for UTC, convert to +00:00 if needed
            let final_str = if !opts.use_zulu && formatted.ends_with('Z') {
                let mut s = formatted;
                s.pop();
                s.push_str("+00:00");
                s
            } else {
                formatted
            };
            return format!("ts\"{}\"", final_str);
        }
        TimestampPrecision::Seconds => &TIMESTAMP_FORMAT_SECONDS,
        TimestampPrecision::Milliseconds => &TIMESTAMP_FORMAT_MILLIS,
        TimestampPrecision::Microseconds => &TIMESTAMP_FORMAT_MICROS,
        TimestampPrecision::Nanoseconds => &TIMESTAMP_FORMAT_NANOS,
    };

    // Custom formats output +00:00, convert to Z if needed
    let formatted = t.format(format).unwrap_or_else(|_| t.to_string());
    let final_str = if opts.use_zulu && formatted.ends_with("+00:00") {
        let mut s = formatted;
        s.truncate(s.len() - 6);
        s.push('Z');
        s
    } else {
        formatted
    };

    format!("ts\"{}\"", final_str)
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
    use rstest::rstest;

    use super::*;
    use crate::parse;

    #[rstest]
    #[case(Value::Null, "null")]
    #[case(Value::Bool(true), "true")]
    #[case(Value::Bool(false), "false")]
    #[case(Value::Int(42), "42")]
    #[case(Value::Int(-123), "-123")]
    fn test_format_primitives(#[case] value: Value, #[case] expected: &str) {
        assert_eq!(to_string(&value), expected);
    }

    #[rstest]
    #[case(3.0, "3.0")]
    #[case(2.5, "2.5")]
    #[case(f64::INFINITY, "inf")]
    #[case(f64::NEG_INFINITY, "-inf")]
    fn test_format_float(#[case] value: f64, #[case] expected: &str) {
        assert_eq!(to_string(&Value::Float(value)), expected);
    }

    #[test]
    fn test_format_float_nan() {
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

    #[rstest]
    #[case("hello", true)]
    #[case("_private", true)]
    #[case("key123", true)]
    #[case("_", true)]
    #[case("", false)]
    #[case("123", false)]
    #[case("null", false)]
    #[case("true", false)]
    #[case("false", false)]
    #[case("kebab-case", false)]
    fn test_can_be_unquoted(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(can_be_unquoted(input), expected);
    }

    #[rstest]
    #[case(Value::Int(42), "+42")]
    #[case(Value::Int(0), "+0")]
    #[case(Value::Int(-42), "-42")]
    #[case(Value::Float(2.5), "+2.5")]
    #[case(Value::Float(-2.5), "-2.5")]
    #[case(Value::Float(f64::INFINITY), "+inf")]
    #[case(Value::Float(f64::NEG_INFINITY), "-inf")]
    #[case(Value::Float(f64::NAN), "nan")]
    fn test_leading_plus(#[case] value: Value, #[case] expected: &str) {
        let opts = Options::compact().with_leading_plus(true);
        assert_eq!(to_string_opts(&value, &opts), expected);
    }

    #[rstest]
    #[case(Value::Int(42), "42")]
    #[case(Value::Float(2.5), "2.5")]
    fn test_no_leading_plus(#[case] value: Value, #[case] expected: &str) {
        let opts = Options::compact();
        assert_eq!(to_string_opts(&value, &opts), expected);
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

    #[rstest]
    #[case("😀", "\"\\ud83d\\ude00\"")]
    #[case("👍", "\"\\ud83d\\udc4d\"")]
    #[case("𝄞", "\"\\ud834\\udd1e\"")]
    #[case("😀😁😂", "\"\\ud83d\\ude00\\ud83d\\ude01\\ud83d\\ude02\"")]
    #[case("Hello 😀 World", "\"Hello \\ud83d\\ude00 World\"")]
    #[case("中文", "\"\\u4e2d\\u6587\"")]
    fn test_surrogate_pair_encoding(#[case] input: &str, #[case] expected: &str) {
        let opts = Options::compact().with_escape_unicode(true);
        let value = Value::String(input.to_string());
        assert_eq!(to_string_opts(&value, &opts), expected);
    }

    #[rstest]
    #[case("😀")]
    #[case("🌍")]
    #[case("👍")]
    #[case("𝄞")]
    #[case("Hello 😀 World")]
    #[case("😀😁😂")]
    fn test_surrogate_pair_round_trip(#[case] original: &str) {
        let opts = Options::compact().with_escape_unicode(true);
        let value = Value::String(original.to_string());
        let formatted = to_string_opts(&value, &opts);
        let parsed = crate::parse(&formatted).expect("Failed to parse");

        if let Value::String(s) = parsed {
            assert_eq!(s, original, "Round-trip failed for: {}", original);
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_format_timestamp_default() {
        use crate::Timestamp;

        let ts = Timestamp::from_unix_timestamp(1234567890).unwrap();
        let value = Value::Timestamp(ts);

        // Default (use_zulu = true) - should use Z notation
        let result = to_string(&value);
        assert_eq!(result, "ts\"2009-02-13T23:31:30Z\"");
    }

    #[rstest]
    #[case(true, "ts\"2009-02-13T23:31:30Z\"")]
    #[case(false, "ts\"2009-02-13T23:31:30+00:00\"")]
    fn test_format_timestamp_zulu(#[case] use_zulu: bool, #[case] expected: &str) {
        use crate::Timestamp;

        let ts = Timestamp::from_unix_timestamp(1234567890).unwrap();
        let value = Value::Timestamp(ts);
        let opts = Options::compact().with_use_zulu(use_zulu);
        let result = to_string_opts(&value, &opts);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(true, "ts\"2009-02-13T23:31:30.123456789Z\"")]
    #[case(false, "ts\"2009-02-13T23:31:30.123456789+00:00\"")]
    fn test_format_timestamp_fractional_zulu(#[case] use_zulu: bool, #[case] expected: &str) {
        use crate::Timestamp;

        let ts = Timestamp::from_unix_timestamp_nanos(1234567890123456789).unwrap();
        let value = Value::Timestamp(ts);
        let opts = Options::compact().with_use_zulu(use_zulu);
        let result = to_string_opts(&value, &opts);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TimestampPrecision::Auto, "ts\"2009-02-13T23:31:30.123456789Z\"")]
    #[case(TimestampPrecision::Seconds, "ts\"2009-02-13T23:31:30Z\"")]
    #[case(TimestampPrecision::Milliseconds, "ts\"2009-02-13T23:31:30.123Z\"")]
    #[case(TimestampPrecision::Microseconds, "ts\"2009-02-13T23:31:30.123456Z\"")]
    #[case(
        TimestampPrecision::Nanoseconds,
        "ts\"2009-02-13T23:31:30.123456789Z\""
    )]
    fn test_format_timestamp_precision(
        #[case] precision: TimestampPrecision,
        #[case] expected: &str,
    ) {
        use crate::Timestamp;

        let ts = Timestamp::from_unix_timestamp_nanos(1234567890123456789).unwrap();
        let value = Value::Timestamp(ts);
        let opts = Options::compact().with_timestamp_precision(precision);
        let result = to_string_opts(&value, &opts);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_timestamp_precision_with_offset() {
        use crate::Timestamp;

        let ts = Timestamp::from_unix_timestamp_nanos(1234567890123456789).unwrap();
        let value = Value::Timestamp(ts);
        let opts = Options::compact()
            .with_timestamp_precision(TimestampPrecision::Milliseconds)
            .with_use_zulu(false);
        let result = to_string_opts(&value, &opts);
        assert_eq!(result, "ts\"2009-02-13T23:31:30.123+00:00\"");
    }

    #[test]
    fn test_format_timestamp_precision_padding() {
        use crate::Timestamp;

        // Test precision padding (timestamp without fractional seconds)
        // When formatted with higher precision, should add zeros
        let ts = Timestamp::from_unix_timestamp(1234567890).unwrap();
        let value = Value::Timestamp(ts);
        let opts = Options::compact().with_timestamp_precision(TimestampPrecision::Milliseconds);
        let result = to_string_opts(&value, &opts);
        assert_eq!(result, "ts\"2009-02-13T23:31:30.000Z\"");
    }
}
