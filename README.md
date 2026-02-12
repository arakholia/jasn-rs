# JASN - Just Another Serialization Notation
Rust parser for JASN (pronounced "Jason"), a human-readable data serialization format similar to JSON but with explicit integer and binary types.

## Motivation
While JSON is widely used, it has limitations such as treating all numbers as floating-point and lacking native support for binary data. 
JASN addresses these issues by introducing distinct integer types and convenient syntax features inspired by JSON5.

## Features

- **Distinct Types**: Separate `i64` integers and `f64` floats (not everything is a float!)
- **Raw Binary Data**: Native support for binary data with `b64"..."` (base64) and `h"..."` (hex) literals
- **Comments**: Line (`//`) and block (`/* */`) comments
- **Flexible Syntax**: Trailing commas, single quotes, unquoted object keys
- **Multiple Radixes**: Support for hexadecimal (`0x`), binary (`0b`), and octal (`0o`) integer literals
- **Liberal Numbers**: Permissive decimal points (`.5`, `5.`), underscores (`1_000_000`), `inf`, `-inf`, `nan` support

## Quick Start

```rust
use jasn::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a JASN string
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

See [basic.jasn](examples/valid/basic.jasn) for summary of supported features and syntax.

## Data Model

```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Binary(Binary),  // Wrapper for `Vec<u8>`.
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}
```

## Grammar

See [GRAMMAR.md](GRAMMAR.md) for the complete formal specification.

### Examples

**Integers** (distinct from floats):
```jasn
42
-123
0xFF        // hexadecimal
0b1010      // binary
0o755       // octal
1_000_000   // underscores for readability
```

**Floats**:
```jasn
3.14
.5          // leading decimal point
5.          // trailing decimal point
1e10        // scientific notation
inf         // infinity
nan         // not a number
```

**Binary Data**:
```jasn
b64"SGVsbG8gV29ybGQh"    // base64 encoded
h"48656c6c6f"            // hex encoded
```

**Flexible Syntax**:
```jasn
{
  // Comments are supported
  unquoted_key: "value",
  'single-quotes': "work too",
  "trailing-commas": [1, 2, 3,],
}
```

## Differences from JSON

1. **Integer Type**: Numbers without decimal points are `i64`, not `f64`
2. **Binary Type**: New `b64"..."` and `h"..."` literals for byte arrays
3. **Comments**: `//` and `/* */` are supported
4. **Trailing Commas**: Allowed in arrays and objects
5. **Unquoted Keys**: Object keys can be identifiers (unless reserved)
6. **Multiple Radixes**: `0x`, `0b`, `0o` integer literals
7. **Liberal Floats**: `.5`, `5.`, `inf`, `nan` are valid

## License

MIT License - see [LICENSE](LICENSE) file for details.
