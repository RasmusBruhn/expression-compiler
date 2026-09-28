//!
//! Structures and methods for handling errors during the compilation of
//! expressions, including line and column information
//!

use std::fmt::Display;

/// A specialized `Result` type for operations that can return an `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// Represents an error that occurred during the compilation of an expression,
/// including its position in the source code
#[derive(Debug, Clone)]
pub struct Error {
    /// The error that occurred
    pub error: ErrorCore,
    /// The line number where the error occurred
    pub line: usize,
    /// The column number where the error occurred
    pub column: usize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line {}, column {}: {}",
            self.line, self.column, self.error
        )
    }
}

/// Errors during compilation of expressions
#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorCore {
    /// No error
    #[error("No error")]
    None,
    /// A string literal contains an invalid escape sequence
    #[error("String or character literal contains an invalid escape sequence: '{}'", .0)]
    TextLiteralInvalidEscapeSequence(String),
    /// A string literal contains an invalid digit in escape sequence
    #[error("String or character literal contains an invalid digit in escape sequence: '{}'", .0)]
    TextLiteralInvalidEscapeValueDigit(String),
    /// A string literal contains an escape sequence with a value larger than 255
    #[error("String or character literal contains an escape sequence with a value larger than 255")]
    TextLiteralLargeEscapeValue(),
    /// A string literal was not properly closed with a double quote
    #[error("Text literal not properly closed, expected '{}'", .0)]
    TextLiteralMissingEnd(char),
    /// A character literal does not contain exactly one character
    #[error("Character literal does not contain exactly one character")]
    CharacterLiteralLength,
}
