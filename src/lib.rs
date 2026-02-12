//! A Rust library for parsing and formatting JASN (Just Another Serialization Notation).
//!
//! JASN is a human-readable data serialization format similar to JSON but with explicit integer and binary types.
//!
//! # Features
//! 1. **Explicit Integer Types**: Distinguish between integers and floats (2 vs 2.0).
//! 2. **Binary Data**: Support for base64 and hex-encoded binary data
//! 3. Permissive syntax, similar to JSON5

mod binary;
pub use binary::Binary;

mod parser;
pub use parser::{Error as ParseError, Result as ParseResult, parse};

mod value;
pub use value::Value;

pub mod formatter;
pub use formatter::{to_string, to_string_pretty};
