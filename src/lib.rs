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
