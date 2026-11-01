//!
//! Provides structures and methods for handling symbols and functions
//!

use crate::Type;

/// Defines a single symbol
#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    typ: Type,
}

/// Defines a single function
#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    args: Vec<Type>,
    typ: Type,
}
