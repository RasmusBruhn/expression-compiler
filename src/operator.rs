//!
//! This module provides functionality for handling operators within expressions.
//!

use crate::{
    Error, ErrorCore, Result, TempToken, Token, Type,
    annotate::{Annotated, AnnotatedStr, AnnotatedString, to_string},
};
use std::collections::HashMap;

/// Defines a single operator, either left, right or both sided
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    /// An operator working from the left side of its input, like ++x
    Left(OperatorLeft),
    /// An operator working from the right side of its input, like x++
    Right(OperatorRight),
    /// An operator taking two inputs using both sides like x+y
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
    /// The type of the input (e.g., integer, float)
    pub typ_in: Type,
    /// The type of the output (e.g., integer, float)
    pub typ_out: Type,
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
    /// The type of the input (e.g., integer, float)
    pub typ_in: Type,
    /// The type of the output (e.g., integer, float)
    pub typ_out: Type,
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
    /// The type of the left input (e.g., integer, float)
    pub typ_left: Type,
    /// The type of the right input (e.g., integer, float)
    pub typ_right: Type,
    /// The type of the output (e.g., integer, float)
    pub typ_out: Type,
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

    for operator in operators.into_iter() {
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

/// Finds all operators in unidentified segments of a list of tokens
///
/// # Parameters
///
/// tokens: A list of tokens to search for operators within
///
/// operators: A reference to a hashmap containing operator names and their
/// corresponding groups
pub(crate) fn find_operators(
    tokens: Vec<TempToken>,
    operators: &HashMap<String, OperatorGroup>,
) -> Result<Vec<TempToken>> {
    // Get the names of operators in the order to detect them (that is long
    // names first to ensure substrings do not split longer, like == being
    // detected as two = signs)
    let mut operators = operators.keys().cloned().collect::<Vec<String>>();
    operators.sort_by(|a, b| b.len().cmp(&a.len()));

    // Convert all tokens
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens.into_iter() {
        match token {
            TempToken::Token(token) => result.push(TempToken::Token(token)),
            TempToken::Unidentified(s) => {
                result.extend(find_operators_str(&s, &operators)?.into_iter())
            }
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
pub(crate) fn find_operators_str(s: &AnnotatedStr, operators: &[String]) -> Result<Vec<TempToken>> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut begin = 0;
    while begin < s.len() {
        let mut matched = false;
        for operator in operators.iter() {
            // Do not consider operators that would extend beyond the end of the string
            let end = begin + operator.len();
            if end > s.len() {
                continue;
            }

            // Check if operator is at the current position in the string
            if to_string(&s[begin..end]) == *operator {
                if start < begin {
                    result.push(TempToken::Unidentified(AnnotatedString::from_str(
                        &s[start..begin],
                    )));
                }

                result.push(TempToken::Token(Token::Operator(Annotated {
                    value: operator.clone(),
                    line: s[begin].line,
                    column: s[begin].column,
                })));

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
        result.push(TempToken::Unidentified(AnnotatedString::from_str(
            &s[start..],
        )));
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
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "1".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: " ".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: ".".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
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
                typ_in: Type::String,
                typ_out: Type::String,
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
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "-".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
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
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
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
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Unidentified(AnnotatedString::from_str(&s))]
            );
        }

        #[test]
        fn single() {
            let operators = vec![Operator::Left(OperatorLeft {
                symbol: "+".to_string(),
                priority: 0,
                typ_in: Type::String,
                typ_out: Type::String,
            })];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a+b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 2
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }

        #[test]
        fn multiple() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "-".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("+a+-b-");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 1
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[1..2])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 3
                    })),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 4
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[4..5])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 6
                    })),
                ]
            );
        }

        #[test]
        fn substring() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "++".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a++b+");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "++".to_string(),
                        line: 1,
                        column: 2
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[3..4])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 5
                    })),
                ]
            );
        }
    }
}
