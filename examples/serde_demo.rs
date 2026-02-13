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
        tags: vec!["rust".to_string(), "jasn".to_string()],
    };

    // Serialize Rust struct to JASN format
    let jasn = jasn::to_string_pretty(&config).unwrap();
    println!("Serialized to JASN:\n{}\n", jasn);

    // Deserialize JASN back to Rust struct
    let deserialized: Config = jasn::from_str(&jasn).unwrap();
    println!("Deserialized: {:?}\n", deserialized);

    // Verify roundtrip
    assert_eq!(config, deserialized);
    println!("✓ Roundtrip successful!");

    // You can also deserialize from JASN text directly
    let jasn_text = r#"{
        name: "Direct Parse",
        version: 2,
        enabled: false,
        timeout_ms: null,
        tags: ["example", "demo"]
    }"#;

    let parsed: Config = jasn::from_str(jasn_text).unwrap();
    println!("\nParsed from JASN text: {:?}", parsed);
}
