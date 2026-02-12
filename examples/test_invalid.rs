use std::fs;

use jasn::parse;

fn main() {
    // Try to parse invalid example files - these should all fail
    let invalid_examples = vec![
        "examples/invalid/double_underscore.jasn",
        "examples/invalid/trailing_underscore.jasn",
        "examples/invalid/leading_underscore_number.jasn",
        "examples/invalid/unquoted_keyword_key.jasn",
        "examples/invalid/invalid_escape.jasn",
        "examples/invalid/unterminated_string.jasn",
        "examples/invalid/missing_comma.jasn",
        "examples/invalid/odd_hex_binary.jasn",
        "examples/invalid/invalid_base64.jasn",
        "examples/invalid/bare_decimal_point.jasn",
        "examples/invalid/unquoted_dash_key.jasn",
        "examples/invalid/unterminated_comment.jasn",
    ];

    println!("Testing JASN Parser - Invalid Examples");
    println!("======================================\n");

    let mut correctly_failed = 0;
    let mut incorrectly_passed = 0;

    for example in &invalid_examples {
        print!("Parsing {}... ", example);
        match fs::read_to_string(example) {
            Ok(content) => match parse(&content) {
                Ok(_value) => {
                    println!("✗ INCORRECTLY PASSED (should have failed)");
                    incorrectly_passed += 1;
                }
                Err(_e) => {
                    println!("✓ correctly failed");
                    correctly_failed += 1;
                }
            },
            Err(e) => {
                println!("! File error: {}", e);
            }
        }
    }

    println!("\n======================================");
    println!(
        "Results: {} correctly failed, {} incorrectly passed",
        correctly_failed, incorrectly_passed
    );

    if incorrectly_passed > 0 {
        println!("\n⚠ WARNING: Some invalid inputs were accepted!");
    }
}
