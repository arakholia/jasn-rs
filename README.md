# JASN - Just Another Serialization Notation
Rust parser for JASN (pronounced "Jason", not to be confused with "JSON"), a human-readable data serialization format similar to JSON but with explicit integer and binary types.

## Motivation
While JSON is widely used, it has limitations such as treating all numbers as floating-point and lacking native support for binary data. 
JASN addresses these issues by introducing distinct integer types and permissive syntax features inspired by JSON5.

## Features
- **Distinct Types**: Separate `i64` integers and `f64` floats (not everything is a float!)
- **Raw Binary Data**: Native support for binary data with `b64"..."` (base64) and `h"..."` (hex) literals
- **Timestamps**: ISO8601/RFC3339 timestamps with `ts"..."` syntax
- **Comments**: Line (`//`) and block (`/* */`) comments
- **Flexible Syntax**: Trailing commas, single quotes, unquoted object keys
- **Multiple Radixes**: Support for hexadecimal (`0x`), binary (`0b`), and octal (`0o`) integer literals
- **Permissive Numbers**: Leading/trailing decimal points (`.5`, `5.`), underscores (`1_000_000`), `inf`, `-inf`, `nan` support

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
    Binary(Binary),       // Wrapper for `Vec<u8>`
    Timestamp(Timestamp), // ISO8601/RFC3339 timestamp
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

**Timestamps**:
```jasn
ts"2024-01-15T12:30:45Z"           // UTC
ts"2024-01-15T12:30:45.123Z"       // with milliseconds
ts"2024-01-15T12:30:45-05:00"      // with timezone offset
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

## Comparison with JSON
JASN is a superset of JSON with the following enhancements:
1. **Integer Type**: Numbers without decimal points are `i64`, not `f64`
2. **Binary Type**: New `b64"..."` and `h"..."` literals for byte arrays
3. **Timestamp Type**: New `ts"..."` literals for ISO8601/RFC3339 timestamps
4. **Comments**: `//` and `/* */` are supported
5. **Trailing Commas**: Allowed in arrays and objects
6. **Unquoted Keys**: Object keys can be identifiers, including reserved words (`null`, `true`, `false`, `inf`, `nan`)
7. **Multiple Radixes**: `0x`, `0b`, `0o` integer literals (case-insensitive prefixes)
8. **Permissive Floats**: `.5`, `5.`, `inf`, `nan` are valid
9. **Duplicate Keys**: Explicitly disallowed - parsing fails on duplicate keys in maps

### JSON Compatibility
JASN accepts most valid JSON, with the following caveats:
  - **Integer overflow**: Integers without decimal points are parsed as `i64` (range: ±9.2 quintillion). JSON documents with larger integers will fail to parse. Workaround: use float notation (`9999999999999999999.0`) or scientific notation (`1e20`).
  - **Duplicate keys**: JASN rejects duplicate keys in objects, while JSON leaves this behavior undefined.

## Planned Features
1. **Serde Integration**: Support for `serde` serialization/deserialization
2. **JAML**: A YAML-inspired syntax using the same data model as JASN

## Features under consideration
- **Multiline Strings**: Support for multiline string literals with proper indentation handling
- **Python-style b-strings**: `b'''...'''` for raw byte strings without escaping (similar to Python's `b''` literals)

## License
MIT License - see [LICENSE](LICENSE) file for details.
