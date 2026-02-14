/// Formatting options for JAML output.
#[derive(Debug, Clone)]
pub struct Options {
    /// Quote style for strings.
    pub quote_style: QuoteStyle,

    /// Binary data encoding preference.
    pub binary_encoding: BinaryEncoding,

    /// Use unquoted keys in maps when possible.
    pub unquoted_keys: bool,

    /// Add leading plus sign to positive numbers (+42, +3.14, +inf).
    pub leading_plus: bool,

    /// Sort map keys alphabetically for consistent output.
    pub sort_keys: bool,

    /// Escape all non-ASCII characters as \uXXXX sequences.
    pub escape_unicode: bool,

    /// Use 'Z' for UTC timestamps instead of '+00:00'.
    pub use_zulu: bool,

    /// Precision for timestamp fractional seconds.
    pub timestamp_precision: TimestampPrecision,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quote_style: QuoteStyle::Double,
            binary_encoding: BinaryEncoding::Base64,
            unquoted_keys: true,
            leading_plus: false,
            sort_keys: true,
            escape_unicode: false,
            use_zulu: true,
            timestamp_precision: TimestampPrecision::Auto,
        }
    }
}

impl Options {
    /// Creates default formatting options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the quote style.
    pub fn with_quote_style(mut self, style: QuoteStyle) -> Self {
        self.quote_style = style;
        self
    }

    /// Sets the binary encoding preference.
    pub fn with_binary_encoding(mut self, encoding: BinaryEncoding) -> Self {
        self.binary_encoding = encoding;
        self
    }

    /// Sets whether to use unquoted keys.
    pub fn with_unquoted_keys(mut self, enable: bool) -> Self {
        self.unquoted_keys = enable;
        self
    }

    /// Sets whether to add leading plus sign to positive numbers.
    pub fn with_leading_plus(mut self, enable: bool) -> Self {
        self.leading_plus = enable;
        self
    }

    /// Sets whether to sort map keys alphabetically.
    pub fn with_sort_keys(mut self, enable: bool) -> Self {
        self.sort_keys = enable;
        self
    }

    /// Sets whether to escape non-ASCII characters as \uXXXX.
    pub fn with_escape_unicode(mut self, enable: bool) -> Self {
        self.escape_unicode = enable;
        self
    }

    /// Sets whether to use 'Z' for UTC timestamps instead of '+00:00'.
    pub fn with_use_zulu(mut self, enable: bool) -> Self {
        self.use_zulu = enable;
        self
    }

    /// Sets the precision for timestamp fractional seconds.
    pub fn with_timestamp_precision(mut self, precision: TimestampPrecision) -> Self {
        self.timestamp_precision = precision;
        self
    }
}

/// Quote style for strings and map keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteStyle {
    /// Always use double quotes: "string"
    Double,

    /// Always use single quotes: 'string'
    Single,

    /// Prefer double quotes, but use single if string contains "
    PreferDouble,
}

/// Binary data encoding preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryEncoding {
    /// Always use base64: b64"..."
    Base64,

    /// Always use hex: hex"..."
    Hex,
}

/// Precision for timestamp fractional seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampPrecision {
    /// Automatically use minimum necessary digits (default).
    Auto,

    /// No fractional seconds (whole seconds only).
    Seconds,

    /// Milliseconds (3 decimal places).
    Milliseconds,

    /// Microseconds (6 decimal places).
    Microseconds,

    /// Nanoseconds (9 decimal places).
    Nanoseconds,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = Options::default();
        assert_eq!(opts.quote_style, QuoteStyle::Double);
        assert_eq!(opts.binary_encoding, BinaryEncoding::Base64);
        assert!(opts.unquoted_keys);
        assert!(!opts.leading_plus);
        assert!(opts.sort_keys);
        assert!(!opts.escape_unicode);
        assert!(opts.use_zulu);
        assert_eq!(opts.timestamp_precision, TimestampPrecision::Auto);
    }

    #[test]
    fn test_builder_pattern() {
        let opts = Options::new()
            .with_quote_style(QuoteStyle::Single)
            .with_binary_encoding(BinaryEncoding::Hex)
            .with_unquoted_keys(false)
            .with_sort_keys(false);

        assert_eq!(opts.quote_style, QuoteStyle::Single);
        assert_eq!(opts.binary_encoding, BinaryEncoding::Hex);
        assert!(!opts.unquoted_keys);
        assert!(!opts.sort_keys);
    }
}
