//! Error types for latex2typst

/// Result type alias for latex2typst operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during parsing and conversion
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to parse Markdown
    #[error("Failed to parse markdown: {0}")]
    MarkdownParse(String),

    /// Failed to parse LaTeX
    #[error("Failed to parse LaTeX at position {pos}: {msg}")]
    LatexParse { pos: usize, msg: String },

    /// Unsupported LaTeX command
    #[error("Unsupported LaTeX command '\\{cmd}' at position {pos}")]
    UnsupportedCommand { cmd: String, pos: usize },

    /// Conversion error
    #[error("Conversion error: {0}")]
    ConversionError(String),

    /// Invalid math expression
    #[error("Invalid math expression at position {pos}: {msg}")]
    InvalidMath { pos: usize, msg: String },

    /// Unexpected end of input
    #[error("Unexpected end of input at position {pos}")]
    UnexpectedEof { pos: usize },

    /// Mismatched delimiters
    #[error("Mismatched delimiters at position {pos}: expected {expected}, found {found}")]
    MismatchedDelimiters {
        pos: usize,
        expected: String,
        found: String,
    },
}

impl Error {
    /// Create a LaTeX parse error
    pub fn latex_parse(pos: usize, msg: impl Into<String>) -> Self {
        Error::LatexParse {
            pos,
            msg: msg.into(),
        }
    }

    /// Create an unsupported command error
    pub fn unsupported_command(cmd: impl Into<String>, pos: usize) -> Self {
        Error::UnsupportedCommand {
            cmd: cmd.into(),
            pos,
        }
    }

    /// Create an invalid math error
    pub fn invalid_math(pos: usize, msg: impl Into<String>) -> Self {
        Error::InvalidMath {
            pos,
            msg: msg.into(),
        }
    }

    /// Create an unexpected EOF error
    pub fn unexpected_eof(pos: usize) -> Self {
        Error::UnexpectedEof { pos }
    }

    /// Create a mismatched delimiters error
    pub fn mismatched_delimiters(
        pos: usize,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Error::MismatchedDelimiters {
            pos,
            expected: expected.into(),
            found: found.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::latex_parse(10, "unexpected character");
        assert!(matches!(err, Error::LatexParse { pos: 10, .. }));
    }

    #[test]
    fn test_unsupported_command() {
        let err = Error::unsupported_command("foobar", 20);
        assert!(matches!(err, Error::UnsupportedCommand { pos: 20, .. }));
    }

    #[test]
    fn test_error_display() {
        let err = Error::latex_parse(5, "test error");
        let msg = format!("{}", err);
        assert!(msg.contains("position 5"));
        assert!(msg.contains("test error"));
    }

    #[test]
    fn test_result_type() {
        fn test_fn() -> Result<i32> {
            Ok(42)
        }

        let result = test_fn();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_mismatched_delimiters() {
        let err = Error::mismatched_delimiters(15, "}", "]");
        match err {
            Error::MismatchedDelimiters {
                pos,
                expected,
                found,
            } => {
                assert_eq!(pos, 15);
                assert_eq!(expected, "}");
                assert_eq!(found, "]");
            }
            _ => panic!("Expected MismatchedDelimiters"),
        }
    }
}
