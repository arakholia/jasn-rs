use super::{Error, Result};

/// Indentation character type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Hard, // Tab character
    Soft, // Space character
}

impl Tab {
    fn as_char(&self) -> char {
        match self {
            Tab::Hard => '\t',
            Tab::Soft => ' ',
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        if s.contains('\t') && s.contains(' ') {
            None // Mixed
        } else if s.contains('\t') {
            Some(Tab::Hard)
        } else if s.contains(' ') {
            Some(Tab::Soft)
        } else {
            None // Empty
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Hard => write!(f, "{:?}", self.as_char()),
            Tab::Soft => write!(f, "{:?}", self.as_char()),
        }
    }
}

/// Tracks indentation style and base unit size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub count: usize,
    pub tab: Tab,
}

impl Style {
    pub fn new(count: usize, tab: Tab) -> Self {
        Self { count, tab }
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent_str = self.tab.as_char().to_string().repeat(self.count);
        write!(f, "{:?}", indent_str)
    }
}

/// Indentation tracker that detects and validates indentation
#[derive(Debug, Default)]
pub struct Tracker {
    style: Option<Style>,
}

impl Tracker {
    /// Validate and track indentation for a line
    /// Returns the indent level (0, 1, 2, ...) if valid
    pub fn validate(&mut self, indent_str: &str) -> Result<usize> {
        if indent_str.is_empty() {
            return Ok(0);
        }

        // Check if it mixes spaces and tabs
        let tab =
            Tab::from_str(indent_str).ok_or_else(|| Error::MixedIndent(indent_str.to_string()))?;

        match self.style {
            None => {
                // First indent - establish the base unit
                let count = indent_str.len();
                self.style = Some(Style::new(count, tab));
                Ok(1) // First indent level
            }
            Some(style) => {
                // Check consistency
                if tab != style.tab {
                    return Err(Error::InconsistentIndentTab(style.tab, tab));
                }

                let count = indent_str.len();
                if !count.is_multiple_of(style.count) {
                    return Err(Error::InvalidIndentCount(style.count, count));
                }

                Ok(count / style.count)
            }
        }
    }
}
