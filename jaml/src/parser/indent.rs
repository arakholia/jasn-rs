use super::{Error, Result};

/// Tracks indentation style (spaces or tabs) and base unit size
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Spaces(usize), // Number of spaces per indent level
    Tabs(usize),   // Number of tabs per indent level
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Spaces(n) => write!(f, "{:?}", " ".repeat(*n)),
            Style::Tabs(n) => write!(f, "{:?}", "\t".repeat(*n)),
        }
    }
}

/// Indentation tracker that detects and validates indentation
#[derive(Debug)]
pub struct Tracker {
    style: Option<Style>,
}

impl Tracker {
    pub fn new() -> Self {
        Self { style: None }
    }

    /// Validate and track indentation for a line
    /// Returns the indent level (0, 1, 2, ...) if valid
    pub fn validate(&mut self, indent_str: &str) -> Result<usize> {
        if indent_str.is_empty() {
            return Ok(0);
        }

        // Check if it mixes spaces and tabs
        let has_spaces = indent_str.contains(' ');
        let has_tabs = indent_str.contains('\t');

        if has_spaces && has_tabs {
            return Err(Error::MixedIndent(indent_str.to_string()));
        }

        match self.style {
            None => {
                // First indent - establish the base unit
                if has_tabs {
                    let num_tabs = indent_str.len();
                    self.style = Some(Style::Tabs(num_tabs));
                    Ok(1) // First indent level
                } else {
                    let num_spaces = indent_str.len();
                    self.style = Some(Style::Spaces(num_spaces));
                    Ok(1) // First indent level
                }
            }
            Some(Style::Spaces(base_unit)) => {
                if has_tabs {
                    return Err(Error::InconsistentIndentStyle(
                        Style::Spaces(base_unit),
                        Style::Tabs(indent_str.len()),
                    ));
                }

                let num_spaces = indent_str.len();
                if num_spaces % base_unit != 0 {
                    return Err(Error::InvalidIndentCount(
                        Style::Spaces(base_unit),
                        num_spaces,
                    ));
                }

                Ok(num_spaces / base_unit)
            }
            Some(Style::Tabs(base_unit)) => {
                if has_spaces {
                    return Err(Error::InconsistentIndentStyle(
                        Style::Tabs(base_unit),
                        Style::Spaces(indent_str.len()),
                    ));
                }

                let num_tabs = indent_str.len();
                if num_tabs % base_unit != 0 {
                    return Err(Error::InvalidIndentCount(Style::Tabs(base_unit), num_tabs));
                }

                Ok(num_tabs / base_unit)
            }
        }
    }
}
