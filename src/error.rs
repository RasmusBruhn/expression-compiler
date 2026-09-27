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
    /// A string literal was not properly closed with a double quote
    #[error("String literal not properly closed with a double quote")]
    StringLiteralMissingEnd,
    /// A string literal contains an invalid special character
    #[error("String literal contains an invalid character: '{}'", .0)]
    StringLiteralInvalidCharacter(String),
    /// A character literal was not properly closed with a single quote
    #[error("Character literal not properly closed with a single quote")]
    CharacterLiteralMissingEnd,
    /// A character literal contains too many characters
    #[error("Character literal contains too many characters")]
    CharacterLiteralTooLong,
}
