//!
//! Handles tokenization of user defined operators from raw text.
//!

use super::{TempToken, Token};
use crate::{
    Result,
    annotate::{Annotated, AnnotatedStr, AnnotatedString, to_string},
    operator::OperatorGroup,
};
use std::collections::HashMap;

/// Finds all operators in unidentified segments of a list of tokens
///
/// # Parameters
///
/// tokens: A list of tokens to search for operators within
///
/// operators: A reference to a hashmap containing operator names and their
/// corresponding groups
pub(super) fn find_operators(
    tokens: Vec<TempToken>,
    operators: &HashMap<String, OperatorGroup>,
) -> Result<Vec<TempToken>> {
    // Get the names of operators in the order to detect them (that is long
    // names first to ensure substrings do not split longer, like == being
    // detected as two = signs)
    let mut operators = operators.keys().cloned().collect::<Vec<String>>();
    operators.sort_by(|a, b| b.len().cmp(&a.len()));

    // Convert all tokens
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens.into_iter() {
        match token {
            TempToken::Token(token) => result.push(TempToken::Token(token)),
            TempToken::Unidentified(s) => {
                result.extend(find_operators_str(&s, &operators)?.into_iter())
            }
        }
    }

    return Ok(result);
}

/// Finds all operators in a single annotated string
///
/// # Parameters
///
/// s: The annotated string to search for operators within
///
/// operators: A slice of operator names to search for within the annotated
/// string, in the order to detect them
fn find_operators_str(s: &AnnotatedStr, operators: &[String]) -> Result<Vec<TempToken>> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut begin = 0;
    while begin < s.len() {
        let mut matched = false;
        for operator in operators.iter() {
            // Do not consider operators that would extend beyond the end of the string
            let end = begin + operator.len();
            if end > s.len() {
                continue;
            }

            // Check if operator is at the current position in the string
            if to_string(&s[begin..end]) == *operator {
                if start < begin {
                    result.push(TempToken::Unidentified(AnnotatedString::from_str(
                        &s[start..begin],
                    )));
                }

                result.push(TempToken::Token(Token::Operator(Annotated {
                    value: operator.clone(),
                    line: s[begin].line,
                    column: s[begin].column,
                })));

                begin += operator.len();
                start = begin;
                matched = true;
                break;
            }
        }

        // Make sure begin is advanced if no operator was matched
        if !matched {
            begin += 1;
        }
    }

    if start < s.len() {
        result.push(TempToken::Unidentified(AnnotatedString::from_str(
            &s[start..],
        )));
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Operator, OperatorLeft, Type, operator::structure_operators};

    mod find_operators {
        use super::*;

        #[test]
        fn none() {
            let operators = vec![];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a+b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Unidentified(AnnotatedString::from_str(&s))]
            );
        }

        #[test]
        fn single() {
            let operators = vec![Operator::Left(OperatorLeft {
                symbol: "+".to_string(),
                priority: 0,
                typ_in: Type::String,
                typ_out: Type::String,
            })];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a+b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 2
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }

        #[test]
        fn multiple() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "-".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("+a+-b-");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 1
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[1..2])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 3
                    })),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 4
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[4..5])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "-".to_string(),
                        line: 1,
                        column: 6
                    })),
                ]
            );
        }

        #[test]
        fn substring() {
            let operators = vec![
                Operator::Left(OperatorLeft {
                    symbol: "+".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
                Operator::Left(OperatorLeft {
                    symbol: "++".to_string(),
                    priority: 0,
                    typ_in: Type::String,
                    typ_out: Type::String,
                }),
            ];
            let operators = structure_operators(operators).unwrap();

            let s = AnnotatedString::new("a++b+");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_operators(tokens, &operators).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "++".to_string(),
                        line: 1,
                        column: 2
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[3..4])),
                    TempToken::Token(Token::Operator(Annotated {
                        value: "+".to_string(),
                        line: 1,
                        column: 5
                    })),
                ]
            );
        }
    }
}
