//! Deserialization of JASN text to Rust values.

use serde::de::Deserialize;

use crate::{Value, parser, value::de};

/// Error type for deserialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Parse error from the JASN parser.
    #[error("Parse error: {0}")]
    ParseError(#[from] parser::Error),
    /// Deserialization error from value module.
    #[error("Eeserialization error: {0}")]
    DeserializationError(#[from] de::Error),
}

/// Result type for deserialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Deserialize a JASN string into a Rust value.
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let value = parser::parse(s)?;
    Ok(de::from_value(&value)?)
}

/// Deserialize a JASN `Value` into a Rust value.
pub fn from_value<'de, T>(value: &'de Value) -> Result<T>
where
    T: Deserialize<'de>,
{
    Ok(de::from_value(value)?)
}
