//! Serialization of Rust values to JAML text.

use jasn_core::ser;
use serde::Serialize;

use crate::{Value, formatter};

/// Error type for serialization.
pub type Error = ser::Error;

/// Result type for serialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize a Rust value to a JAML string.
///
/// JAML is inherently indentation-based, so output is always formatted
/// with proper indentation (similar to YAML).
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let jaml_value = ser::to_value(value)?;
    Ok(formatter::format(&jaml_value))
}

/// Serialize a Rust value to a JAML string with pretty formatting.
///
/// **Note:** JAML is inherently indentation-based (like YAML), so this function
/// produces the same output as [`to_string`]. It exists for API consistency
/// with other serialization formats.
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    to_string(value)
}

/// Serialize a Rust value to a JAML string with custom formatting options.
pub fn to_string_opts<T>(value: &T, options: &formatter::Options) -> Result<String>
where
    T: Serialize,
{
    // TODO: optimize by directly serializing to string instead of going through Value
    let jaml_value = ser::to_value(value)?;
    Ok(formatter::format_with_opts(&jaml_value, options))
}

/// Serialize a Rust value to a JAML [`Value`].
pub fn to_value<T>(value: &T) -> Result<Value>
where
    T: Serialize + ?Sized,
{
    ser::to_value(value)
}
