use std::fs;

use jasn::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file.jasn>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(content) => match parse(&content) {
            Ok(value) => {
                println!("Successfully parsed {}:", filename);
                println!("{:#?}", value);
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    }
}
