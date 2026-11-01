//!
//! Structures and methods for handling errors during the compilation of
//! expressions, including line and column information
//!

use std::fmt::Display;

use crate::Type;

/// A specialized `Result` type for operations that can return an `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// Represents an error that occurred during the compilation of an expression,
/// including its position in the source code
#[derive(Debug, Clone, PartialEq)]
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
        if self.line == 0 && self.column == 0 {
            write!(f, "{}", self.error)
        } else {
            write!(
                f,
                "line {}, column {}: {}",
                self.line, self.column, self.error
            )
        }
    }
}

/// Errors during compilation of expressions
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ErrorCore {
    /// No error
    #[error("No error")]
    None,
    /// A string literal contains an invalid escape sequence
    #[error("String or character literal contains an invalid escape sequence: '\\{}'", .0)]
    TextLiteralInvalidEscapeSequence(String),
    /// A string literal contains an invalid digit in escape sequence
    #[error("String or character literal contains an invalid digit in escape sequence: '{}'", .0)]
    TextLiteralInvalidEscapeValueDigit(String),
    /// A string literal contains an escape sequence with a value larger than 255
    #[error("String or character literal contains an escape sequence with a value larger than 255")]
    TextLiteralLargeEscapeValue,
    /// A string literal was not properly closed with a double quote
    #[error("Text literal not properly closed, expected '{}'", .0)]
    TextLiteralMissingEnd(char),
    /// A character literal does not contain exactly one character
    #[error("Character literal does not contain exactly one character")]
    CharacterLiteralLength,
    /// An integer literal is too large to fit in the target type
    #[error("Integer literal is too large to fit in the target type (u64)")]
    IntegerLiteralTooLarge,
    /// An operator contains illegal characters
    #[error("Operator contains illegal characters: '{}'", .0)]
    OperatorIllegalCharacters(String),
    /// Type alias is used multiple times
    #[error("Type alias is used multiple times: '{}'", .0)]
    DuplicateTypeAlias(String),
    /// Type alias has an invalid name
    #[error("Type alias has an invalid name: '{}'", .0)]
    InvalidTypeAliasName(String),
    /// Variable name is used multiple times
    #[error("Variable name is used multiple times: '{}'", .0)]
    DuplicateVariable(String),
    /// Variable has an invalid name
    #[error("Variable has an invalid name: '{}'", .0)]
    InvalidVariableName(String),
    /// Function signature is used multiple times
    #[error("Function signature is used multiple times: '{}({})'", .0, .1.iter().map(|typ| format!("{}", typ)).collect::<Vec<String>>().join(", "))]
    DuplicateFunction(String, Vec<Type>),
    /// Function has an invalid name
    #[error("Function has an invalid name: '{}'", .0)]
    InvalidFunctionName(String),
}
