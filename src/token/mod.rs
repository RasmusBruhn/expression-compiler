//!
//! Handles tokenization from raw text
//!

mod literal;
mod operator;
mod separator;
mod symbol;

use crate::{
    Result,
    annotate::{Annotated, AnnotatedString},
    operator::OperatorGroup,
};
use std::collections::HashMap;

/// A single token extracted from an annotated string
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    StringLiteral(Annotated<String>),
    CharacterLiteral(Annotated<char>),
    FloatLiteral(Annotated<f64>),
    IntegerLiteral(Annotated<u64>),
    BooleanLiteral(Annotated<bool>),
    Comma(Annotated<()>),
    Dot(Annotated<()>),
    Colon(Annotated<()>),
    Question(Annotated<()>),
    OpenBracket(Annotated<()>),
    CloseBracket(Annotated<()>),
    OpenSquareBracket(Annotated<()>),
    CloseSquareBracket(Annotated<()>),
    OpenCurlyBracket(Annotated<()>),
    CloseCurlyBracket(Annotated<()>),
    Operator(Annotated<String>),
    Symbol(Annotated<String>),
}

/// A single token extracted from an annotated string which can also be a
/// unidentified string
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TempToken {
    Unidentified(AnnotatedString),
    Token(Token),
}

/// Splits the input string into a list of tokens
///
/// # Parameters
///
/// s: The string to convert to tokens
pub(crate) fn find_tokens(
    s: AnnotatedString,
    operators: &HashMap<String, OperatorGroup>,
) -> Result<Vec<Token>> {
    let tokens = vec![TempToken::Unidentified(s)];
    let tokens = literal::find_literals(tokens)?;
    let tokens = separator::find_separators(tokens)?;
    let tokens = operator::find_operators(tokens, operators)?;
    let tokens = symbol::find_symbols(tokens)?;

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Error, ErrorCore, Type,
        operator::{Operator, OperatorBoth, structure_operators},
    };

    mod find_tokens {
        use super::*;

        #[test]
        fn literal() {
            let s = AnnotatedString::new("1");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap();

            assert_eq!(
                result,
                vec![Token::IntegerLiteral(Annotated {
                    value: 1,
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn separator() {
            let s = AnnotatedString::new(",");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap();

            assert_eq!(
                result,
                vec![Token::Comma(Annotated {
                    value: (),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn operator() {
            let s = AnnotatedString::new("-");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap();

            assert_eq!(
                result,
                vec![Token::Operator(Annotated {
                    value: "-".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn symbol() {
            let s = AnnotatedString::new("a");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap();

            assert_eq!(
                result,
                vec![Token::Symbol(Annotated {
                    value: "a".to_string(),
                    line: 1,
                    column: 1
                }),]
            );
        }

        #[test]
        fn all() {
            let s = AnnotatedString::new(",a-1");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Comma(Annotated {
                        value: (),
                        line: 1,
                        column: 1
                    }),
                    Token::Symbol(Annotated {
                        value: "a".to_string(),
                        line: 1,
                        column: 2
                    }),
                    Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 3
                    }),
                    Token::IntegerLiteral(Annotated {
                        value: 1,
                        line: 1,
                        column: 4
                    }),
                ]
            );
        }

        #[test]
        fn error_literal() {
            let s = AnnotatedString::new("99999999999999999999");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::IntegerLiteralTooLarge,
                    line: 1,
                    column: 20,
                }
            );
        }

        #[test]
        fn error_symbol() {
            let s = AnnotatedString::new("+");
            let operators = vec![Operator::Both(OperatorBoth {
                symbol: "-".to_string(),
                typ_left: Type::Float,
                typ_right: Type::UnsignedInteger,
                typ_out: Type::SignedInteger,
                priority: 0,
            })];
            let operators = structure_operators(operators).unwrap();
            let result = find_tokens(s, &operators).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::InvalidSymbolName("+".to_string()),
                    line: 1,
                    column: 1,
                }
            );
        }
    }
}
