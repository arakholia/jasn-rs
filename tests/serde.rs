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
        float_field: 3.14,
        string_field: "hello".to_string(),
    };

    let jasn = jasn::to_string(&data).unwrap();
    assert!(jasn.contains("null"));
    assert!(jasn.contains("true"));
    assert!(jasn.contains("42"));
    assert!(jasn.contains("3.14"));
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
        float_field: 3.14,
        string_field: "hello"
    }"#;

    let data: Data = jasn::from_str(jasn).unwrap();
    assert_eq!(data.null_field, None);
    assert_eq!(data.bool_field, true);
    assert_eq!(data.int_field, 42);
    assert_eq!(data.float_field, 3.14);
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
fn test_roundtrip() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Config {
        name: String,
        version: u32,
        enabled: bool,
        values: Vec<i32>,
    }

    let original = Config {
        name: "test".to_string(),
        version: 1,
        enabled: true,
        values: vec![1, 2, 3],
    };

    let jasn = jasn::to_string(&original).unwrap();
    let deserialized: Config = jasn::from_str(&jasn).unwrap();
    assert_eq!(original, deserialized);
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

    let jasn = jasn::to_string_opts(&data, &opts).unwrap();
    assert!(jasn.contains("test"));
}
