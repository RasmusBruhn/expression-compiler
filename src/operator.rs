//!
//! This module provides functionality for handling operators within expressions.
//!

use crate::{
    Error, ErrorCore, Result, Type,
    annotate::{Annotated, AnnotatedStr, AnnotatedString, to_string},
    literal, separator,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Left(OperatorLeft),
    Right(OperatorRight),
    Both(OperatorBoth),
}

impl Operator {
    /// Returns the symbol of the operator
    pub fn symbol(&self) -> &str {
        match self {
            Operator::Left(op) => &op.symbol,
            Operator::Right(op) => &op.symbol,
            Operator::Both(op) => &op.symbol,
        }
    }
}

/// Defines a single left-sided (like ++x) operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OperatorLeft {
    /// The symbol representing the operator (e.g., "++" for increment), must
    /// not contain (alphanumerical characters, whitespace, underscore, dot,
    /// comma, colon, or any brackets)
    pub symbol: String,
    /// The type of the operator (e.g., integer, float)
    pub typ: Type,
    /// The priority of the operator, a lower value indicates higher precedence,
    /// operators of same priority are evaluated left to right
    pub priority: usize,
}

/// Defines a single right-sided (like x++) operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OperatorRight {
    /// The symbol representing the operator (e.g., "++" for increment), must
    /// not contain (alphanumerical characters, whitespace, underscore, dot,
    /// comma, colon, or any brackets)
    pub symbol: String,
    /// The type of the operator (e.g., integer, float)
    pub typ: Type,
    /// The priority of the operator, a lower value indicates higher precedence,
    /// operators of same priority are evaluated left to right
    pub priority: usize,
}

/// Defines a single both-sided (like x+y) operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OperatorBoth {
    /// The symbol representing the operator (e.g., "++" for increment), must
    /// not contain (alphanumerical characters, whitespace, underscore, dot,
    /// comma, colon, or any brackets)
    pub symbol: String,
    /// The type of the left operand (e.g., integer, float)
    pub typ_left: Type,
    /// The type of the right operand (e.g., integer, float)
    pub typ_right: Type,
    /// The priority of the operator, a lower value indicates higher precedence,
    /// operators of same priority are evaluated left to right
    pub priority: usize,
}

/// A group of operators with the same symbol
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct OperatorGroup {
    /// The symbol representing the operator group (e.g., "++" for increment),
    /// must not contain (alphanumerical characters, whitespace, underscore,
    /// dot, comma, colon, or any brackets)
    pub symbol: String,
    /// All left-sided operators with this symbol
    pub left: Vec<OperatorLeft>,
    /// All right-sided operators with this symbol
    pub right: Vec<OperatorRight>,
    /// All both-sided operators with this symbol
    pub both: Vec<OperatorBoth>,
}

/// Structures a list of operators into a hashmap grouped by their symbol, fails
/// if any operator contains illigal characters
///
/// # Parameters
///
/// operators: A vector of all operators
pub(crate) fn structure_operators(
    operators: Vec<Operator>,
) -> Result<HashMap<String, OperatorGroup>> {
    let mut map: HashMap<String, OperatorGroup> = HashMap::new();

    for operator in operators {
        if operator
            .symbol()
            .chars()
            .any(|c| c.is_alphanumeric() || c.is_whitespace() || "_.,:()[]{}".contains(c))
        {
            return Err(Error {
                error: ErrorCore::OperatorIllegalCharacters(operator.symbol().to_string()),
                line: 0,
                column: 0,
            });
        }

        let symbol = operator.symbol();
        let entry = map.entry(symbol.to_string()).or_insert(OperatorGroup {
            symbol: symbol.to_string(),
            left: Vec::new(),
            right: Vec::new(),
            both: Vec::new(),
        });

        match operator {
            Operator::Left(op) => {
                entry.left.push(op);
            }
            Operator::Right(op) => {
                entry.right.push(op);
            }
            Operator::Both(op) => {
                entry.both.push(op);
            }
        }
    }

    return Ok(map);
}

/// A single token extracted from an annotated string, either an unidentified
/// segment, a literal, a separator, or an operator
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Unidentified(AnnotatedString),
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
    Operator(Annotated<String>),
}

/// Finds all operators in unidentified segments of a list of literal tokens
///
/// # Parameters
///
/// tokens: A slice of literal tokens to search for operators within
///
/// operators: A reference to a hashmap containing operator names and their
/// corresponding groups
pub(crate) fn find_operators(
    tokens: Vec<separator::Token>,
    operators: &HashMap<String, OperatorGroup>,
) -> Result<Vec<Token>> {
    // Get the names of operators in the order to detect them (that is long
    // names first to ensure substrings do not split longer, like == being
    // detected as two = signs)
    let mut operators = operators.keys().cloned().collect::<Vec<String>>();
    operators.sort_by(|a, b| b.len().cmp(&a.len()));

    // Convert all tokens
    let mut result = Vec::new();

    for token in tokens {
        match token {
            separator::Token::StringLiteral(x) => result.push(Token::StringLiteral(x)),
            separator::Token::CharacterLiteral(x) => result.push(Token::CharacterLiteral(x)),
            separator::Token::FloatLiteral(x) => result.push(Token::FloatLiteral(x)),
            separator::Token::IntegerLiteral(x) => result.push(Token::IntegerLiteral(x)),
            separator::Token::BooleanLiteral(x) => result.push(Token::BooleanLiteral(x)),
            separator::Token::Dot(x) => result.push(Token::Dot(x)),
            separator::Token::Comma(x) => result.push(Token::Comma(x)),
            separator::Token::Colon(x) => result.push(Token::Colon(x)),
            separator::Token::OpenBracket(x) => result.push(Token::OpenBracket(x)),
            separator::Token::CloseBracket(x) => result.push(Token::CloseBracket(x)),
            separator::Token::OpenSquareBracket(x) => result.push(Token::OpenSquareBracket(x)),
            separator::Token::CloseSquareBracket(x) => result.push(Token::CloseSquareBracket(x)),
            separator::Token::OpenCurlyBracket(x) => result.push(Token::OpenCurlyBracket(x)),
            separator::Token::CloseCurlyBracket(x) => result.push(Token::CloseCurlyBracket(x)),
            separator::Token::Unidentified(x) => result.extend(find_operators_str(&x, &operators)?),
        }
    }

    return Ok(result);
}

/// Finds all operators in a single annotated string
///
/// # Parameters
///
/// s: The annotated string to search for operators within
///
/// operators: A slice of operator names to search for within the annotated
/// string, in the order to detect them
pub(crate) fn find_operators_str(s: &AnnotatedStr, operators: &[String]) -> Result<Vec<Token>> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut begin = 0;
    while begin < s.len() {
        let mut matched = false;
        for operator in operators {
            // Do not consider operators that would extend beyond the end of the string
            let end = begin + operator.len();
            if end > s.len() {
                continue;
            }

            // Check if operator is at the current position in the string
            if to_string(&s[begin..end]) == *operator {
                if start < begin {
                    result.push(Token::Unidentified(AnnotatedString::from_str(
                        &s[start..begin],
                    )));
                }

                result.push(Token::Operator(Annotated {
                    value: operator.clone(),
                    line: s[begin].line,
                    column: s[begin].column,
                }));

                begin += operator.len();
                start = begin;
                matched = true;
                break;
            }
        }

        // Make sure begin is advanced if no operator was matched
        if !matched {
            begin += 1;
        }
    }

    if start < s.len() {
        result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..])));
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod structure_operators {
        use super::*;

        #[test]
        fn illegal_characters() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "a".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "1".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: " ".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: ".".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
            ];

            for operator in &operators {
                let result = structure_operators(vec![operator.clone()]).unwrap_err();

                assert_eq!(
                    result,
                    Error {
                        error: ErrorCore::OperatorIllegalCharacters(operator.symbol().to_string()),
                        line: 0,
                        column: 0,
                    }
                );
            }
        }

        #[test]
        fn single() {
            let operators = vec![Operator::Left(OperatorLeft {
                symbol: "+".to_string(),
                priority: 0,
                typ: Type::String,
            })];

            let result = structure_operators(operators).unwrap();
            assert_eq!(result.len(), 1);
        }

        #[test]
        fn multiple() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "-".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
            ];

            let result = structure_operators(operators).unwrap();
            assert_eq!(result.len(), 2);
        }

        #[test]
        fn duplicate() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ: Type::Character,
                }),
            ];

            let result = structure_operators(operators).unwrap();
            assert_eq!(result.len(), 1);
        }
    }

    mod find_operators {
        use super::*;

        #[test]
        fn none() {
            let operators = vec![];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a+b");
            let tokens = literal::find_literals(&s).unwrap();
            let tokens = separator::find_separators(tokens).unwrap();
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![Token::Unidentified(AnnotatedString::from_str(&s))]
            );
        }

        #[test]
        fn single() {
            let operators = vec![Operator::Left(OperatorLeft {
                symbol: "+".to_string(),
                priority: 0,
                typ: Type::String,
            })];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a+b");
            let tokens = literal::find_literals(&s).unwrap();
            let tokens = separator::find_separators(tokens).unwrap();
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 2
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }

        #[test]
        fn multiple() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "-".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("+a+-b-");
            let tokens = literal::find_literals(&s).unwrap();
            let tokens = separator::find_separators(tokens).unwrap();
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 1
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[1..2])),
                    Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 3
                    }),
                    Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 4
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[4..5])),
                    Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 6
                    }),
                ]
            );
        }

        #[test]
        fn substring() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "++".to_string(),
                    priority: 0,
                    typ: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a++b+");
            let tokens = literal::find_literals(&s).unwrap();
            let tokens = separator::find_separators(tokens).unwrap();
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::Operator(Annotated {
                        value: "++".to_string(),
                        line: 1,
                        column: 2
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[3..4])),
                    Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 5
                    }),
                ]
            );
        }
    }
}
