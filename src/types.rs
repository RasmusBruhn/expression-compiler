//!
//! This module provides functionality for handling types within expressions.
//!

/// All different types that can be used within expressions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    SignedInteger,
    UnsignedInteger,
    Float,
    Boolean,
    String,
    Character,
}
