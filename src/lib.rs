use colored::Colorize;
use std::fmt;

pub enum CephalonError {
    PathNotFound {
        attempted: String,
        suggestions: Option<Vec<String>>,
    },
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
        }
    }
}