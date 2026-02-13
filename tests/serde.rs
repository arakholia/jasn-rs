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

    let jasn = jasn::to_string(&data).unwrap();
    assert!(jasn.contains("null"));
    assert!(jasn.contains("true"));
    assert!(jasn.contains("42"));
    assert!(jasn.contains("2.5"));
    assert!(jasn.contains("hello"));
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

    let jasn = jasn::to_string(&data).unwrap();
    assert!(jasn.contains("[1, 2, 3]") || jasn.contains("[1,2,3]"));
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

    let jasn = jasn::to_string(&data).unwrap();
    assert!(jasn.contains("test"));
    assert!(jasn.contains("42"));
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
    let jasn = jasn::to_string(&color).unwrap();
    assert_eq!(jasn, "\"Green\"");
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
    let jasn = jasn::to_string(&msg).unwrap();
    assert!(jasn.contains("Text"));
    assert!(jasn.contains("hello"));
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
    let jasn = jasn::to_string(&event).unwrap();
    assert!(jasn.contains("Click"));
    assert!(jasn.contains("10"));
    assert!(jasn.contains("20"));
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

    let jasn = r#"{
        null_field: null,
        bool_field: true,
        int_field: 42,
        float_field: 2.5,
        string_field: "hello"
    }"#;

    let data: Data = jasn::from_str(jasn).unwrap();
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

    let jasn = r#"{ items: [1, 2, 3] }"#;
    let data: Data = jasn::from_str(jasn).unwrap();
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

    let jasn = r#"{
        name: "test",
        inner: { value: 42 }
    }"#;

    let data: Outer = jasn::from_str(jasn).unwrap();
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

    let jasn = "\"Green\"";
    let color: Color = jasn::from_str(jasn).unwrap();
    assert_eq!(color, Color::Green);
}

#[test]
fn test_deserialize_enum_newtype() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Message {
        Text(String),
        Number(i32),
    }

    let jasn = r#"{ Text: "hello" }"#;
    let msg: Message = jasn::from_str(jasn).unwrap();
    assert_eq!(msg, Message::Text("hello".to_string()));
}

#[test]
fn test_deserialize_enum_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress { key: String },
    }

    let jasn = r#"{ Click: { x: 10, y: 20 } }"#;
    let event: Event = jasn::from_str(jasn).unwrap();
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

    let jasn = jasn::to_string(&original).unwrap();
    let deserialized: Simple = jasn::from_str(&jasn).unwrap();
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

    let original = Advanced {
        null_field: None,
        bool_field: true,
        int_field: -9223372036854775807, // Large i64
        negative_int: -42,
        float_field: 2.5,
        scientific: 1.5e10,
        float_inf: f64::INFINITY,
        float_nan: f64::NAN,
        string_field: "Hello, JASN!".to_string(),
        unicode_string: "世界 🌍".to_string(),
        binary_field: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello"
        timestamp_field: time::OffsetDateTime::parse(
            "2024-01-15T12:30:45Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
        list_field: vec![1, 2, 3, 4, 5],
        nested_list: vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ],
        nested_struct: Nested {
            id: 123,
            tags: vec!["rust".to_string(), "parser".to_string()],
        },
        map_field: {
            let mut map = HashMap::new();
            map.insert("one".to_string(), 1);
            map.insert("two".to_string(), 2);
            map.insert("three".to_string(), 3);
            map
        },
        optional_value: Some("present".to_string()),
    };

    // Test with compact format
    let jasn_compact = jasn::to_string(&original).unwrap();
    let deserialized_compact: Advanced = jasn::from_str(&jasn_compact).unwrap();

    // Custom comparison for fields (since we can't rely on PartialEq for f64 due to NaN)
    assert_eq!(original.null_field, deserialized_compact.null_field);
    assert_eq!(original.bool_field, deserialized_compact.bool_field);
    assert_eq!(original.int_field, deserialized_compact.int_field);
    assert_eq!(original.negative_int, deserialized_compact.negative_int);
    assert_eq!(original.float_field, deserialized_compact.float_field);
    assert_eq!(original.scientific, deserialized_compact.scientific);
    assert_eq!(original.string_field, deserialized_compact.string_field);
    assert_eq!(original.unicode_string, deserialized_compact.unicode_string);
    assert_eq!(original.binary_field, deserialized_compact.binary_field);
    assert_eq!(
        original.timestamp_field,
        deserialized_compact.timestamp_field
    );
    assert_eq!(original.list_field, deserialized_compact.list_field);
    assert_eq!(original.nested_list, deserialized_compact.nested_list);
    assert_eq!(original.nested_struct, deserialized_compact.nested_struct);
    assert_eq!(original.optional_value, deserialized_compact.optional_value);

    // Test with pretty format
    let jasn_pretty = jasn::to_string_pretty(&original).unwrap();
    let deserialized_pretty: Advanced = jasn::from_str(&jasn_pretty).unwrap();

    assert_eq!(original.null_field, deserialized_pretty.null_field);
    assert_eq!(original.bool_field, deserialized_pretty.bool_field);
    assert_eq!(original.int_field, deserialized_pretty.int_field);
    assert_eq!(original.negative_int, deserialized_pretty.negative_int);
    assert_eq!(original.float_field, deserialized_pretty.float_field);
    assert_eq!(original.scientific, deserialized_pretty.scientific);
    assert_eq!(original.string_field, deserialized_pretty.string_field);
    assert_eq!(original.unicode_string, deserialized_pretty.unicode_string);
    assert_eq!(original.binary_field, deserialized_pretty.binary_field);
    assert_eq!(
        original.timestamp_field,
        deserialized_pretty.timestamp_field
    );
    assert_eq!(original.list_field, deserialized_pretty.list_field);
    assert_eq!(original.nested_list, deserialized_pretty.nested_list);
    assert_eq!(original.nested_struct, deserialized_pretty.nested_struct);
    assert_eq!(original.optional_value, deserialized_pretty.optional_value);
}

#[test]
fn test_pretty_format() {
    #[derive(Serialize)]
    struct Data {
        name: String,
        value: i32,
    }

    let data = Data {
        name: "test".to_string(),
        value: 42,
    };

    let jasn = jasn::to_string_pretty(&data).unwrap();
    // Pretty format should have newlines
    assert!(jasn.contains('\n'));
    assert!(jasn.contains("name"));
    assert!(jasn.contains("test"));
    assert!(jasn.contains("42"));
}

#[test]
fn test_custom_format_options() {
    #[derive(Serialize)]
    struct Data {
        name: String,
    }

    let data = Data {
        name: "test".to_string(),
    };

    let opts = jasn::formatter::Options {
        indent: "    ".to_string(),
        ..Default::default()
    };

    let jasn = jasn::ser::to_string_opts(&data, &opts).unwrap();
    assert!(jasn.contains("test"));
}
