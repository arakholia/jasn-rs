use std::{fs, path::Path};

use jaml::parse;

#[test]
fn test_all_invalid_examples() {
    let invalid_dir = Path::new("examples/invalid");

    let mut examples: Vec<_> = fs::read_dir(invalid_dir)
        .expect("Failed to read invalid examples directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "jaml" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    examples.sort();

    assert!(!examples.is_empty(), "No invalid example files found");

    for example in examples {
        let content = fs::read_to_string(&example)
            .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", example, e));

        assert!(
            parse(&content).is_err(),
            "Invalid example {:?} should have failed to parse but succeeded",
            example
        );
    }
}
