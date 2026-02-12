/// Formatting options for JASN output.
#[derive(Debug, Clone)]
pub struct Options {
    /// Indentation string (e.g., "  " or "\t"). Empty string means compact output.
    pub indent: String,

    /// Add trailing commas to lists and maps.
    pub trailing_commas: bool,

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
}

impl Default for Options {
    fn default() -> Self {
        Self::pretty()
    }
}

impl Options {
    /// Creates options for compact output.
    pub fn compact() -> Self {
        Self {
            indent: String::new(),
            trailing_commas: false,
            quote_style: QuoteStyle::Double,
            binary_encoding: BinaryEncoding::Base64,
            unquoted_keys: true,
            leading_plus: false,
            sort_keys: false,
            escape_unicode: true,
        }
    }

    /// Creates options for pretty-printed output.
    pub fn pretty() -> Self {
        Self {
            indent: "  ".to_string(),
            trailing_commas: true,
            quote_style: QuoteStyle::Double,
            binary_encoding: BinaryEncoding::Base64,
            unquoted_keys: true,
            leading_plus: false,
            sort_keys: true,
            escape_unicode: false,
        }
    }

    /// Sets the indentation string.
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }

    /// Sets whether to use trailing commas.
    pub fn with_trailing_commas(mut self, enable: bool) -> Self {
        self.trailing_commas = enable;
        self
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

    /// Always use hex: h"..."
    Hex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_options() {
        let opts = Options::compact();
        assert!(opts.indent.is_empty());
        assert!(!opts.trailing_commas);
        assert!(opts.unquoted_keys);
    }

    #[test]
    fn test_pretty_options() {
        let opts = Options::pretty();
        assert_eq!(opts.indent, "  ");
        assert!(opts.trailing_commas);
        assert!(opts.unquoted_keys);
    }

    #[test]
    fn test_builder_pattern() {
        let opts = Options::compact()
            .with_indent("\t")
            .with_trailing_commas(true)
            .with_quote_style(QuoteStyle::Single);

        assert_eq!(opts.indent, "\t");
        assert!(opts.trailing_commas);
        assert_eq!(opts.quote_style, QuoteStyle::Single);
    }
}
