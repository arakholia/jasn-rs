# jasn-core

Core data types for JASN and JAML serialization formats.

This crate provides the shared data model used by both JASN (JSON5-like) and JAML (YAML-like)
serialization formats. Both formats share the same type system and in-memory representation.

## Data Model

```rust
use std::collections::BTreeMap;
use jasn_core::Value;

let mut map = BTreeMap::new();
map.insert("name".to_string(), Value::String("Alice".to_string()));
map.insert("age".to_string(), Value::Int(30));

let value = Value::Map(map);
```

## Types

- `Value`: The main enum representing all possible values
- `Binary`: Wrapper for binary data (`Vec<u8>`)
- `Timestamp`: ISO8601/RFC3339 timestamp with timezone

## Features

- `serde` (default): Enable serde serialization/deserialization support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
jasn-core = "0.2"
```

Or if you want to work with JASN or JAML formats directly:

```toml
[dependencies]
jasn = "0.2"  # for JASN format
# jaml = "0.1"  # for JAML format (coming soon)
```

## License

MIT License - see [LICENSE](../LICENSE) file for details.
