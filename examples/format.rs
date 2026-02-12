use std::collections::BTreeMap;

use jasn::{
    Binary, Value,
    formatter::{BinaryEncoding, FormatOptions, QuoteStyle, to_string_with_options},
    to_string, to_string_pretty,
};

fn main() {
    // Create a complex JASN value
    let mut person = BTreeMap::new();
    person.insert("name".to_string(), Value::String("Alice".to_string()));
    person.insert("age".to_string(), Value::Int(30));
    person.insert("active".to_string(), Value::Bool(true));
    person.insert(
        "scores".to_string(),
        Value::List(vec![Value::Int(95), Value::Int(87), Value::Int(92)]),
    );
    person.insert(
        "avatar".to_string(),
        Value::Binary(Binary(b"Hello".to_vec())),
    );
    person.insert("pi".to_string(), Value::Float(std::f64::consts::PI));
    person.insert("temperature".to_string(), Value::Float(f64::NEG_INFINITY));

    let value = Value::Map(person);

    // Compact format (no whitespace)
    println!("=== Compact Format ===");
    println!("{}", to_string(&value));
    println!();

    // Pretty format (default)
    println!("=== Pretty Format (Default) ===");
    println!("{}", to_string_pretty(&value));
    println!();

    // Custom format options
    println!("=== Custom Format (Single Quotes, Hex Binary, Tab Indent) ===");
    let custom_options = FormatOptions::pretty()
        .with_quote_style(QuoteStyle::Single)
        .with_binary_encoding(BinaryEncoding::Hex)
        .with_indent("\t")
        .with_unquoted_keys(false);
    println!("{}", to_string_with_options(&value, &custom_options));
    println!();

    // Compact binary encoding
    println!("=== Compact Binary (Chooses Shortest) ===");
    let opts = FormatOptions::pretty().with_binary_encoding(BinaryEncoding::Compact);
    println!("{}", to_string_with_options(&value, &opts));
    println!();

    // No trailing commas
    println!("=== No Trailing Commas ===");
    let opts = FormatOptions::pretty().with_trailing_commas(false);
    println!("{}", to_string_with_options(&value, &opts));
}
