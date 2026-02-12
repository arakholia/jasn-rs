use std::collections::BTreeMap;

use jasn::{
    Binary, Value,
    formatter::{
        Options,
        options::{BinaryEncoding, QuoteStyle},
        to_string_opts,
    },
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
    person.insert("ferris".to_string(), Value::String("🦀".to_string()));
    person.insert(
        "timestamp".to_string(),
        Value::Timestamp(chrono::DateTime::from_timestamp(1234567890, 0).unwrap()),
    );

    let value = Value::Map(person);

    // Compact format (no whitespace)
    println!("=== Compact Format ===");
    println!("{}", to_string(&value));
    println!();

    // Pretty format (default)
    println!("=== Pretty Format ===");
    println!("{}", to_string_pretty(&value));
    println!();

    // Custom format options
    println!(
        "=== Custom Format (Single Quotes, Hex Binary, 4-Space Indent, No Trailing Commas, Quoted Keys, Leading Plus, Sorted Keys, Unicode Escape) ==="
    );
    let custom_options = Options::pretty()
        .with_quote_style(QuoteStyle::Single)
        .with_binary_encoding(BinaryEncoding::Hex)
        .with_indent("    ")
        .with_trailing_commas(false)
        .with_unquoted_keys(false)
        .with_leading_plus(true)
        .with_sort_keys(true)
        .with_escape_unicode(true);
    println!("{}", to_string_opts(&value, &custom_options));
}
