use super::Rule;

/// Errors that can occur during parsing.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Error from the pest parser (syntax errors).
    #[error("Parse error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),

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

    /// Unknown binary encoding.
    #[error("Unknown binary encoding: {0}")]
    UnknownBinaryEncoding(String),

    /// Duplicate key in map.
    #[error("Duplicate key in map: {0}")]
    DuplicateKey(String),
}

/// Result type for parsing operations.
pub type Result<T> = std::result::Result<T, Error>;
