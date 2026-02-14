use jaml::{parse, Value};
use std::fs;

#[test]
fn test_parse_basic() {
    let input = fs::read_to_string("examples/valid/basic.jaml")
        .expect("Failed to read basic.jaml");
    
    let value = parse(&input).expect("Failed to parse basic.jaml");
    
    // Verify we can parse it
    assert!(value.is_map(), "Root should be a map");
    
    // Check a few specific values
    let map = value.as_map().unwrap();
    
    // Check null value
    assert_eq!(map.get("nothing"), Some(&Value::Null));
    
    // Check booleans
    assert_eq!(map.get("active"), Some(&Value::Bool(true)));
    assert_eq!(map.get("disabled"), Some(&Value::Bool(false)));
    
    // Check integer
    assert_eq!(map.get("count"), Some(&Value::Int(42)));
    
    // Check string
    if let Some(Value::String(s)) = map.get("name") {
        assert_eq!(s, "Alice");
    } else {
        panic!("name should be a string");
    }
}
