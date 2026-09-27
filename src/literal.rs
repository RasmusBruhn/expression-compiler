//!
//! Provides structures and methods for extracting string literals from
//! annotated strings.
//!

use crate::{
    Error, ErrorCore, Result,
    annotate::{Annotated, AnnotatedStr},
};

/// A single token extracted from an annotated string, either an unidentified
/// segment or a literal
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token<'a> {
    Unidentified(&'a AnnotatedStr),
    StringLiteral(Annotated<String>),
    CharacterLiteral(Annotated<char>),
    FloatLiteral(Annotated<f64>),
    IntegerLiteral(Annotated<u64>),
    BooleanLiteral(Annotated<bool>),
}

/// Finds all literals (string, character, float, integer, boolean) in the
/// annotated string and returns a vector of tokens containing them
///
/// # Parameters
///
/// str: The annotated string to search for literals
pub(crate) fn find_literals<'a>(str: &'a AnnotatedStr) -> Result<Vec<Token<'a>>> {
    // Find all string literals
    let tokens = find_string_literals(str)?;

    // Find all value literals
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens {
        if let Token::Unidentified(s) = token {
            let new_tokens = find_value_literals(s)?;
            result.extend(new_tokens);
        } else {
            result.push(token);
        }
    }

    return Ok(result);
}

/// Find all string and character literals in the annotated string and returns a
/// vector of tokens (`Unidentified`, `StringLiteral`, or `CharacterLiteral`)
///
/// # Parameters
///
/// s: The annotated string to search for string and character literals
fn find_string_literals<'a>(s: &'a AnnotatedStr) -> Result<Vec<Token<'a>>> {
    /// Active string being parsed
    struct ActiveString {
        /// The current string which has been parsed so far
        current_string: String,
        /// The line number where the active string starts
        line: usize,
        /// The column number where the active string starts
        column: usize,
        /// The escape character to end the string
        escape_character: char,
    }

    let mut result = Vec::new();
    let mut start = 0;
    let mut active_string_opt = None;
    let mut special = false;
    for (i, c) in s.iter().enumerate() {
        // Check if it is currently parsing a string
        if let Some(active_string) = &mut active_string_opt {
            // Handle special character (previous character was '\')
            if special {
                special = false;

            }
        } else {
            // Start a new string or character literal
            if c.value == '"' || c.value == '\'' {
                // Initialize a new active string based on the type of quote encountered
                if c.value == '"' {
                    active_string_opt = Some(ActiveString {
                        current_string: String::new(),
                        line: c.line,
                        column: c.column,
                        escape_character: '"',
                    });
                } else if c.value == '\'' {
                    active_string_opt = Some(ActiveString {
                        current_string: String::new(),
                        line: c.line,
                        column: c.column,
                        escape_character: '\'',
                    });
                }

                // End the previous token
                if start < i {
                    result.push(Token::Unidentified(&s[start..i]));
                }
            }
        }
    }

    return Ok(result);
}

/// Find all value literals (float, integer, boolean) in the annotated string and returns a
/// vector of tokens (`Unidentified`, `FloatLiteral`, `IntegerLiteral`, or `BooleanLiteral`)
///
/// # Parameters
///
/// str: The annotated string to search for value literals
fn find_value_literals<'a>(str: &'a AnnotatedStr) -> Result<Vec<Token<'a>>> {
    todo!()
}
