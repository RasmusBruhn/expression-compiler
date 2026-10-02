//!
//! This module provides functionality for handling operators within expressions.
//!

use crate::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Left(OperatorLeft),
    Right(OperatorRight),
    Both(OperatorBoth),
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
