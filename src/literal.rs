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
    /// Active character value escape sequence being parsed
    struct ActiveValue {
        /// The current value of the escape sequence
        value: u32,
        /// The number of remaining digits to parse in the escape sequence
        remaining_digits: usize,
        /// The base/radix of the escape sequence
        radix: u32,
        /// The line number where the escape sequence starts
        start_line: usize,
        /// The column number where the escape sequence starts
        start_column: usize,
    }

    /// Active string being parsed
    struct ActiveString {
        /// The current string which has been parsed so far
        current_string: String,
        /// The escape character to end the string
        escape_character: char,
        /// Indicates if the string is a raw string (ignores escape sequences)
        raw: bool,
        /// Indicates if the next character is part of an escape sequence
        special: bool,
        /// The active value escape sequence being parsed, if any
        active_value_opt: Option<ActiveValue>,
        /// The line number where the active string starts
        line: usize,
        /// The column number where the active string starts
        column: usize,
    }

    let mut result = Vec::new();
    let mut start = 0;
    let mut active_string_opt: Option<ActiveString> = None;
    let mut raw_string = false;
    for (i, c) in s.iter().enumerate() {
        // Check if it is currently parsing a string
        if let Some(mut active_string) = active_string_opt.take() {
            if active_string.special {
                // Special character
                match c.value {
                    'a' => active_string.current_string.push('\x07'), // Alert (Beed, Bell)
                    'b' => active_string.current_string.push('\x08'), // Backspace
                    'e' => active_string.current_string.push('\x1B'), // Escape character
                    'f' => active_string.current_string.push('\x0C'), // Formfeed Page Break
                    'n' => active_string.current_string.push('\x0A'), // Newline (Line Feed)
                    'r' => active_string.current_string.push('\x0D'), // Carriage Return
                    't' => active_string.current_string.push('\x09'), // Horizontal Tab
                    'v' => active_string.current_string.push('\x0B'), // Vertical Tab
                    '\\' => active_string.current_string.push('\\'),  // Backslash
                    '\'' => active_string.current_string.push('\''),  // Single quote
                    '"' => active_string.current_string.push('"'),    // Double quote
                    '?' => active_string.current_string.push('?'),    // Question mark
                    'x' => {
                        active_string.active_value_opt = Some(ActiveValue {
                            value: 0,
                            remaining_digits: 2,
                            radix: 16,
                            start_line: c.line,
                            start_column: c.column,
                        })
                    }
                    _ => {
                        if c.value.is_digit(8) {
                            active_string.active_value_opt = Some(ActiveValue {
                                value: c.value.to_digit(8).unwrap(),
                                remaining_digits: 2,
                                radix: 8,
                                start_line: c.line,
                                start_column: c.column,
                            });
                        } else {
                            return Err(Error {
                                error: ErrorCore::TextLiteralInvalidEscapeSequence(
                                    c.value.to_string(),
                                ),
                                line: c.line,
                                column: c.column,
                            });
                        }
                    }
                }

                active_string.special = false;
            } else if let Some(mut active_value) = active_string.active_value_opt.take() {
                // Special value
                if !c.value.is_digit(active_value.radix) {
                    return Err(Error {
                        error: ErrorCore::TextLiteralInvalidEscapeValueDigit(c.value.to_string()),
                        line: c.line,
                        column: c.column,
                    });
                }

                // Update value with new digit
                active_value.value = active_value.value * active_value.radix
                    + c.value.to_digit(active_value.radix).unwrap();
                active_value.remaining_digits -= 1;

                // Finish value
                if active_value.remaining_digits == 0 {
                    // Make sure value is not too large
                    if active_value.value > 255 {
                        return Err(Error {
                            error: ErrorCore::TextLiteralLargeEscapeValue(),
                            line: c.line,
                            column: c.column,
                        });
                    }

                    // Add to result
                    active_string
                        .current_string
                        .push(active_value.value as u8 as char);
                }

                active_string.active_value_opt = Some(active_value);
            } else if c.value == active_string.escape_character {
                // End of active string literal
                if active_string.escape_character == '"' {
                    result.push(Token::StringLiteral(Annotated {
                        value: active_string.current_string,
                        line: active_string.line,
                        column: active_string.column,
                    }));
                } else {
                    // Ensure character literal has exactly one character
                    if active_string.current_string.chars().count() != 1 {
                        return Err(Error {
                            error: ErrorCore::CharacterLiteralLength,
                            line: active_string.line,
                            column: active_string.column,
                        });
                    }

                    result.push(Token::CharacterLiteral(Annotated {
                        value: active_string.current_string.chars().next().unwrap(),
                        line: active_string.line,
                        column: active_string.column,
                    }));
                }

                start = i + 1;
                continue;
            } else if !active_string.raw && c.value == '\\' {
                // Start of an escape sequence
                active_string.special = true;
            } else {
                // Regular character, add to current string
                active_string.current_string.push(c.value);
            }

            active_string_opt = Some(active_string);
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
                        raw: raw_string,
                        special: false,
                        active_value_opt: None,
                    });
                } else if c.value == '\'' {
                    active_string_opt = Some(ActiveString {
                        current_string: String::new(),
                        line: c.line,
                        column: c.column,
                        escape_character: '\'',
                        raw: raw_string,
                        special: false,
                        active_value_opt: None,
                    });
                }

                // End the previous token
                if start < i {
                    result.push(Token::Unidentified(&s[start..i]));
                }
            } else if c.value == 'r' {
                // Mark the upcoming string as a raw string literal
                raw_string = true;

                continue;
            }

            // Stop marking the upcoming string as raw
            raw_string = false;
        }
    }

    // Make sure literal was properly closed before returning the result
    if let Some(active_string) = active_string_opt {
        return Err(Error {
            error: ErrorCore::TextLiteralMissingEnd(active_string.escape_character),
            line: active_string.line,
            column: active_string.column,
        });
    }

    // Add last unidentified token if any
    if start < s.len() {
        result.push(Token::Unidentified(&s[start..]));
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
