use std::io::{self, Read};
use jaml;

fn main() {
    let mut input = String::new();
    
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }
    
    match jaml::parse(&input) {
        Ok(value) => {
            println!("{:#?}", value);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
