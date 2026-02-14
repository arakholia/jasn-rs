use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    name: String,
    version: u32,
    enabled: bool,
    timeout_ms: Option<u64>,
    tags: Vec<String>,
}

fn main() {
    let config = Config {
        name: "My App".to_string(),
        version: 1,
        enabled: true,
        timeout_ms: Some(5000),
        tags: vec!["rust".to_string(), "jaml".to_string()],
    };

    // Serialize Rust struct to JAML format
    let jaml = jaml::to_string(&config).unwrap();
    println!("Serialized to JAML:\n{}\n", jaml);

    // Deserialize JAML back to Rust struct
    let deserialized: Config = jaml::from_str(&jaml).unwrap();
    println!("Deserialized: {:?}\n", deserialized);

    // Verify roundtrip
    assert_eq!(config, deserialized);
    println!("✓ Roundtrip successful!");

    // You can also deserialize from JAML text directly
    let jaml_text = r#"
name: "Direct Parse"
version: 2
enabled: false
timeout_ms: null
tags:
  - "example"
  - "demo"
"#;

    let parsed: Config = jaml::from_str(jaml_text).unwrap();
    println!("\nParsed from JAML text: {:?}", parsed);
}
