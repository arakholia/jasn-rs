use std::fs;

use jasn::parse;

fn main() {
    // Try to parse example files
    let examples = vec![
        "examples/valid/minimal.jasn",
        "examples/valid/numbers_only.jasn",
        "examples/valid/string_only.jasn",
        "examples/valid/list_only.jasn",
        "examples/valid/trailing_commas.jasn",
        "examples/valid/comments.jasn",
        "examples/valid/edge_cases.jasn",
        "examples/valid/basic.jasn",
    ];

    println!("Testing JASN Parser");
    println!("==================\n");
    
    let mut passed = 0;
    let mut failed = 0;

    for example in &examples {
        print!("Parsing {}... ", example);
        match fs::read_to_string(example) {
            Ok(content) => match parse(&content) {
                Ok(_value) => {
                    println!("✓");
                    passed += 1;
                }
                Err(e) => {
                    println!("✗");
                    println!("  Error: {}", e);
                    failed += 1;
                }
            },
            Err(e) => {
                println!("✗");
                println!("  File error: {}", e);
                failed += 1;
            }
        }
    }
    
    println!("\n==================");
    println!("Results: {} passed, {} failed", passed, failed);
}
