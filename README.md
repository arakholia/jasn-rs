# JASN - Just Another Serialization Notation
JASN (pronounced "Jason", not to be confused with "JSON") is a human-readable data serialization format similar to JSON but with explicit integer and binary types.

![competing standards](https://imgs.xkcd.com/comics/standards.png)

## Motivation
While JSON is widely used, it has limitations such as treating all numbers as floating-point and lacking native support for binary data. 
JASN addresses these issues by introducing distinct integer and binary types and permissive syntax features inspired by [JSON5](https://json5.org/).

## Features
- **Distinct Types**: Separate `i64` integers and `f64` floats (not everything is a float!)
- **Raw Binary Data**: Native support for binary data with `b64"..."` (base64) and `hex"..."` (hex) literals
- **Timestamps**: ISO8601/RFC3339 timestamps with `ts"..."` syntax
- **Comments**: Block comments (`/* */`) only (whitespace-agnostic design)
- **Flexible Syntax**: Trailing commas, single quotes, unquoted object keys
- **Multiple Radixes**: Support for hexadecimal (`0x`), binary (`0b`), and octal (`0o`) integer literals
- **Permissive Numbers**: Leading/trailing decimal points (`.5`, `5.`), underscores (`1_000_000`), `inf`, `-inf`, `nan` support

## Example
A comprehensive example showing all supported value types:

```jasn
{
  /* Comments are supported */
  null_value: null,
  
  /* Booleans */
  bool_true: true,
  bool_false: false,
  
  /* Integers (explicit type, no decimal point) */
  integer: 42,
  negative: -123,
  hex: 0xFF,
  binary: 0b1010,
  octal: 0o755,
  with_underscores: 1_000_000,
  
  /* Floats (always have decimal point or exponent) */
  float: 3.14,
  scientific: 1.5e10,
  special_inf: inf,
  special_neg_inf: -inf,
  special_nan: nan,
  
  /* Strings (double or single quotes) */
  string_double: "Hello, World!",
  string_single: 'Hello, World!',
  string_unicode: "Hello \u4E16\u754C",  /* Unicode escapes */
  
  /* Binary data */
  binary_hex: hex"48656c6c6f",           /* Hex encoding */
  binary_base64: b64"SGVsbG8gV29ybGQ=", /* Base64 encoding */
  
  /* Timestamps (RFC3339/ISO8601) */
  timestamp: ts"2024-01-15T12:30:45Z",
  timestamp_offset: ts"2024-01-15T12:30:45-05:00",
  
  /* Lists */
  list: [1, 2, 3, "mixed", true, null],
  nested_list: [[1, 2], [3, 4]],
  
  /* Maps (objects) */
  map: {
    unquoted_key: "value",
    "quoted key": "also works",
    nested: { a: 1, b: 2 },
  },
  
  /* Trailing commas allowed */
  trailing: [1, 2, 3,],
}
```

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
See [GRAMMAR.md](jasn/GRAMMAR.md) for the complete grammar specification.

> **Note:** The specification is still under active development and may be subject to change.

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
hex"48656c6c6f"            // hex encoded
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
  /* Comments are supported */
  unquoted_key: "value",
  'single-quotes': "work too",
  "trailing-commas": [1, 2, 3,],
}
```

## Comparison with JSON
JASN is a superset of JSON with the following enhancements:
1. **Integer Type**: Numbers without decimal points are `i64`, not `f64`
2. **Binary Type**: New `b64"..."` and `hex"..."` literals for byte arrays
3. **Timestamp Type**: New `ts"..."` literals for ISO8601/RFC3339 timestamps
4. **Comments**: Block comments `/* */` only (whitespace-agnostic design)
5. **Trailing Commas**: Allowed in arrays and objects
6. **Unquoted Keys**: Object keys can be identifiers, including reserved words (`null`, `true`, `false`, `inf`, `nan`)
7. **Multiple Radixes**: `0x`, `0b`, `0o` integer literals (case-insensitive prefixes)
8. **Permissive Floats**: `.5`, `5.`, `inf`, `nan` are valid
9. **Duplicate Keys**: Explicitly disallowed - parsing fails on duplicate keys in maps

### JSON Compatibility
JASN accepts most valid JSON, with the following caveats:
  - **Integer overflow**: Integers without decimal points are parsed as `i64` (range: ±9.2 quintillion). JSON documents with larger integers will fail to parse. 
    - Workaround: use float notation (`9999999999999999999.0`) or scientific notation (`1e20`).
  - **Duplicate keys**: JASN rejects duplicate keys in objects, while JSON leaves this behavior undefined.

## Serde Integration
JASN provides custom `Serializer` and `Deserializer` implementations, allowing you to serialize and deserialize **any** Rust type directly to/from JASN format (not just JASN's `Value` type).

**Add to your `Cargo.toml`**:
```toml
[dependencies]
jasn = "0.1"
```

**Example usage**:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    name: String,
    version: u32,
    enabled: bool,
}

// Serialize Rust struct directly to JASN text
let config = Config { 
    name: "app".into(), 
    version: 1, 
    enabled: true 
};
let jasn = jasn::to_string_pretty(&config).unwrap();

// Deserialize JASN text directly to Rust struct
let parsed: Config = jasn::from_str(&jasn).unwrap();
assert_eq!(config, parsed);
```

This works with all serde-compatible types: structs, enums, vectors, maps, options, etc.

See [examples/serde_demo.rs](examples/serde_demo.rs) for a complete example.

## JAML - Alternative YAML-like Syntax

JAML (Just Another Markup Language) provides a **YAML-inspired indentation-based syntax** for the same data model as JASN. If you prefer YAML's cleaner look without braces, JAML is for you.

**Example**:
```jaml
# Same data as JASN, YAML-style syntax
name: "Alice"
age: 30
balance: 1234.56
data: b64"SGVsbG8="
tags:
  - "rust"
  - "yaml"
  - "parser"
config:
  timeout: 30
  retries: 5
  enabled: true
```

**Key Features**:
- Indentation-based structure (like YAML/Python)
- Same types as JASN: distinct integers, floats, binary, timestamps
- Explicit string quoting (avoids YAML's "Norway Problem")
- Full serde support
- Flexible indentation: first indent defines base unit

**Note:** JAML is **not a superset** of JASN (unlike how JASN is a superset of JSON). While both share the same data model, they use intentionally disjoint syntax styles:
- JAML supports inline `[...]` and `{...}` but only single-line (no newlines)
- JAML uses line comments `#`, JASN uses block comments `/* */`
- Both can represent the same data, but with different syntax

**Installation**:
```toml
[dependencies]
jaml = "0.2"
```

See the [JAML README](jaml/README.md) for complete documentation and examples.

## Planned Features

- **Streaming Serde Implementation**: Support for parsing and serializing large documents without loading entire structure into memory
- **Faster/Custom Parser**: Replace Pest with a custom parser for improved performance

## Documentation

To view the API documentation locally:

```bash
# Build documentation for all workspace crates
cargo doc --workspace --no-deps

# Generate landing page with links to JASN and JAML docs
./generate-doc-index.sh

# Open in browser
xdg-open target/doc/index.html  # Linux
open target/doc/index.html      # macOS
start target/doc/index.html     # Windows
```

Or view the published documentation online:
- **JASN**: [docs.rs/jasn](https://docs.rs/jasn)
- **JAML**: [docs.rs/jaml](https://docs.rs/jaml)

## License
MIT License - see [LICENSE](LICENSE) file for details.
