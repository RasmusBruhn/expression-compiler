//!
//! Provides structures and methods for finding and handling separators (like
//! commas, dots, colons and brackets) in expression strings.
//!

use crate::{
    Error, ErrorCore, Result,
    annotate::{Annotated, AnnotatedStr},
    literal,
};

/// A single token extracted from an annotated string, either an unidentified
/// segment, a literal, or a separator
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token<'a> {
    Unidentified(&'a AnnotatedStr),
    StringLiteral(Annotated<String>),
    CharacterLiteral(Annotated<char>),
    FloatLiteral(Annotated<f64>),
    IntegerLiteral(Annotated<u64>),
    BooleanLiteral(Annotated<bool>),
    Comma(Annotated<()>),
    Dot(Annotated<()>),
    Colon(Annotated<()>),
    OpenBracket(Annotated<()>),
    CloseBracket(Annotated<()>),
    OpenSquareBracket(Annotated<()>),
    CloseSquareBracket(Annotated<()>),
    OpenCurlyBracket(Annotated<()>),
    CloseCurlyBracket(Annotated<()>),
}

/// Finds all separators in unidentified segments of a list of literal tokens
///
/// # Parameters
///
/// tokens: A slice of literal tokens to search for separators within
pub(crate) fn find_separators<'a>(tokens: Vec<literal::Token<'a>>) -> Result<Vec<Token<'a>>> {
    let mut result = Vec::new();

    for token in tokens {
        match token {
            literal::Token::StringLiteral(x) => result.push(Token::StringLiteral(x)),
            literal::Token::CharacterLiteral(x) => result.push(Token::CharacterLiteral(x)),
            literal::Token::FloatLiteral(x) => result.push(Token::FloatLiteral(x)),
            literal::Token::IntegerLiteral(x) => result.push(Token::IntegerLiteral(x)),
            literal::Token::BooleanLiteral(x) => result.push(Token::BooleanLiteral(x)),
            literal::Token::Unidentified(x) => result.extend(find_separators_str(x)?),
        }
    }

    return Ok(result);
}

/// Finds all separators in a single annotated string
///
/// # Parameters
///
/// s: The annotated string to search for separators within
pub(crate) fn find_separators_str<'a>(s: &'a AnnotatedStr) -> Result<Vec<Token<'a>>> {
    let mut result = Vec::new();
    let mut start = 0;
    for (i, c) in s.iter().enumerate() {
        // Find separator characters
        let new_token = match c.value {
            ',' => Token::Comma(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '.' => Token::Dot(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ':' => Token::Colon(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '(' => Token::OpenBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ')' => Token::CloseBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '[' => Token::OpenSquareBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ']' => Token::CloseSquareBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '{' => Token::OpenCurlyBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '}' => Token::CloseCurlyBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            _ => continue,
        };

        // Add previous unidentified segment if any
        if start < i {
            result.push(Token::Unidentified(&s[start..i]));
        }

        result.push(new_token);
        start = i + 1;
    }

    // Add any remaining unidentified segment if any
    if start < s.len() {
        result.push(Token::Unidentified(&s[start..]));
    }

    return Ok(result);
}
