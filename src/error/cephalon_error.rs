use colored::Colorize;
use std::fmt;
use crate::primitives::Format;

pub enum CephalonError {
    PathNotFound {
        attempted: String,
        suggestions: Option<Vec<String>>,
    },
    InvalidFormat {
        attempted: Format,
        suggestions: Option<Vec<Format>>,
    },
    SourceUnavailable(String),
    PermissionDenied(String),
    EmptySource(String),
}

impl fmt::Display for CephalonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CephalonError::PathNotFound { attempted, suggestions } => {
                writeln!(f, "{} Path not found: {}", "ERROR".red(), attempted.red())?;
                if let Some(suggestions) = suggestions {
                    writeln!(f, "{} Did you mean:", "SUGGESTION".truecolor(255, 165, 0))?;
                    for s in suggestions {
                        writeln!(f, "  {} {}", "→".blue(), s.blue())?;
                    }
                }
                Ok(())
            }
            CephalonError::InvalidFormat { attempted, suggestions } => {
                writeln!(f, "{} Invalid format: {:?}", "ERROR".red(), attempted)?;
                if let Some(suggestions) = suggestions {
                    writeln!(f, "{} Did you mean:", "SUGGESTION".truecolor(255, 165, 0))?;
                    for s in suggestions {
                        writeln!(f, "  {} {:?}", "→".blue(), s)?;
                    }
                }
                Ok(())
            }
            CephalonError::SourceUnavailable(src) => {
                writeln!(f, "{} Source unavailable: {}", "ERROR".red(), src.red())
            }
            CephalonError::PermissionDenied(src) => {
                writeln!(f, "{} Permission denied: {}", "ERROR".red(), src.red())
            }
            CephalonError::EmptySource(src) => {
                writeln!(f, "{} Empty source: {}", "WARNING".truecolor(255, 165, 0), src.yellow())
            }
        }
    }
}