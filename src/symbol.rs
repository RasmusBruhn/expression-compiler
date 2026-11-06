//!
//! Provides structures and methods for handling symbols and functions
//!

use std::collections::HashMap;

use crate::{Error, ErrorCore, Result, Type};

/// Defines a single variable
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            if function.name == prev.name && function.args == prev.args {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod is_name_valid {
        use super::*;

        #[test]
        fn alphabetic() {
            let input = "test";

            assert!(is_name_valid(input));
        }

        #[test]
        fn alphanumeric() {
            let input = "test1";

            assert!(is_name_valid(input));
        }

        #[test]
        fn underscore() {
            let input = "_test";

            assert!(is_name_valid(input));
        }

        #[test]
        fn error_symbol() {
            let input = "test-";

            assert!(!is_name_valid(input));
        }

        #[test]
        fn error_init() {
            let input = "1test";

            assert!(!is_name_valid(input));
        }

        #[test]
        fn error_empty() {
            let input = "";

            assert!(!is_name_valid(input));
        }
    }

    mod structure_variables {
        use super::*;

        #[test]
        fn empty() {
            let input = vec![];
            let result = structure_variables(input).unwrap();

            assert_eq!(result.len(), 0);
        }

        #[test]
        fn single() {
            let input = vec![Variable {
                name: "Type1".to_string(),
                typ: Type::Float,
            }];
            let result = structure_variables(input).unwrap();

            assert_eq!(result.len(), 1);
            assert_eq!(
                result.get("Type1").unwrap(),
                &Variable {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                }
            );
        }

        #[test]
        fn multiple() {
            let input = vec![
                Variable {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                },
                Variable {
                    name: "Type2".to_string(),
                    typ: Type::String,
                },
            ];
            let result = structure_variables(input).unwrap();

            assert_eq!(result.len(), 2);
            assert_eq!(
                result.get("Type1").unwrap(),
                &Variable {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                }
            );
            assert_eq!(
                result.get("Type2").unwrap(),
                &Variable {
                    name: "Type2".to_string(),
                    typ: Type::String,
                }
            );
        }

        #[test]
        fn duplicate() {
            let input = vec![
                Variable {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                },
                Variable {
                    name: "Type1".to_string(),
                    typ: Type::String,
                },
            ];
            let result = structure_variables(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::DuplicateVariable("Type1".to_string()),
                    line: 0,
                    column: 0,
                }
            );
        }

        #[test]
        fn invalid_name() {
            let input = vec![Variable {
                name: "Type-1".to_string(),
                typ: Type::Float,
            }];
            let result = structure_variables(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::InvalidVariableName("Type-1".to_string()),
                    line: 0,
                    column: 0,
                }
            );
        }
    }

    mod structure_functions {
        use super::*;

        #[test]
        fn empty() {
            let input = vec![];
            let result = structure_functions(input).unwrap();

            assert_eq!(result.len(), 0);
        }

        #[test]
        fn single() {
            let input = vec![Function {
                name: "Type1".to_string(),
                args: Vec::new(),
                typ: Type::Float,
            }];
            let result = structure_functions(input).unwrap();

            assert_eq!(result.len(), 1);
            assert_eq!(
                result.get("Type1").unwrap(),
                &vec![Function {
                    name: "Type1".to_string(),
                    args: Vec::new(),
                    typ: Type::Float,
                }]
            );
        }

        #[test]
        fn multiple() {
            let input = vec![
                Function {
                    name: "Type1".to_string(),
                    args: Vec::new(),
                    typ: Type::Float,
                },
                Function {
                    name: "Type2".to_string(),
                    args: Vec::new(),
                    typ: Type::String,
                },
            ];
            let result = structure_functions(input).unwrap();

            assert_eq!(result.len(), 2);
            assert_eq!(
                result.get("Type1").unwrap(),
                &vec![Function {
                    name: "Type1".to_string(),
                    args: Vec::new(),
                    typ: Type::Float,
                }]
            );
            assert_eq!(
                result.get("Type2").unwrap(),
                &vec![Function {
                    name: "Type2".to_string(),
                    args: Vec::new(),
                    typ: Type::String,
                }]
            );
        }

        #[test]
        fn same() {
            let input = vec![
                Function {
                    name: "Type1".to_string(),
                    args: vec![Type::Float],
                    typ: Type::Float,
                },
                Function {
                    name: "Type1".to_string(),
                    args: vec![Type::String],
                    typ: Type::String,
                },
            ];
            let result = structure_functions(input).unwrap();

            assert_eq!(result.len(), 1);
            assert_eq!(
                result.get("Type1").unwrap(),
                &vec![
                    Function {
                        name: "Type1".to_string(),
                        args: vec![Type::Float],
                        typ: Type::Float,
                    },
                    Function {
                        name: "Type1".to_string(),
                        args: vec![Type::String],
                        typ: Type::String,
                    },
                ]
            );
        }

        #[test]
        fn duplicate() {
            let input = vec![
                Function {
                    name: "Type1".to_string(),
                    args: vec![Type::Float],
                    typ: Type::Float,
                },
                Function {
                    name: "Type1".to_string(),
                    args: vec![Type::Float],
                    typ: Type::String,
                },
            ];
            let result = structure_functions(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::DuplicateFunction("Type1".to_string(), vec![Type::Float]),
                    line: 0,
                    column: 0,
                }
            );
        }

        #[test]
        fn invalid_name() {
            let input = vec![Function {
                name: "Type-1".to_string(),
                args: Vec::new(),
                typ: Type::Float,
            }];
            let result = structure_functions(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::InvalidFunctionName("Type-1".to_string()),
                    line: 0,
                    column: 0,
                }
            );
        }
    }
}
