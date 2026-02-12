use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JasnParser;

use std::collections::BTreeMap;

use crate::{Binary, Value};

pub type ParseResult<T> = Result<T, String>;

/// Parse a JASN string into a Value
pub fn parse(input: &str) -> ParseResult<Value> {
    let mut pairs = JasnParser::parse(Rule::jasn, input).map_err(|e| e.to_string())?;
    let pair = pairs.next().unwrap(); // jasn rule
    let inner = pair.into_inner().next().unwrap(); // value rule
    parse_value(inner)
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let rule = if pair.as_rule() == Rule::value {
        // value is a wrapper, get the actual inner rule
        pair.into_inner().next().unwrap()
    } else {
        pair
    };

    match rule.as_rule() {
        Rule::null => Ok(Value::Null),
        Rule::boolean => Ok(Value::Bool(rule.as_str() == "true")),
        Rule::integer => parse_integer(rule),
        Rule::float => parse_float(rule),
        Rule::string => parse_string(rule),
        Rule::binary => parse_binary(rule),
        Rule::list => parse_list(rule),
        Rule::map => parse_map(rule),
        _ => unreachable!("Unexpected rule: {:?}", rule.as_rule()),
    }
}

fn parse_integer(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let s = pair.as_str();

    // Remove underscores
    let cleaned = s.replace('_', "");

    // Parse based on prefix
    let value = if cleaned.starts_with('+') || cleaned.starts_with('-') {
        let sign = &cleaned[0..1];
        let rest = &cleaned[1..];

        if rest.starts_with("0x") || rest.starts_with("0X") {
            let num = i64::from_str_radix(&rest[2..], 16)
                .map_err(|e| format!("Integer overflow: {}", e))?;
            if sign == "-" { -num } else { num }
        } else if rest.starts_with("0b") || rest.starts_with("0B") {
            let num = i64::from_str_radix(&rest[2..], 2)
                .map_err(|e| format!("Integer overflow: {}", e))?;
            if sign == "-" { -num } else { num }
        } else if rest.starts_with("0o") || rest.starts_with("0O") {
            let num = i64::from_str_radix(&rest[2..], 8)
                .map_err(|e| format!("Integer overflow: {}", e))?;
            if sign == "-" { -num } else { num }
        } else {
            cleaned
                .parse::<i64>()
                .map_err(|e| format!("Integer overflow: {}", e))?
        }
    } else if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
        i64::from_str_radix(&cleaned[2..], 16).map_err(|e| format!("Integer overflow: {}", e))?
    } else if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
        i64::from_str_radix(&cleaned[2..], 2).map_err(|e| format!("Integer overflow: {}", e))?
    } else if cleaned.starts_with("0o") || cleaned.starts_with("0O") {
        i64::from_str_radix(&cleaned[2..], 8).map_err(|e| format!("Integer overflow: {}", e))?
    } else {
        cleaned
            .parse::<i64>()
            .map_err(|e| format!("Integer overflow: {}", e))?
    };

    Ok(Value::Int(value))
}

fn parse_float(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let s = pair.as_str();

    // Handle special values
    let value = match s.to_lowercase().as_str() {
        "inf" | "+inf" => f64::INFINITY,
        "-inf" => f64::NEG_INFINITY,
        "nan" | "+nan" | "-nan" => f64::NAN,
        _ => s
            .parse::<f64>()
            .map_err(|e| format!("Float parse error: {}", e))?,
    };

    Ok(Value::Float(value))
}

fn parse_string(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    // The string rule contains the entire string with quotes due to $
    // We need to get the inner content
    let mut inner = pair.into_inner();
    let quoted = inner.next().unwrap(); // double_quoted_string or single_quoted_string
    let content_pair = quoted.into_inner().next().unwrap(); // The actual content
    let content = content_pair.as_str();

    // Process escape sequences
    let mut result = String::new();
    let mut chars = content.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('b') => result.push('\u{0008}'),
                Some('f') => result.push('\u{000C}'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    let code =
                        u32::from_str_radix(&hex, 16).map_err(|_| "Invalid unicode escape")?;
                    let ch = char::from_u32(code).ok_or("Invalid unicode codepoint")?;
                    result.push(ch);
                }
                _ => return Err("Invalid escape sequence".into()),
            }
        } else {
            result.push(ch);
        }
    }

    Ok(Value::String(result))
}

fn parse_binary(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let s = pair.as_str();

    let bytes = if s.starts_with("b64\"") {
        // Base64 encoding
        let content = &s[4..s.len() - 1]; // Remove b64" and "
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, content)
            .map_err(|e| format!("Invalid base64: {}", e))?
    } else if s.starts_with("h\"") {
        // Hex encoding
        let content = &s[2..s.len() - 1]; // Remove h" and "

        if content.len() % 2 != 0 {
            return Err("Hex binary must have even number of digits".into());
        }

        (0..content.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&content[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| format!("Invalid hex: {}", e))?
    } else {
        return Err("Unknown binary encoding".into());
    };

    Ok(Value::Binary(Binary(bytes)))
}

fn parse_list(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let values = pair
        .into_inner()
        .map(parse_value)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Value::List(values))
}

fn parse_map(pair: pest::iterators::Pair<Rule>) -> ParseResult<Value> {
    let mut map = BTreeMap::new();

    for member in pair.into_inner() {
        let mut inner = member.into_inner();
        let key_pair = inner.next().unwrap();
        let value_pair = inner.next().unwrap();

        let key = match key_pair.as_rule() {
            Rule::key => {
                // key is a wrapper rule, extract the actual string or identifier
                let actual_key = key_pair.into_inner().next().unwrap();
                match actual_key.as_rule() {
                    Rule::string => {
                        if let Value::String(s) = parse_string(actual_key)? {
                            s
                        } else {
                            unreachable!()
                        }
                    }
                    Rule::identifier => actual_key.as_str().to_string(),
                    _ => unreachable!("Unexpected key rule: {:?}", actual_key.as_rule()),
                }
            }
            Rule::string => {
                if let Value::String(s) = parse_string(key_pair)? {
                    s
                } else {
                    unreachable!()
                }
            }
            Rule::identifier => key_pair.as_str().to_string(),
            _ => unreachable!("Unexpected rule for key: {:?}", key_pair.as_rule()),
        };

        let value = parse_value(value_pair)?;
        map.insert(key, value);
    }

    Ok(Value::Map(map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        assert_eq!(parse("null").unwrap(), Value::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse("true").unwrap(), Value::Bool(true));
        assert_eq!(parse("false").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse("42").unwrap(), Value::Int(42));
        assert_eq!(parse("-123").unwrap(), Value::Int(-123));
        assert_eq!(parse("0xFF").unwrap(), Value::Int(255));
        assert_eq!(parse("0b1010").unwrap(), Value::Int(10));
        assert_eq!(parse("0o755").unwrap(), Value::Int(493));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse("3.14").unwrap(), Value::Float(3.14));
        assert_eq!(parse("1e10").unwrap(), Value::Float(1e10));
        assert!(
            matches!(parse("inf").unwrap(), Value::Float(f) if f.is_infinite() && f.is_sign_positive())
        );
        assert!(
            matches!(parse("-inf").unwrap(), Value::Float(f) if f.is_infinite() && f.is_sign_negative())
        );
        assert!(matches!(parse("nan").unwrap(), Value::Float(f) if f.is_nan()));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse("\"hello\"").unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            parse("'world'").unwrap(),
            Value::String("world".to_string())
        );
    }

    #[test]
    fn test_parse_list() {
        let result = parse("[1, 2, 3]").unwrap();
        assert!(matches!(result, Value::List(ref v) if v.len() == 3));
    }

    #[test]
    fn test_parse_map() {
        let result = parse("{\"key\": \"value\"}").unwrap();
        assert!(matches!(result, Value::Map(_)));
    }
}
