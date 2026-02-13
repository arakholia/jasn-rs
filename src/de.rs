//! Deserialization of JASN text to Rust values.

use serde::de::Deserialize;

use crate::{Value, parse, value::de};

/// Error type for deserialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Custom deserialization error.
    #[error("custom error: {0}")]
    Custom(String),
    /// Parse error from the JASN parser.
    #[error("parse error: {0}")]
    Parse(#[from] crate::parser::Error),
    /// Deserialization error from value module.
    #[error("deserialization error: {0}")]
    Value(#[from] de::Error),
}

/// Result type for deserialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Deserialize a JASN string into a Rust value.
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let value = parse(s)?;
    Ok(de::from_value(&value)?)
}

/// Deserialize a JASN `Value` into a Rust value.
pub fn from_value<'de, T>(value: &'de Value) -> Result<T>
where
    T: Deserialize<'de>,
{
    Ok(de::from_value(value)?)
}
