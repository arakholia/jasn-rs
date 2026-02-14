# JAML - Just Another Markup Language

A Rust library for parsing and formatting JAML, a **YAML-inspired data serialization format** with explicit integer and binary types.

JAML shares the same data model as [JASN](../jasn/README.md) but uses indentation-based syntax for a cleaner, more readable appearance. Think "YAML done right" - no implicit type conversions, no ambiguous syntax, explicit types.

## Why JAML?

- **Clean YAML-like syntax** without the gotchas (no "Norway Problem")
- **Explicit types**: `42` is an integer, `42.0` is a float
- **Quoted strings**: No implicit type conversion, all string values must be quoted
- **Native binary data**: `b64"..."` and `hex"..."` literals
- **Timestamps**: First-class `ts"..."` support
- **Flexible indentation**: First indent defines base unit (2 spaces, 4 spaces, tabs, etc.)
- **Full serde support**: Serialize/deserialize any Rust type

## Quick Start

### Parsing JAML

```rust
use jaml::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = parse(r#"
name: "Alice"
age: 30
balance: 1234.56
data: b64"SGVsbG8="
tags:
  - "rust"
  - "yaml"
  - "parser"
    "#)?;
    
    println!("{:#?}", value);
    Ok(())
}
```

### Serde Integration

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: u32,
    enabled: bool,
    timeout_ms: Option<u64>,
    tags: Vec<String>,
}

let config = Config {
    name: "My App".to_string(),
    version: 1,
    enabled: true,
    timeout_ms: Some(5000),
    tags: vec!["rust".to_string(), "jaml".to_string()],
};

// Serialize to JAML
let jaml = jaml::to_string(&config)?;

// Deserialize from JAML
let parsed: Config = jaml::from_str(&jaml)?;
```

## Example

A comprehensive example showing all supported types:

```jaml
# Null
nothing: null

# Booleans
active: true
disabled: false

# Integers (distinct from floats)
count: 42
negative: -123
hex_value: 0xFF
binary_value: 0b1010
octal_value: 0o755
with_underscores: 1_000_000

# Floats
pi: 3.14159
scientific: 1.5e10
infinity: inf
not_a_number: nan

# Strings (must be quoted)
name: "Alice"
message: "Hello, World!"
unicode: "Hello \u4E16\u754C"

# Binary data
data_b64: b64"SGVsbG8gV29ybGQh"
data_hex: hex"48656c6c6f"

# Timestamps
created: ts"2024-01-15T12:30:45Z"
updated: ts"2024-01-15T12:30:45.123Z"

# Lists (block syntax)
numbers:
  - 1
  - 2
  - 3

# Nested lists
matrix:
  -
    - 1
    - 2
  -
    - 3
    - 4

# Maps (nested structures)
person:
  name: "Bob"
  age: 25
  address:
    street: "123 Main St"
    city: "Portland"
    zip: "97201"

# Inline lists and maps (compact single-line syntax)
inline_list: [1, 2, 3]
inline_map: {x: 10, y: 20}

# Lists of maps
users:
  - name: "Alice"
    role: "admin"
  - name: "Bob"
    role: "user"
```

## Features

### Data Types

- **Null**: `null`
- **Boolean**: `true`, `false`
- **Integer** (i64): `42`, `-123`, `0xFF`, `0b1010`, `0o755`
- **Float** (f64): `3.14`, `1e10`, `.5`, `5.`, `inf`, `nan`
- **String**: `"quoted"` or `'quoted'` (always quoted)
- **Binary**: `b64"base64..."` or `hex"hexdigits..."`
- **Timestamp**: `ts"2024-01-15T12:30:45Z"`
- **List**: Block syntax or inline `[...]`
- **Map**: Block syntax or inline `{...}`

### Syntax Features

- **Indentation-based**: Clean structure without braces
- **Flexible indentation**: First indent (2 spaces, 4 spaces, tabs) defines base unit
- **Line comments**: `#` starts a comment
- **Unquoted keys**: Map keys can be identifiers (`name: "value"`)
- **Quoted keys**: For special characters (`"key-with-dash": "value"`)
- **Inline syntax**: Compact `[...]` and `{...}` for single-line structures
- **Trailing commas**: Allowed in inline lists/maps

### No YAML Gotchas

JAML avoids common YAML pitfalls:

```yaml
# YAML 1.1 (problematic)
NO: Norway    # NO becomes false!
yes: Yemen    # yes becomes true!
```

```jaml
# JAML (explicit)
NO: "Norway"    # NO is an identifier key, "Norway" is a quoted string
yes: "Yemen"    # yes is an identifier key, "Yemen" is a quoted string
```

**All string values must be quoted** - no implicit type conversion!

## Installation

```toml
[dependencies]
jaml = "0.2"
```

For serde support (enabled by default):
```toml
[dependencies]
jaml = { version = "0.2", features = ["serde"] }
```

## API Overview

### Parsing

```rust
use jaml::{parse, Value};

// Parse to AST
let value: Value = jaml::parse(text)?;

// With serde
let config: MyStruct = jaml::from_str(text)?;
```

### Formatting

```rust
use jaml::{format, formatter::Options};

// Format a value
let text = jaml::format(&value);

// With custom options
let opts = Options::new()
    .with_sort_keys(true)
    .with_quote_style(formatter::QuoteStyle::PreferDouble);
let text = jaml::format_with_opts(&value, &opts);

// With serde
let text = jaml::to_string(&my_struct)?;
```

## Documentation

- **[Grammar Specification](GRAMMAR.md)**: Complete EBNF grammar and rules
- **[Examples](examples/)**: Valid and invalid example files
- **[JASN](../jasn/README.md)**: Alternative JSON5-like syntax with same data model

## Differences from YAML

1. **Explicit string values**: All string values must be quoted
2. **Integer/Float distinction**: `42` and `42.0` are different types
3. **Binary type**: Native `b64"..."` and `hex"..."` literals
4. **Timestamp type**: Native `ts"..."` literals
5. **No implicit conversions**: No yes/no/on/off boolean conversion
6. **Flexible indentation**: First indent defines base unit
7. **Single document**: No multi-document support
8. **No anchors/aliases**: No `&anchor` or `*alias`
9. **No tags**: No `!!type` support
10. **Simpler, explicit syntax**: Focused subset with clear rules

## Differences from JASN

**Important:** JAML is **not a superset** of JASN. Unlike YAML/JSON (where JASN is a superset of JSON), JAML and JASN use intentionally **disjoint syntax styles**.

Both share the same data model (`jasn-core::Value`) and can represent identical data, but use different syntax:

| Feature | JASN | JAML |
|---------|------|------|
| Structure | Braces `{}` and brackets `[]` | Indentation-based |
| Multi-line structures | Allowed in `{}` and `[]` | Only via indentation (block syntax) |
| Inline structures | `[...]` and `{...}` with newlines allowed | `[...]` and `{...}` must be single-line |
| Comments | Block `/* */` | Line `#` only |
| String values | Optional quotes | Must be quoted |
| Trailing commas | Everywhere | Only in inline `[]` and `{}` |

**Design Philosophy:** The syntax incompatibility is intentional. Each format has its own consistent style:
- **JASN**: Braces/brackets everywhere, block comments, flexible quoting
- **JAML**: Indentation for structure, line comments, explicit quoting
- **Compatibility**: Via the shared `Value` type - convert between formats through the AST

```rust
// JASN text -> Value -> JAML text
let jasn = r#"{ name: "Alice", age: 30 }"#;
let value = jasn::parse(jasn)?;
let jaml = jaml::format(&value);

// JAML text -> Value -> JASN text
let jaml = "name: \"Alice\"\nage: 30";
let value = jaml::parse(jaml)?;
let jasn = jasn::format(&value);
```

## License

MIT License - see [LICENSE](../LICENSE) file for details.
