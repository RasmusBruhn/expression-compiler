//!
//! Provides structures and methods for extracting string literals from
//! annotated strings.
//!

use crate::{
    Error, ErrorCore, Result,
    annotate::{Annotated, AnnotatedStr},
};

/// A single token extracted from an annotated string, either an unidentified
/// segment or a literal
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token<'a> {
    Unidentified(&'a AnnotatedStr),
    StringLiteral(Annotated<String>),
    CharacterLiteral(Annotated<char>),
    FloatLiteral(Annotated<f64>),
    IntegerLiteral(Annotated<u64>),
    BooleanLiteral(Annotated<bool>),
}

/// Finds all literals (string, character, float, integer, boolean) in the
/// annotated string and returns a vector of tokens containing them
/// 
/// # Parameters
///
/// str: The annotated string to search for literals
pub(crate) fn find_literals(str: &AnnotatedStr) -> Result<Vec<Token>> {
    todo!()
}

/// Find all string and character literals in the annotated string and returns a
/// vector of tokens (`Unidentified`, `StringLiteral`, or `CharacterLiteral`)
///
/// # Parameters
///
/// str: The annotated string to search for string and character literals
fn find_string_literals(str: &AnnotatedStr) -> Result<Vec<Token>> {
    todo!()
}

/// Find all value literals (float, integer, boolean) in the annotated string and returns a
/// vector of tokens (`Unidentified`, `FloatLiteral`, `IntegerLiteral`, or `BooleanLiteral`)
///
/// # Parameters
///
/// str: The annotated string to search for value literals
fn find_value_literals(str: &AnnotatedStr) -> Result<Vec<Token>> {
    todo!()
}
