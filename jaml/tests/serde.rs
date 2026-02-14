#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

#[test]
fn test_serialize_primitives() {
    #[derive(Serialize)]
    struct Data {
        null_field: Option<i32>,
        bool_field: bool,
        int_field: i64,
        float_field: f64,
        string_field: String,
    }

    let data = Data {
        null_field: None,
        bool_field: true,
        int_field: 42,
        float_field: 2.5,
        string_field: "hello".to_string(),
    };

    let jaml = jaml::to_string(&data).unwrap();
    assert!(jaml.contains("null"));
    assert!(jaml.contains("true"));
    assert!(jaml.contains("42"));
    assert!(jaml.contains("2.5"));
    assert!(jaml.contains("hello"));
}

#[test]
fn test_serialize_list() {
    #[derive(Serialize)]
    struct Data {
        items: Vec<i32>,
    }

    let data = Data {
        items: vec![1, 2, 3],
    };

    let jaml = jaml::to_string(&data).unwrap();
    assert!(jaml.contains("items:"));
    assert!(jaml.contains("- 1"));
    assert!(jaml.contains("- 2"));
    assert!(jaml.contains("- 3"));
}

#[test]
fn test_serialize_nested_struct() {
    #[derive(Serialize)]
    struct Inner {
        value: i32,
    }

    #[derive(Serialize)]
    struct Outer {
        name: String,
        inner: Inner,
    }

    let data = Outer {
        name: "test".to_string(),
        inner: Inner { value: 42 },
    };

    let jaml = jaml::to_string(&data).unwrap();
    assert!(jaml.contains("test"));
    assert!(jaml.contains("42"));
}

#[test]
fn test_serialize_enum_unit() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Green;
    let jaml = jaml::to_string(&color).unwrap();
    assert_eq!(jaml, "\"Green\"");
}

#[test]
fn test_serialize_enum_newtype() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Message {
        Text(String),
        Number(i32),
    }

    let msg = Message::Text("hello".to_string());
    let jaml = jaml::to_string(&msg).unwrap();
    assert!(jaml.contains("Text"));
    assert!(jaml.contains("hello"));
}

#[test]
fn test_serialize_enum_struct() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress { key: String },
    }

    let event = Event::Click { x: 10, y: 20 };
    let jaml = jaml::to_string(&event).unwrap();
    assert!(jaml.contains("Click"));
    assert!(jaml.contains("10"));
    assert!(jaml.contains("20"));
}

#[test]
fn test_deserialize_primitives() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        null_field: Option<i32>,
        bool_field: bool,
        int_field: i64,
        float_field: f64,
        string_field: String,
    }

    let jaml = r#"
null_field: null
bool_field: true
int_field: 42
float_field: 2.5
string_field: "hello"
"#;

    let data: Data = jaml::from_str(jaml).unwrap();
    assert_eq!(data.null_field, None);
    assert!(data.bool_field);
    assert_eq!(data.int_field, 42);
    assert_eq!(data.float_field, 2.5);
    assert_eq!(data.string_field, "hello");
}

#[test]
fn test_deserialize_list() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        items: Vec<i32>,
    }

    let jaml = r#"
items:
  - 1
  - 2
  - 3
"#;
    let data: Data = jaml::from_str(jaml).unwrap();
    assert_eq!(data.items, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_nested() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Inner {
        value: i32,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Outer {
        name: String,
        inner: Inner,
    }

    let jaml = r#"
name: "test"
inner:
  value: 42
"#;

    let data: Outer = jaml::from_str(jaml).unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.inner.value, 42);
}

#[test]
fn test_deserialize_enum_unit() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    let jaml = "\"Green\"";
    let color: Color = jaml::from_str(jaml).unwrap();
    assert_eq!(color, Color::Green);
}

#[test]
fn test_deserialize_enum_newtype() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Message {
        Text(String),
        Number(i32),
    }

    let jaml = r#"
Text: "hello"
"#;
    let msg: Message = jaml::from_str(jaml).unwrap();
    assert_eq!(msg, Message::Text("hello".to_string()));
}

#[test]
fn test_deserialize_enum_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress { key: String },
    }

    let jaml = r#"
Click:
  x: 10
  y: 20
"#;
    let event: Event = jaml::from_str(jaml).unwrap();
    assert_eq!(event, Event::Click { x: 10, y: 20 });
}

#[test]
fn test_roundtrip_simple() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Simple {
        name: String,
        version: u32,
        enabled: bool,
        values: Vec<i32>,
    }

    let original = Simple {
        name: "test".to_string(),
        version: 1,
        enabled: true,
        values: vec![1, 2, 3],
    };

    let jaml = jaml::to_string(&original).unwrap();
    let deserialized: Simple = jaml::from_str(&jaml).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_roundtrip_advanced() {
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Nested {
        id: i32,
        tags: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Advanced {
        // Null (via Option::None)
        null_field: Option<i32>,

        // Bool
        bool_field: bool,

        // Int (various integer types)
        int_field: i64,
        negative_int: i32,

        // Float
        float_field: f64,
        scientific: f64,
        float_inf: f64,
        float_nan: f64,

        // String (with unicode)
        string_field: String,
        unicode_string: String,

        // Binary data (using serde_bytes for proper binary serialization)
        #[serde(with = "serde_bytes")]
        binary_field: Vec<u8>,

        // Timestamp (using time::OffsetDateTime with serde support)
        #[serde(with = "time::serde::rfc3339")]
        timestamp_field: time::OffsetDateTime,

        // List
        list_field: Vec<i32>,

        // Nested list
        nested_list: Vec<Vec<String>>,

        // Map (via nested struct)
        nested_struct: Nested,

        // Map (via HashMap)
        map_field: HashMap<String, i32>,

        // Option::Some
        optional_value: Option<String>,
    }

    impl PartialEq for Advanced {
        fn eq(&self, other: &Self) -> bool {
            self.null_field == other.null_field
                && self.bool_field == other.bool_field
                && self.int_field == other.int_field
                && self.negative_int == other.negative_int
                && self.float_field == other.float_field
                && self.scientific == other.scientific
                && self.float_inf == other.float_inf
                && self.float_nan.is_nan() == other.float_nan.is_nan()
                && self.string_field == other.string_field
                && self.unicode_string == other.unicode_string
                && self.binary_field == other.binary_field
                && self.timestamp_field == other.timestamp_field
                && self.list_field == other.list_field
                && self.nested_list == other.nested_list
                && self.nested_struct == other.nested_struct
                && self.map_field == other.map_field
                && self.optional_value == other.optional_value
        }
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 42);
    map.insert("key2".to_string(), 99);

    let original = Advanced {
        null_field: None,
        bool_field: true,
        int_field: 123456789,
        negative_int: -987,
        float_field: 123.456,
        scientific: 1.23e-10,
        float_inf: f64::INFINITY,
        float_nan: f64::NAN,
        string_field: "hello world".to_string(),
        unicode_string: "Hello 世界 🦀".to_string(),
        binary_field: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f],
        timestamp_field: time::macros::datetime!(2024-01-15 12:30:45 UTC),
        list_field: vec![1, 2, 3, 4, 5],
        nested_list: vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ],
        nested_struct: Nested {
            id: 42,
            tags: vec!["rust".to_string(), "serde".to_string()],
        },
        map_field: map,
        optional_value: Some("present".to_string()),
    };

    let jaml = jaml::to_string(&original).unwrap();
    let deserialized: Advanced = jaml::from_str(&jaml).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_to_value() {
    #[derive(Serialize)]
    struct Data {
        name: String,
        count: i32,
    }

    let data = Data {
        name: "test".to_string(),
        count: 42,
    };

    let value = jaml::to_value(&data).unwrap();
    assert!(value.is_map());
}

#[test]
fn test_from_value() {
    use std::collections::BTreeMap;

    use jaml::Value;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        name: String,
        count: i32,
    }

    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::String("test".to_string()));
    map.insert("count".to_string(), Value::Int(42));
    let value = Value::Map(map);

    let data: Data = jaml::from_value(&value).unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.count, 42);
}

#[test]
fn test_inline_list_syntax() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        items: Vec<i32>,
    }

    let jaml = r#"items: [1, 2, 3]"#;
    let data: Data = jaml::from_str(jaml).unwrap();
    assert_eq!(data.items, vec![1, 2, 3]);
}

#[test]
fn test_inline_map_syntax() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Inner {
        x: i32,
        y: i32,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        point: Inner,
    }

    let jaml = r#"point: {x: 10, y: 20}"#;
    let data: Data = jaml::from_str(jaml).unwrap();
    assert_eq!(data.point.x, 10);
    assert_eq!(data.point.y, 20);
}
