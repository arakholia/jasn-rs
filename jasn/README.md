# JASN

A Rust library for parsing and formatting JASN (Just Another Serialization Notation).

JASN is a human-readable data serialization format similar to JSON but with explicit integer and binary types, plus convenient JSON5-inspired syntax features.

## Quick Start

```rust
use jasn::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = parse(r#"
        {
          name: "Alice",
          age: 30,
          balance: 1_234.56,
          data: b64"SGVsbG8=",
          tags: ["rust", "json", "parser"],
        }
    "#)?;
    
    println!("{:#?}", value);
    Ok(())
}
```

## Features

- **Distinct Types**: Separate `i64` integers and `f64` floats
- **Raw Binary Data**: Native `b64"..."` (base64) and `hex"..."` (hex) literals
- **Timestamps**: ISO8601/RFC3339 timestamps with `ts"..."` syntax
- **Comments**: Block comments (`/* */`) only (whitespace-agnostic design)
- **Flexible Syntax**: Trailing commas, single quotes, unquoted object keys
- **Multiple Radixes**: Hexadecimal (`0x`), binary (`0b`), and octal (`0o`) integers
- **Permissive Numbers**: Leading/trailing decimal points (`.5`, `5.`), underscores (`1_000_000`), `inf`, `-inf`, `nan`

## Installation

```toml
[dependencies]
jasn = "0.2"
```

For more examples and documentation, see the [main repository README](../README.md).

## License

MIT License - see [LICENSE](../LICENSE) file for details.
