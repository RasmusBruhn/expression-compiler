//!
//! Provides structures and methods for extracting string literals from
//! annotated strings.
//!

use crate::{
    Error, ErrorCore, Result,
    annotate::{Annotated, AnnotatedStr, AnnotatedString},
};

/// A single token extracted from an annotated string, either an unidentified
/// segment or a literal
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Unidentified(AnnotatedString),
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
/// s: The annotated string to search for literals
pub(crate) fn find_literals(s: &AnnotatedStr) -> Result<Vec<Token>> {
    // Find all string literals
    let tokens = find_string_literals(s)?;

    // Find all value literals
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens {
        if let Token::Unidentified(s) = token {
            let new_tokens = find_value_literals(&s)?;
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
fn find_string_literals(s: &AnnotatedStr) -> Result<Vec<Token>> {
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
                            error: ErrorCore::TextLiteralLargeEscapeValue,
                            line: c.line,
                            column: c.column,
                        });
                    }

                    // Add to result
                    active_string
                        .current_string
                        .push(active_value.value as u8 as char);

                    active_string.active_value_opt = None;
                } else {
                    active_string.active_value_opt = Some(active_value);
                }
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
                let end = if raw_string { i - 1 } else { i };
                if start < end {
                    result.push(Token::Unidentified(AnnotatedString::from_str(
                        &s[start..end],
                    )));
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
        result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..])));
    }

    return Ok(result);
}

/// Find all value literals (float, integer, boolean) in the annotated string and returns a
/// vector of tokens (`Unidentified`, `FloatLiteral`, `IntegerLiteral`, or `BooleanLiteral`)
///
/// # Parameters
///
/// s: The annotated string to search for value literals
fn find_value_literals(s: &AnnotatedStr) -> Result<Vec<Token>> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut allow_literal = true;
    let mut iterator = s.iter().enumerate();
    while let Some((i, c)) = iterator.next() {
        // Attempt to parse a literal
        if allow_literal {
            let (skip_count, literal) = if let Some(parsed) = parse_float_literal(&s[i..])? {
                (
                    parsed.count,
                    Token::FloatLiteral(Annotated {
                        value: parsed.value,
                        line: c.line,
                        column: c.column,
                    }),
                )
            } else if let Some(parsed) = parse_int_literal(&s[i..], false)? {
                (
                    parsed.count,
                    Token::IntegerLiteral(Annotated {
                        value: parsed.value,
                        line: c.line,
                        column: c.column,
                    }),
                )
            } else if let Some(parsed) = parse_bool_literal(&s[i..])? {
                (
                    parsed.count,
                    Token::BooleanLiteral(Annotated {
                        value: parsed.value,
                        line: c.line,
                        column: c.column,
                    }),
                )
            } else {
                allow_literal = !is_invalid_literal_terminator(c.value);
                continue;
            };

            // Add unidentified token for characters before the literal and the literal itself and update the start position
            if start < i {
                result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..i])));
            }
            result.push(literal);
            start = i + skip_count;
            iterator.nth(skip_count - 1);

            allow_literal = false;
        } else {
            // Check if next character could be the start of a literal (the
            // previous character must not be able to be part of a variable
            // name)
            allow_literal = !is_invalid_literal_terminator(c.value);
        }
    }

    // Get the last unidentified element
    if start < s.len() {
        result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..])));
    }

    return Ok(result);
}

/// Return value for literal parsing
#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedLiteral<T>
where
    T: Copy + PartialEq,
{
    /// The parsed literal value
    value: T,
    /// The number of characters used for the literal
    count: usize,
}

/// Attempts to parse a float literal from the annotated string, it must start
/// at the start of the string slice but does not need to consume the entire
/// slice
///
/// # Parameters
///
/// s: The annotated string slice to parse the float literal from
fn parse_float_literal(s: &AnnotatedStr) -> Result<Option<ParsedLiteral<f64>>> {
    // Make sure there is at least one digit at the start of the string slice or a dot
    if s.is_empty()
        || !(s[0].value.is_digit(10)
            || (s[0].value == '.' && s.len() > 1 && s[1].value.is_digit(10)))
    {
        return Ok(None);
    }

    // Parse value
    let mut value = 0.0;
    let mut divisor = None;
    let mut count = s.len();
    let mut iterator = s.iter().enumerate();
    while let Some((i, c)) = iterator.next() {
        if c.value.is_digit(10) {
            // Handle digit
            let digit = c.value.to_digit(10).unwrap() as f64;
            if let Some(d) = &mut divisor {
                value += digit / *d;
                *d *= 10.0;
            } else {
                value = value * 10.0 + digit;
            }
        } else if c.value == '.' {
            // Handle dot
            if let Some(_) = divisor {
                count = i;
                break;
            }

            divisor = Some(10.0);
        } else if c.value == 'e' || c.value == 'E' {
            // Detect exponential
            let (sign, begin_exponent) = if s.len() > i + 1 && s[i + 1].value == '+' {
                (1.0, i + 2)
            } else if s.len() > i + 1 && s[i + 1].value == '-' {
                (-1.0, i + 2)
            } else {
                (1.0, i + 1)
            };

            // Parse the exponent part of the float literal
            let exponent = match parse_int_literal(&s[begin_exponent..], true) {
                Ok(value) => {
                    if let Some(value) = value {
                        value
                    } else {
                        count = i;
                        break;
                    }
                }
                Err(value) => {
                    if value.error == ErrorCore::IntegerLiteralTooLarge {
                        let exponent_length = s[begin_exponent..]
                            .iter()
                            .enumerate()
                            .skip_while(|(_, c)| c.value.is_digit(10))
                            .next()
                            .map_or(s[begin_exponent..].len(), |(i, _)| i);

                        ParsedLiteral {
                            value: u64::MAX,
                            count: exponent_length,
                        }
                    } else {
                        return Err(value);
                    }
                }
            };

            value *= 10.0_f64.powf((exponent.value as f64) * sign);
            divisor = Some(1.0);
            count = begin_exponent + exponent.count;
            break;
        } else {
            // End of the float literal
            count = i;
            break;
        }
    }

    // Make sure it is not an integer
    if let None = divisor {
        return Ok(None);
    }

    // Make sure the next character is not alphanumeric, which would make it an invalid integer literal
    if count < s.len() && is_invalid_literal_terminator(s[count].value) {
        return Ok(None);
    }

    Ok(Some(ParsedLiteral { value, count }))
}

/// Attempts to parse an integer literal from the annotated string, it must start
/// at the start of the string slice but does not need to consume the entire
/// slice
///
/// # Parameters
///
/// s: The annotated string slice to parse the integer literal from
///
/// only_decimal: If true, only decimal literals are allowed
fn parse_int_literal(s: &AnnotatedStr, only_decimal: bool) -> Result<Option<ParsedLiteral<u64>>> {
    let mut radix = 0;
    let mut skip_count = 0;

    if s.len() >= 3 && s[0].value == '0' {
        if s[1].value == 'x' || s[1].value == 'X' {
            radix = 16;
            skip_count = 2;
        } else if s[1].value == 'o' || s[1].value == 'O' {
            radix = 8;
            skip_count = 2;
        } else if s[1].value == 'b' || s[1].value == 'B' {
            radix = 2;
            skip_count = 2;
        }
    } else if s.len() >= 1 {
        radix = 10;
        skip_count = 0;
    } else {
        return Ok(None);
    }

    // Make sure there is at least one digit after the prefix
    if !s[skip_count].value.is_digit(radix) || (only_decimal && radix != 10) {
        return Ok(None);
    }

    // Parse the literal
    let mut value: u64 = 0;
    let mut count = s.len();
    for (i, c) in s.iter().enumerate().skip(skip_count) {
        // Stop the literal
        if !c.value.is_digit(radix) {
            count = i;
            break;
        }

        // Make sure value is not too large
        let new_digit = c.value.to_digit(radix).unwrap() as u64;
        if value > (u64::MAX - new_digit) / (radix as u64) {
            return Err(Error {
                error: ErrorCore::IntegerLiteralTooLarge,
                line: c.line,
                column: c.column,
            });
        }

        // Accumulate the digit value
        value = value * (radix as u64) + c.value.to_digit(radix).unwrap() as u64;
    }

    // Make sure the next character is not alphanumeric, which would make it an invalid integer literal
    if count < s.len() && is_invalid_literal_terminator(s[count].value) {
        return Ok(None);
    }

    return Ok(Some(ParsedLiteral { value, count }));
}

/// Attempts to parse a boolean literal from the annotated string, it must start
/// at the start of the string slice but does not need to consume the entire
/// slice
///
/// # Parameters
///
/// s: The annotated string slice to parse the integer literal from
fn parse_bool_literal(s: &AnnotatedStr) -> Result<Option<ParsedLiteral<bool>>> {
    return Ok(
        if s.len() >= 4
            && s[0].value == 't'
            && s[1].value == 'r'
            && s[2].value == 'u'
            && s[3].value == 'e'
        {
            if s.len() > 4 && is_invalid_literal_terminator(s[4].value) {
                None
            } else {
                Some(ParsedLiteral {
                    value: true,
                    count: 4,
                })
            }
        } else if s.len() >= 5
            && s[0].value == 'f'
            && s[1].value == 'a'
            && s[2].value == 'l'
            && s[3].value == 's'
            && s[4].value == 'e'
        {
            if s.len() > 5 && is_invalid_literal_terminator(s[5].value) {
                None
            } else {
                Some(ParsedLiteral {
                    value: false,
                    count: 5,
                })
            }
        } else {
            None
        },
    );
}

/// Checks whether a character is an invalid terminator for a value literal
fn is_invalid_literal_terminator(c: char) -> bool {
    return c.is_alphanumeric() || c == '_' || c == '.';
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotate::AnnotatedString;

    mod boolean_literal {
        use super::*;

        #[test]
        fn single_true() {
            let s = AnnotatedString::new("true");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::BooleanLiteral(Annotated {
                    value: true,
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn single_false() {
            let s = AnnotatedString::new("false");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::BooleanLiteral(Annotated {
                    value: false,
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn imbedded_true() {
            let s = AnnotatedString::new("a true b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::BooleanLiteral(Annotated {
                        value: true,
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[6..]))
                ]
            );
        }

        #[test]
        fn imbedded_false() {
            let s = AnnotatedString::new("a false b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::BooleanLiteral(Annotated {
                        value: false,
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[7..]))
                ]
            );
        }

        #[test]
        fn invalid_true_begin() {
            let s = AnnotatedString::new("atrue");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn invalid_false_begin() {
            let s = AnnotatedString::new("afalse");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn invalid_true_end() {
            let s = AnnotatedString::new("truea");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn invalid_false_end() {
            let s = AnnotatedString::new("falsea");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }
    }

    mod integer_literal {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("1234567890");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 1234567890,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a 1234567890 b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::IntegerLiteral(Annotated {
                        value: 1234567890,
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[12..]))
                ]
            );
        }

        #[test]
        fn invalid_start() {
            let s = AnnotatedString::new("a1234567890");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn invalid_end() {
            let s = AnnotatedString::new("1234567890a");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn hexadecimal_lowercase() {
            let s = AnnotatedString::new("0x123abcdef");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 0x123abcdef,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn hexadecimal_uppercase() {
            let s = AnnotatedString::new("0X123ABCDEF");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 0x123abcdef,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn binary_lowercase() {
            let s = AnnotatedString::new("0b101010");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 0b101010,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn binary_uppercase() {
            let s = AnnotatedString::new("0B101010");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 0b101010,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn too_large() {
            let s = AnnotatedString::new("123456789012345678901234567890");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::IntegerLiteralTooLarge,
                    line: 1,
                    column: 21
                }
            );
        }
    }

    mod float_literal {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("123.456");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 123.456,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a 123.456 b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::FloatLiteral(Annotated {
                        value: 123.456,
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[9..]))
                ]
            );
        }

        #[test]
        fn invalid_start() {
            let s = AnnotatedString::new("a123.456");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn invalid_end() {
            let s = AnnotatedString::new("123.456a");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s)),]
            );
        }

        #[test]
        fn start_dot() {
            let s = AnnotatedString::new(".456");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 0.456,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn end_dot() {
            let s = AnnotatedString::new("123.");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 123.0,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_lowercase() {
            let s = AnnotatedString::new("123.456e2");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 12345.6,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_uppercase() {
            let s = AnnotatedString::new("123.456E2");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 12345.6,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_neg() {
            let s = AnnotatedString::new("123.456e-2");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 1.23456,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_pos() {
            let s = AnnotatedString::new("123.456e+2");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 12345.6,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_small() {
            let s = AnnotatedString::new("123.456e-1000");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 0.0,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_large() {
            let s = AnnotatedString::new("123.456e+1000");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: f64::INFINITY,
                    line: 1,
                    column: 1
                })]
            );
        }

        #[test]
        fn scientific_no_dot() {
            let s = AnnotatedString::new("123e2");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::FloatLiteral(Annotated {
                    value: 12300.0,
                    line: 1,
                    column: 1
                })]
            );
        }
    }

    mod character_literal {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("'a'");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::CharacterLiteral(Annotated {
                    value: 'a',
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a 'a' b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::CharacterLiteral(Annotated {
                        value: 'a',
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[5..]))
                ]
            );
        }

        #[test]
        fn special() {
            let s = AnnotatedString::new("'\\''");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::CharacterLiteral(Annotated {
                    value: '\'',
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn hexadecimal() {
            let s = AnnotatedString::new("'\\x41'");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::CharacterLiteral(Annotated {
                    value: 'A',
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn octal() {
            let s = AnnotatedString::new("'\\101'");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::CharacterLiteral(Annotated {
                    value: 'A',
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn raw() {
            let s = AnnotatedString::new("r'\\'");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::CharacterLiteral(Annotated {
                    value: '\\',
                    line: 1,
                    column: 2
                }),]
            );
        }

        #[test]
        fn too_short() {
            let s = AnnotatedString::new("''");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::CharacterLiteralLength,
                    line: 1,
                    column: 1
                }
            );
        }

        #[test]
        fn too_long() {
            let s = AnnotatedString::new("'ab'");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::CharacterLiteralLength,
                    line: 1,
                    column: 1
                }
            );
        }

        #[test]
        fn not_ended() {
            let s = AnnotatedString::new("'a");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralMissingEnd('\''),
                    line: 1,
                    column: 1
                }
            );
        }

        #[test]
        fn invalid_escape_sequence() {
            let s = AnnotatedString::new("'\\z'");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralInvalidEscapeSequence("z".to_string()),
                    line: 1,
                    column: 3
                }
            );
        }

        #[test]
        fn invalid_escape_digit() {
            let s = AnnotatedString::new("'\\xt'");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralInvalidEscapeValueDigit("t".to_string()),
                    line: 1,
                    column: 4
                }
            );
        }

        #[test]
        fn large_escape_value() {
            let s = AnnotatedString::new("'\\777'");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralLargeEscapeValue,
                    line: 1,
                    column: 5
                }
            );
        }
    }

    mod string_literal {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("\"a\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "a".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a \"a\" b");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                    Token::StringLiteral(Annotated {
                        value: "a".to_string(),
                        line: 1,
                        column: 3
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[5..]))
                ]
            );
        }

        #[test]
        fn special() {
            let s = AnnotatedString::new("\"\\\"\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "\"".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn hexadecimal() {
            let s = AnnotatedString::new("\"\\x41\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "A".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn octal() {
            let s = AnnotatedString::new("\"\\101\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "A".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn raw() {
            let s = AnnotatedString::new("r\"\\\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "\\".to_string(),
                    line: 1,
                    column: 2
                }),]
            );
        }

        #[test]
        fn long() {
            let s = AnnotatedString::new("\"ab\"");
            let result = find_literals(&s).unwrap();

            assert_eq!(
                result,
                vec![Token::StringLiteral(Annotated {
                    value: "ab".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn not_ended() {
            let s = AnnotatedString::new("\"a");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralMissingEnd('"'),
                    line: 1,
                    column: 1
                }
            );
        }

        #[test]
        fn invalid_escape_sequence() {
            let s = AnnotatedString::new("\"\\z\"");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralInvalidEscapeSequence("z".to_string()),
                    line: 1,
                    column: 3
                }
            );
        }

        #[test]
        fn invalid_escape_digit() {
            let s = AnnotatedString::new("\"\\xt\"");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralInvalidEscapeValueDigit("t".to_string()),
                    line: 1,
                    column: 4
                }
            );
        }

        #[test]
        fn large_escape_value() {
            let s = AnnotatedString::new("\"\\777\"");
            let result = find_literals(&s).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::TextLiteralLargeEscapeValue,
                    line: 1,
                    column: 5
                }
            );
        }
    }

    #[test]
    fn empty() {
        let s = AnnotatedString::new("");
        let result = find_literals(&s).unwrap();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn multiple() {
        let s = AnnotatedString::new("a \"string\"\nb 'c'\nc 123\nd 4.56\ne true");
        let result = find_literals(&s).unwrap();

        assert_eq!(
            result,
            vec![
                Token::Unidentified(AnnotatedString::from_str(&s[0..2])),
                Token::StringLiteral(Annotated {
                    value: "string".to_string(),
                    line: 1,
                    column: 3
                }),
                Token::Unidentified(AnnotatedString::from_str(&s[10..13])),
                Token::CharacterLiteral(Annotated {
                    value: 'c',
                    line: 2,
                    column: 3
                }),
                Token::Unidentified(AnnotatedString::from_str(&s[16..19])),
                Token::IntegerLiteral(Annotated {
                    value: 123,
                    line: 3,
                    column: 3
                }),
                Token::Unidentified(AnnotatedString::from_str(&s[22..25])),
                Token::FloatLiteral(Annotated {
                    value: 4.56,
                    line: 4,
                    column: 3
                }),
                Token::Unidentified(AnnotatedString::from_str(&s[29..32])),
                Token::BooleanLiteral(Annotated {
                    value: true,
                    line: 5,
                    column: 3
                }),
            ]
        );
    }
}
