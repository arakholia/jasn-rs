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
//! # Usage
//!
//! ## AST Manipulation (no serde required)
//!
//! ```
//! use jasn::{parse, formatter};
//!
//! let jasn_text = r#"{ name: "Alice", age: 30 }"#;
//! let value = parse(jasn_text).unwrap();
//! println!("{}", formatter::to_string_pretty(&value));
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

mod parser;
pub use parser::{Error as ParseError, Result as ParseResult, parse};

/// Formatting options for JASN output.
pub mod formatter;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

#[cfg(feature = "serde")]
pub use de::{Error as DeserializeError, Result as DeserializeResult, from_str, from_value};
#[cfg(feature = "serde")]
pub use ser::{
    Error as SerializeError, Result as SerializeResult, to_string, to_string_opts,
    to_string_pretty, to_value,
};
