use super::Rule;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error from the pest parser (syntax errors).
    #[error("Parse error: {0}")]
    PestError(#[from] Box<pest::error::Error<Rule>>),

    /// Integer parsing or overflow error.
    #[error("Integer parse error: {0}")]
    IntError(#[from] std::num::ParseIntError),

    /// Float parsing error.
    #[error("Float parse error: {0}")]
    FloatError(#[from] std::num::ParseFloatError),

    /// Base64 decoding error.
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    /// Invalid escape sequence in string.
    #[error("Invalid escape sequence")]
    InvalidEscape,

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
    #[error("Unknown binary encoding")]
    UnknownBinaryEncoding,
}

pub type Result<T> = std::result::Result<T, Error>;
