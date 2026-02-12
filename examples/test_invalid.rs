use std::fs;

use jasn::parse;

fn main() {
    println!("Testing JASN Parser - Invalid Examples");
    println!("======================================\n");

    let invalid_dir = "examples/invalid";
    let mut examples: Vec<_> = fs::read_dir(invalid_dir)
        .expect("Failed to read invalid examples directory")
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

    let mut correctly_failed = 0;
    let mut incorrectly_passed = 0;

    for example in &examples {
        let display_path = example.display();
        print!("Parsing {}... ", display_path);
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
        std::process::exit(1);
    }
}
