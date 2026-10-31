//!
//! Provides structures and methods for finding and handling separators (like
//! commas, dots, colons and brackets) in expression strings.
//!

use crate::{
    Result, TempToken, Token,
    annotate::{Annotated, AnnotatedStr, AnnotatedString},
};

/// Finds all separators in unidentified segments of a list of tokens
///
/// # Parameters
///
/// tokens: A slice of tokens to search for separators within
pub(crate) fn find_separators(tokens: Vec<TempToken>) -> Result<Vec<TempToken>> {
    let mut result = Vec::new();
    result.reserve(tokens.len());

    for token in tokens.into_iter() {
        match token {
            TempToken::Token(token) => result.push(TempToken::Token(token)),
            TempToken::Unidentified(s) => result.extend(find_separators_single(&s)?.into_iter()),
        }
    }

    return Ok(result);
}

/// Finds all separators in a single annotated string
///
/// # Parameters
///
/// s: The annotated string to search for separators within
pub(crate) fn find_separators_single(s: &AnnotatedStr) -> Result<Vec<TempToken>> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in s.iter().enumerate() {
        // Find separator characters
        let new_token = match c.value {
            ',' => Token::Comma(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '.' => Token::Dot(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ':' => Token::Colon(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '(' => Token::OpenBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ')' => Token::CloseBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '[' => Token::OpenSquareBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            ']' => Token::CloseSquareBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '{' => Token::OpenCurlyBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            '}' => Token::CloseCurlyBracket(Annotated {
                value: (),
                line: c.line,
                column: c.column,
            }),
            _ => continue,
        };

        // Add previous unidentified segment if any
        if start < i {
            result.push(TempToken::Unidentified(AnnotatedString::from_str(
                &s[start..i],
            )));
        }

        result.push(TempToken::Token(new_token));
        start = i + 1;
    }

    // Add any remaining unidentified segment if any
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
    use crate::annotate::AnnotatedString;

    mod dot {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(".");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::Dot(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a.b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Dot(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod comma {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(",");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::Comma(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a,b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Comma(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod colon {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(":");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::Colon(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a:b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::Colon(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("(");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::OpenBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a(b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::OpenBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(")");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::CloseBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a)b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::CloseBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_square_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("[");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::OpenSquareBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a[b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::OpenSquareBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_square_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("]");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::CloseSquareBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a]b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::CloseSquareBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_curly_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("{");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::OpenCurlyBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a{b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::OpenCurlyBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_curly_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("}");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![TempToken::Token(Token::CloseCurlyBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }))]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a}b");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    TempToken::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    TempToken::Token(Token::CloseCurlyBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    })),
                    TempToken::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    #[test]
    fn empty() {
        let s = AnnotatedString::new("");
        let tokens = vec![TempToken::Unidentified(s.clone())];
        let result = find_separators(tokens).unwrap();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn multiple() {
        let s = AnnotatedString::new(".,:()\n[]{}");
        let tokens = vec![TempToken::Unidentified(s.clone())];
        let result = find_separators(tokens).unwrap();

        assert_eq!(
            result,
            vec![
                TempToken::Token(Token::Dot(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })),
                TempToken::Token(Token::Comma(Annotated {
                    value: (),
                    line: 1,
                    column: 2,
                })),
                TempToken::Token(Token::Colon(Annotated {
                    value: (),
                    line: 1,
                    column: 3,
                })),
                TempToken::Token(Token::OpenBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 4,
                })),
                TempToken::Token(Token::CloseBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 5,
                })),
                TempToken::Unidentified(AnnotatedString::from_str(&s[5..6])),
                TempToken::Token(Token::OpenSquareBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 1,
                })),
                TempToken::Token(Token::CloseSquareBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 2,
                })),
                TempToken::Token(Token::OpenCurlyBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 3,
                })),
                TempToken::Token(Token::CloseCurlyBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 4,
                })),
            ]
        );
    }
}
