//!
//! This module provides functionality for handling types within expressions.
//!

use std::collections::{HashMap, HashSet};

/// All different types that can be used within expressions
#[derive(Debug, Clone)]
pub enum Type {
    SignedInteger,
    UnsignedInteger,
    Float,
    Boolean,
    String,
    Character,
    Array(Box<Type>),
    Struct(HashMap<String, Type>),
    Enum(HashMap<String, Option<Type>>),
    Variant(HashSet<Type>),
}
