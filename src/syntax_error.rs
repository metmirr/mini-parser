/// This module provides an error to be used for rest of the parser.
///
use std::{error, fmt};

/// Represent the grammar error
#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    message: String,
    level: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Error {}", self.level, self.message)
    }
}

impl error::Error for SyntaxError {}

impl SyntaxError {
    /// Create an error for Tokenizer, an error happened during tokenizing
    pub fn tokenizer_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Tokenizer".to_string(),
        }
    }

    /// Create an error for Parser, indicate that an error happened during parsing
    pub fn parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "Parse".to_string(),
        }
    }
}
