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
//! # Example
//! ```
//! use jasn::{parse, to_string_pretty};
//!
//! let jasn_text = r#"
//! {
//!   name: "Alice",
//!   age: 30,
//!   balance: 1234.56,
//!   avatar: b64"SGVsbG8=",
//!   metadata: {
//!     created: "2024-01-15",
//!     stats: {
//!       login_count: 42,
//!       average_score: 91.3,
//!     },
//!   },
//!   scores: [95, 87, 92],
//!   weights: [0.8, 1.0, 0.95],
//! }
//! "#;
//!
//! let value = parse(jasn_text).unwrap();
//! println!("{}", to_string_pretty(&value));
//! ```

#![warn(missing_docs)]

mod binary;
pub use binary::Binary;

mod parser;
pub use parser::{Error as ParseError, Result as ParseResult, parse};

mod value;
pub use value::{Timestamp, Value};

/// Formatting JASN values to strings with custom options.
pub mod formatter;
pub use formatter::{to_string, to_string_pretty};
