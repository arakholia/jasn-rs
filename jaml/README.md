# JAML

A Rust library for parsing and formatting JAML (Just Another Markup Language).

JAML is a human-readable data serialization format similar to YAML but with explicit integer and binary types, sharing the same data model as JASN.

## Quick Start

```rust
use jaml::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = parse(r#"
name: Alice
age: 30
balance: 1234.56
data: b64"SGVsbG8="
tags:
  - rust
  - yaml
  - parser
    "#)?;
    
    println!("{:#?}", value);
    Ok(())
}
```

## Features

- **Distinct Types**: Separate `i64` integers and `f64` floats
- **Raw Binary Data**: Native `b64"..."` (base64) and `hex"..."` (hex) literals
- **Timestamps**: ISO8601/RFC3339 timestamps
- **YAML-inspired syntax**: Indentation-based structure, cleaner appearance
- **Shared Data Model**: Uses `jasn-core` types, compatible with JASN format

## Installation

```toml
[dependencies]
jaml = "0.2"
```

For more information about the shared data model, see [jasn-core](../jasn-core/README.md).

## License

MIT License - see [LICENSE](../LICENSE) file for details.
