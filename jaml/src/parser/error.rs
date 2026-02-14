use super::{indent::Style as IndentStyle, parse::PestError};

/// Errors that can occur during parsing.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Error from the pest parser (syntax errors).
    #[error("Parse error: {0}")]
    PestError(#[from] PestError),

    /// Integer parsing or overflow error.
    #[error("Integer parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Float parsing error.
    #[error("Float parse error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Base64 decoding error.
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    /// Invalid escape sequence in string.
    #[error("Invalid escape character: {0}")]
    InvalidEscapeChar(char),

    /// Invalid unicode escape sequence.
    #[error("Invalid unicode escape: {0}")]
    InvalidUnicodeEscape(String),

    /// Invalid unicode codepoint.
    #[error("Invalid unicode codepoint: {0}")]
    InvalidUnicodeCodepoint(u32),

    /// Hex binary with odd number of digits.
    #[error("Hex binary must have even number of digits")]
    OddHexDigits,

    /// Duplicate key in map.
    #[error("Duplicate key in map: {0}")]
    DuplicateKey(String),

    /// Invalid timestamp format.
    #[error("Invalid timestamp '{0}': {1}")]
    InvalidTimestamp(String, String),

    /// Mixed tabs and spaces in indentation base unit.
    #[error("Mixed tabs and spaces in indentation, got {0:?}")]
    MixedIndent(String),

    /// Inconsistent indentation type (switching between spaces and tabs).
    #[error("Inconsistent indentation: expected {0}, got {1}")]
    InconsistentIndentStyle(IndentStyle, IndentStyle),

    /// Invalid indentation (not a multiple of the base unit).
    #[error("Invalid indentation: expected multiple of {0}, got {1}")]
    InvalidIndentCount(IndentStyle, usize),

    /// Unexpected indentation level.
    #[error("Unexpected indentation: expected {0}, got {1}")]
    UnexpectedIndent(usize, usize),

    /// Empty document.
    #[error("Empty document")]
    EmptyDocument,

    /// Missing value for list item or map entry.
    #[error("Missing value at line {0}")]
    MissingValue(usize),
}

/// Result type for parsing operations.
pub type Result<T> = std::result::Result<T, Error>;
