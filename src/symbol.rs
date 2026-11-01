//!
//! Provides structures and methods for handling symbols and functions
//!

use std::collections::HashMap;

use crate::{Annotated, Error, ErrorCore, Result, TempToken, Token, Type, annotate::to_string};

/// Checks all unidentified tokens and converts them to symbols
///
/// # Parameters
///
/// tokens: The tokens to parse through
pub(crate) fn find_symbols(tokens: Vec<TempToken>) -> Result<Vec<Token>> {
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens.into_iter() {
        match token {
            TempToken::Token(token) => result.push(token),
            TempToken::Unidentified(s) => {
                let (line, column) = if let Some(c) = s.first() {
                    (c.line, c.column)
                } else {
                    continue;
                };

                let name = to_string(&s);
                if !is_name_valid(&name) {
                    return Err(Error {
                        error: ErrorCore::InvalidSymbolName(name),
                        line,
                        column,
                    });
                }

                result.push(Token::Symbol(Annotated {
                    value: name,
                    line,
                    column,
                }));
            }
        }
    }

    return Ok(result);
}

/// Defines a single variable
#[derive(Debug, Clone)]
pub struct Variable {
    /// The name of the variable
    pub name: String,
    /// The type of the variable
    pub typ: Type,
}

/// Structures all variables into a hashmap while checking for invalid names and
/// duplicates
///
/// # Parameters
///
/// variables: List of all variables
pub(crate) fn structure_variables(variables: Vec<Variable>) -> Result<HashMap<String, Variable>> {
    let mut result = HashMap::new();
    result.reserve(variables.len());

    for variable in variables.into_iter() {
        if !is_name_valid(&variable.name) {
            return Err(Error {
                error: ErrorCore::InvalidVariableName(variable.name),
                line: 0,
                column: 0,
            });
        }

        if let Some(prev) = result.insert(variable.name.clone(), variable) {
            return Err(Error {
                error: ErrorCore::DuplicateVariable(prev.name),
                line: 0,
                column: 0,
            });
        }
    }

    return Ok(result);
}

/// Defines a single function
#[derive(Debug, Clone)]
pub struct Function {
    /// The name of the function
    pub name: String,
    /// This types of all the input arguments
    pub args: Vec<Type>,
    /// The type of the output
    pub typ: Type,
}

/// Structures all functions into a hashmap while checking for invalid names and
/// duplicates
///
/// # Parameters
///
/// functions: List of all functions
pub(crate) fn structure_functions(
    functions: Vec<Function>,
) -> Result<HashMap<String, Vec<Function>>> {
    let mut result = HashMap::new();

    for function in functions.into_iter() {
        if !is_name_valid(&function.name) {
            return Err(Error {
                error: ErrorCore::InvalidFunctionName(function.name),
                line: 0,
                column: 0,
            });
        }

        let entry = result
            .entry(function.name.clone())
            .or_insert(Vec::<Function>::new());
        for prev in entry.iter() {
            if function.name == prev.name || function.args == prev.args {
                return Err(Error {
                    error: ErrorCore::DuplicateFunction(function.name, function.args),
                    line: 0,
                    column: 0,
                });
            }
        }
        entry.push(function);
    }

    return Ok(result);
}

/// Checks if the name is valid (alphanumeric + underscore and does not start with digit)
///
/// # Parameters
///
/// name: The name to check
pub(crate) fn is_name_valid(name: &str) -> bool {
    let is_char_valid = |c: char| c.is_alphanumeric() || c == '_';
    let is_char_initial = |c: char| c.is_alphabetic() || c == '_';

    return !(name.contains(|c: char| !is_char_valid(c))
        || name.is_empty()
        || !is_char_initial(name.chars().next().unwrap()));
}
