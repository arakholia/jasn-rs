//! Parse JASN text into a [`Value`].
//!
//! The main entry point is [`parse`], which parses a string into a [`Value`].
//!
//! ```
//! use jasn::parse;
//!
//! let value = parse(r#"{ name: "Alice", age: 30 }"#).unwrap();
//! assert!(value.is_map());
//! ```

use crate::Value;

mod error;
mod parse;

pub use error::{Error, Result};

/// Parse a JASN string into a [`Value`].
pub fn parse(input: &str) -> Result<Value> {
    parse::parse_impl(input)
}
