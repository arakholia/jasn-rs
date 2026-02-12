use std::fs;

use jasn::parse;

fn main() {
    // Try to parse one of the example files
    let examples = vec![
        "examples/valid/minimal.jasn",
        "examples/valid/numbers_only.jasn",
        "examples/valid/string_only.jasn",
        "examples/valid/list_only.jasn",
    ];

    for example in examples {
        println!("\n=== Parsing {} ===", example);
        match fs::read_to_string(example) {
            Ok(content) => match parse(&content) {
                Ok(value) => println!("✓ Success: {:?}", value),
                Err(e) => println!("✗ Parse error: {}", e),
            },
            Err(e) => println!("✗ File error: {}", e),
        }
    }
}
