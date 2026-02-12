use std::fs;

use jasn::parse;

fn main() {
    println!("Testing JASN Parser - Valid Examples");
    println!("====================================\n");

    let valid_dir = "examples/valid";
    let mut examples: Vec<_> = fs::read_dir(valid_dir)
        .expect("Failed to read valid examples directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "jasn" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    examples.sort();

    let mut passed = 0;
    let mut failed = 0;

    for example in &examples {
        let display_path = example.display();
        print!("Parsing {}... ", display_path);
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

    println!("\n====================================");
    println!("Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }
}
