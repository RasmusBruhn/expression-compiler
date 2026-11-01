//!
//! This module provides functionality for handling types within expressions.
//!

use crate::{Error, ErrorCore, Result, symbol};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

/// All different types that can be used within expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    SignedInteger,
    UnsignedInteger,
    Float,
    Boolean,
    String,
    Character,
    Array(String),
    Struct(HashMap<String, String>),
    Enum(HashMap<String, Option<String>>),
    Variant(HashSet<String>),
    Alias(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::SignedInteger => f.write_str("sint"),
            Self::UnsignedInteger => f.write_str("uint"),
            Self::Float => f.write_str("float"),
            Self::Boolean => f.write_str("bool"),
            Self::String => f.write_str("string"),
            Self::Character => f.write_str("char"),
            Self::Array(data) => f.write_str(&format!("array({})", data)),
            Self::Struct(data) => f.write_str(&format!(
                "struct({})",
                data.iter()
                    .map(|entry| format!("{}: {}", entry.0, entry.1))
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            Self::Enum(data) => f.write_str(&format!(
                "enum({})",
                data.iter()
                    .map(|entry| if let Some(typ) = entry.1 {
                        format!("{}: {}", entry.0, typ)
                    } else {
                        format!("{}", entry.0)
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            Self::Variant(data) => f.write_str(&format!(
                "variant({})",
                data.iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            Self::Alias(data) => f.write_str(data),
        };
    }
}

/// An alias for a type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    /// The name of the type alias
    pub name: String,
    /// The type of the alias
    pub typ: Type,
}

/// Structures all type aliases into a map, checking name validities and
/// duplicates
///
/// # Parameters
///
/// aliases: A list of all type aliases
pub(crate) fn structure_type_aliases(
    aliases: Vec<TypeAlias>,
) -> Result<HashMap<String, TypeAlias>> {
    let mut result = HashMap::new();
    result.reserve(aliases.len() + 6);
    result.insert(
        "sint".to_string(),
        TypeAlias {
            name: "sint".to_string(),
            typ: Type::SignedInteger,
        },
    );
    result.insert(
        "uint".to_string(),
        TypeAlias {
            name: "uint".to_string(),
            typ: Type::UnsignedInteger,
        },
    );
    result.insert(
        "float".to_string(),
        TypeAlias {
            name: "float".to_string(),
            typ: Type::Float,
        },
    );
    result.insert(
        "bool".to_string(),
        TypeAlias {
            name: "bool".to_string(),
            typ: Type::Boolean,
        },
    );
    result.insert(
        "string".to_string(),
        TypeAlias {
            name: "string".to_string(),
            typ: Type::String,
        },
    );
    result.insert(
        "char".to_string(),
        TypeAlias {
            name: "char".to_string(),
            typ: Type::Character,
        },
    );

    for alias in aliases.into_iter() {
        if !symbol::is_name_valid(&alias.name) {
            return Err(Error {
                error: ErrorCore::InvalidTypeAliasName(alias.name),
                line: 0,
                column: 0,
            });
        }

        if let Some(prev) = result.insert(alias.name.clone(), alias) {
            return Err(Error {
                error: ErrorCore::DuplicateTypeAlias(prev.name),
                line: 0,
                column: 0,
            });
        }
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod structure_type_aliases {
        use super::*;

        #[test]
        fn empty() {
            let input = vec![];
            let result = structure_type_aliases(input).unwrap();

            assert_eq!(result.len(), 6);
        }

        #[test]
        fn single() {
            let input = vec![TypeAlias {
                name: "Type1".to_string(),
                typ: Type::Float,
            }];
            let result = structure_type_aliases(input).unwrap();

            assert_eq!(result.len(), 7);
            assert_eq!(
                result.get("Type1").unwrap(),
                &TypeAlias {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                }
            );
        }

        #[test]
        fn multiple() {
            let input = vec![
                TypeAlias {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                },
                TypeAlias {
                    name: "Type2".to_string(),
                    typ: Type::String,
                },
            ];
            let result = structure_type_aliases(input).unwrap();

            assert_eq!(result.len(), 8);
            assert_eq!(
                result.get("Type1").unwrap(),
                &TypeAlias {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                }
            );
            assert_eq!(
                result.get("Type2").unwrap(),
                &TypeAlias {
                    name: "Type2".to_string(),
                    typ: Type::String,
                }
            );
        }

        #[test]
        fn duplicate() {
            let input = vec![
                TypeAlias {
                    name: "Type1".to_string(),
                    typ: Type::Float,
                },
                TypeAlias {
                    name: "Type1".to_string(),
                    typ: Type::String,
                },
            ];
            let result = structure_type_aliases(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::DuplicateTypeAlias("Type1".to_string()),
                    line: 0,
                    column: 0,
                }
            );
        }

        #[test]
        fn invalid_name() {
            let input = vec![TypeAlias {
                name: "Type-1".to_string(),
                typ: Type::Float,
            }];
            let result = structure_type_aliases(input).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::InvalidTypeAliasName("Type-1".to_string()),
                    line: 0,
                    column: 0,
                }
            );
        }
    }
}
