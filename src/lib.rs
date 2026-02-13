//! A Rust library for parsing and formatting JASN (Just Another Serialization Notation).
//!
//! JASN is a human-readable data serialization format similar to JSON but with explicit integer and binary types.
//!
//! # Features
//! 1. **Explicit Integer Types**: Distinguish between integers and floats (2 vs 2.0).
//! 2. **Binary Data**: Support for base64 and hex-encoded binary data
//! 3. **Timestamps**: ISO8601/RFC3339 timestamps with `ts"..."` syntax
//! 4. Permissive syntax, similar to JSON5
//!
//! # JASN Syntax
//!
//! A comprehensive example showing all supported value types:
//!
//! ```jasn
//! {
//!   // Comments are supported
//!   /* Block comments are supported */
//!   null_value: null,
//!   
//!   // Booleans
//!   bool_true: true,
//!   bool_false: false,
//!   
//!   // Integers (explicit type, no decimal point)
//!   integer: 42,
//!   negative: -123,
//!   hex: 0xFF,
//!   binary: 0b1010,
//!   octal: 0o755,
//!   with_underscores: 1_000_000,
//!   
//!   // Floats (always have decimal point or exponent)
//!   float: 3.14,
//!   scientific: 1.5e10,
//!   special_inf: inf,
//!   special_neg_inf: -inf,
//!   special_nan: nan,
//!   
//!   // Strings (double or single quotes)
//!   string_double: "Hello, World!",
//!   string_single: 'Hello, World!',
//!   string_unicode: "Hello \u4E16\u754C",  // Unicode escapes
//!   
//!   // Binary data
//!   binary_hex: h"48656c6c6f",           // Hex encoding
//!   binary_base64: b64"SGVsbG8gV29ybGQ=", // Base64 encoding
//!   
//!   // Timestamps (RFC3339/ISO8601)
//!   timestamp: ts"2024-01-15T12:30:45Z",
//!   timestamp_offset: ts"2024-01-15T12:30:45-05:00",
//!   
//!   // Lists
//!   list: [1, 2, 3, "mixed", true, null],
//!   nested_list: [[1, 2], [3, 4]],
//!   
//!   // Maps (objects)
//!   map: {
//!     unquoted_key: "value",
//!     "quoted key": "also works",
//!     nested: { a: 1, b: 2 },
//!   },
//!   
//!   // Trailing commas allowed
//!   trailing: [1, 2, 3,],
//! }
//! ```
//!
//! # Usage
//!
//! ## AST Manipulation (no serde required)
//!
//! ```
//! use jasn::{parse, format_pretty};
//!
//! let jasn_text = r#"{ name: "Alice", age: 30 }"#;
//! let value = parse(jasn_text).unwrap();
//! println!("{}", format_pretty(&value));
//!
//! // For custom formatting:
//! let opts = jasn::formatter::Options::pretty()
//!     .with_indent("\t");
//! println!("{}", jasn::formatter::format_with_opts(&value, &opts));
//! ```
//!
//! ## Serde Integration (default feature)
//!
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let person = Person { name: "Alice".into(), age: 30 };
//! let jasn_text = jasn::to_string_pretty(&person).unwrap();
//! let parsed: Person = jasn::from_str(&jasn_text).unwrap();
//! ```
//!
//! # Features
//!
//! - `serde` (default): Enable serde serialization/deserialization support

#![warn(missing_docs)]

mod value;
pub use value::{Binary, Timestamp, Value};

pub mod parser;
pub use parser::parse;

pub mod formatter;
pub use formatter::{format, format_pretty};

#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "serde")]
pub use de::{from_str, from_value};
#[cfg(feature = "serde")]
pub use ser::{to_string, to_string_pretty, to_value};
