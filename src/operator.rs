//!
//! This module provides functionality for handling operators within expressions.
//!

use crate::{Error, ErrorCore, Result, Type};
use std::collections::HashMap;

/// Defines a single operator, either left, right or both sided
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            .any(|c| c.is_alphanumeric() || c.is_whitespace() || "_.,:?()[]{}".contains(c))
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
}
