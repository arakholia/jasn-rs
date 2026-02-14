//! Format a [`Value`] into JAML text.
//!
//! The main entry point is [`format()`] for standard use cases.
//! For custom formatting options, use [`format_with_opts()`] with [`Options`].
//!
//! ```
//! use jaml::{Value, format};
//!
//! let value = Value::String("hello".to_string());
//! assert_eq!(format(&value), "\"hello\"");
//!
//! // Custom formatting with advanced options
//! use jaml::formatter::{Options, format_with_opts};
//! let opts = Options::new().with_sort_keys(false);
//! let formatted = format_with_opts(&value, &opts);
//! ```

use std::collections::BTreeMap;

use time::{format_description, macros::format_description as fd};

use crate::{Binary, Value};

/// Formatting options and configuration.
mod options;
pub use options::{BinaryEncoding, Options, QuoteStyle, TimestampPrecision};

/// Formats a JAML [`Value`] into an indentation-based string.
///
/// JAML format is inherently indentation-based (like YAML), so there's no compact vs pretty distinction.
/// All output uses 2-space indentation by design.
pub fn format(value: &Value) -> String {
    format_impl(value, &Options::default(), 0, false)
}

/// Formats a JAML [`Value`] with custom formatting options.
pub fn format_with_opts(value: &Value, opts: &Options) -> String {
    format_impl(value, opts, 0, false)
}

fn format_impl(value: &Value, opts: &Options, depth: usize, inline: bool) -> String {
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
        Value::List(items) => format_list(items, opts, depth, inline),
        Value::Map(map) => format_map(map, opts, depth, inline),
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

const TIMESTAMP_FORMAT_SECONDS: &[format_description::FormatItem<'static>] = fd!(
    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
);

const TIMESTAMP_FORMAT_MILLIS: &[format_description::FormatItem<'static>] = fd!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]"
);

const TIMESTAMP_FORMAT_MICROS: &[format_description::FormatItem<'static>] = fd!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]"
);

const TIMESTAMP_FORMAT_NANOS: &[format_description::FormatItem<'static>] = fd!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:9][offset_hour sign:mandatory]:[offset_minute]"
);

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
        TimestampPrecision::Seconds => TIMESTAMP_FORMAT_SECONDS,
        TimestampPrecision::Milliseconds => TIMESTAMP_FORMAT_MILLIS,
        TimestampPrecision::Microseconds => TIMESTAMP_FORMAT_MICROS,
        TimestampPrecision::Nanoseconds => TIMESTAMP_FORMAT_NANOS,
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
            format!("hex\"{}\"", hex)
        }
    }
}

fn format_list(items: &[Value], opts: &Options, depth: usize, inline: bool) -> String {
    if items.is_empty() {
        // Empty list can't be represented in JAML block syntax without context
        // This would typically appear as an empty sequence in a parent structure
        return String::new();
    }

    let indent = "  ".repeat(depth);
    let mut result = String::new();

    for (i, item) in items.iter().enumerate() {
        if i > 0 || !inline {
            result.push_str(&indent);
        }
        result.push_str("- ");

        // Check if the item can be written inline or needs nesting
        match item {
            Value::List(_) | Value::Map(_) => {
                // Nested structures need to go on the next indented line
                result.push('\n');
                result.push_str(&format_impl(item, opts, depth + 1, false));
            }
            _ => {
                // Primitive values can go inline after the dash
                result.push_str(&format_impl(item, opts, depth + 1, true));
                result.push('\n');
            }
        }
    }

    result
}

fn format_map(map: &BTreeMap<String, Value>, opts: &Options, depth: usize, inline: bool) -> String {
    if map.is_empty() {
        // Empty map can't be represented in JAML block syntax without context
        return String::new();
    }

    let indent = "  ".repeat(depth);
    let mut result = String::new();

    let entries: Vec<_> = if opts.sort_keys {
        let mut sorted: Vec<_> = map.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        sorted
    } else {
        map.iter().collect()
    };

    for (i, (key, value)) in entries.iter().enumerate() {
        if i > 0 || !inline {
            result.push_str(&indent);
        }

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

        result.push(':');

        // Check if the value can be written inline or needs nesting
        match value {
            Value::List(_) | Value::Map(_) => {
                // Nested structures need to go on the next indented line
                result.push('\n');
                result.push_str(&format_impl(value, opts, depth + 1, false));
            }
            _ => {
                // Primitive values can go inline after the colon
                result.push(' ');
                result.push_str(&format_impl(value, opts, depth + 1, true));
                result.push('\n');
            }
        }
    }

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
