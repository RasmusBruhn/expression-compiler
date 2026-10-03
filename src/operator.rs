//!
//! This module provides functionality for handling operators within expressions.
//!

use crate::Type;
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
    /// The symbol representing the operator (e.g., "++" for increment)
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
    /// The symbol representing the operator (e.g., "++" for increment)
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
    /// The symbol representing the operator (e.g., "++" for increment)
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
    /// The symbol representing the operator group (e.g., "++" for increment)
    pub symbol: String,
    /// All left-sided operators with this symbol
    pub left: Vec<OperatorLeft>,
    /// All right-sided operators with this symbol
    pub right: Vec<OperatorRight>,
    /// All both-sided operators with this symbol
    pub both: Vec<OperatorBoth>,
}

/// Structures a list of operators into a hashmap grouped by their symbol
///
/// # Parameters
///
/// operators: A vector of all operators
pub(crate) fn structure_operators(operators: Vec<Operator>) -> HashMap<String, OperatorGroup> {
    let mut map: HashMap<String, OperatorGroup> = HashMap::new();

    for operator in operators {
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

    return map;
}
