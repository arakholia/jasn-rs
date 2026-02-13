//! Serialization of Rust values to JASN text.

use serde::Serialize;

use crate::{formatter, value::ser};

/// Error type for serialization.
pub type Error = ser::Error;

/// Result type for serialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize a Rust value to a JASN string.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let jasn_value = ser::to_value(value)?;
    Ok(formatter::to_string(&jasn_value))
}

/// Serialize a Rust value to a JASN string with pretty formatting.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let jasn_value = ser::to_value(value)?;
    Ok(formatter::to_string_pretty(&jasn_value))
}

/// Serialize a Rust value to a JASN string with custom formatting options.
pub fn to_string_opts<T>(value: &T, options: &formatter::Options) -> Result<String>
where
    T: Serialize,
{
    // TODO: optimize by directly serializing to string instead of going through Value
    let jasn_value = ser::to_value(value)?;
    Ok(formatter::to_string_opts(&jasn_value, options))
}

/// Serialize a Rust value to a JASN `Value`.
pub fn to_value<T>(value: &T) -> Result<crate::Value>
where
    T: Serialize + ?Sized,
{
    ser::to_value(value)
}
