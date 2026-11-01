//!
//! This module provides functionality for handling types within expressions.
//!

use crate::{Error, ErrorCore, Result};
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
    Alias(String),
}

/// An alias for a type
#[derive(Debug, Clone)]
pub struct TypeAlias {
    name: String,
    typ: Type,
}

/// Structures all type aliases into a map, checking name validities and
/// duplicates
/// 
/// # Parameters
/// 
/// aliases: A list of all type aliases
pub(crate) fn structure_type_aliases(aliases: Vec<TypeAlias>) -> Result<HashMap<String, Type>> {
    let mut result = HashMap::new();
    result.reserve(aliases.len());

    for alias in aliases.into_iter() {
        if alias
            .name
            .contains(|c: char| !(c.is_alphanumeric() || c == '_'))
            || alias.name.is_empty()
            || !(alias.name.chars().next().unwrap().is_alphabetic()
                || alias.name.chars().next().unwrap() == '_')
        {
            return Err(Error {
                error: ErrorCore::InvalidTypeAliasName(alias.name),
                line: 0,
                column: 0,
            });
        }

        if let Some(_) = result.insert(alias.name.clone(), alias.typ) {
            return Err(Error {
                error: ErrorCore::DuplicateTypeAlias(alias.name),
                line: 0,
                column: 0,
            });
        }
    }

    return Ok(result);
}
