//! A Rust library for parsing and formatting JAML (Just Another Markup Language).
//!
//! JAML is a human-readable data serialization format similar to YAML but with explicit integer and binary types.
//! It shares the same data model as JASN, providing a YAML-like syntax as an alternative to the JSON5-like JASN format.
//!
//! # Features
//! 1. **Explicit Integer Types**: Distinguish between integers and floats (2 vs 2.0).
//! 2. **Binary Data**: Support for base64 and hex-encoded binary data
//! 3. **Timestamps**: ISO8601/RFC3339 timestamps
//! 4. **YAML-inspired syntax**: Indentation-based structure, cleaner appearance
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use jaml::{parse, format};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let value = parse(r#"
//! name: "Alice"
//! age: 30
//! balance: 1234.56
//! data: b64"SGVsbG8="
//! tags:
//!   - "rust"
//!   - "yaml"
//!   - "parser"
//!     "#)?;
//!     
//!     println!("{:#?}", value);
//!     
//!     // Format back to JAML
//!     let formatted = format(&value);
//!     println!("{}", formatted);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

// Re-export core types
pub use jasn_core::{Binary, Timestamp, Value};

pub mod formatter;
mod parser;

pub use formatter::{format, format_with_opts};
pub use parser::{Error as ParseError, Result as ParseResult, parse};
