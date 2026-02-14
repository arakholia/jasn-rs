use std::collections::BTreeMap;

use jaml::{Value, format, parse};

#[test]
fn test_format_simple_values() {
    assert_eq!(format(&Value::Null), "null");
    assert_eq!(format(&Value::Bool(true)), "true");
    assert_eq!(format(&Value::Int(42)), "42");
    assert_eq!(format(&Value::String("hello".to_string())), "\"hello\"");
}

#[test]
fn test_format_list() {
    let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    let result = format(&list);
    assert_eq!(result, "- 1\n- 2\n- 3\n");
}

#[test]
fn test_format_map() {
    let mut map = BTreeMap::new();
    map.insert("age".to_string(), Value::Int(30));
    map.insert("name".to_string(), Value::String("Alice".to_string()));

    let result = format(&Value::Map(map));
    assert_eq!(result, "age: 30\nname: \"Alice\"\n");
}

#[test]
fn test_format_nested() {
    let mut inner = BTreeMap::new();
    inner.insert("count".to_string(), Value::Int(5));
    inner.insert("enabled".to_string(), Value::Bool(true));

    let mut outer = BTreeMap::new();
    outer.insert("config".to_string(), Value::Map(inner));

    let result = format(&Value::Map(outer));
    assert!(result.contains("config:\n"));
    assert!(result.contains("  count: 5\n"));
    assert!(result.contains("  enabled: true\n"));
}

#[test]
fn test_round_trip() {
    let input = "name: \"Alice\"\nage: 30\n";
    let value = parse(input).unwrap();
    let formatted = format(&value);
    let reparsed = parse(&formatted).unwrap();
    assert_eq!(value, reparsed);
}
