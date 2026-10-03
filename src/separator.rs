//!
//! Provides structures and methods for finding and handling separators (like
//! commas, dots, colons and brackets) in expression strings.
//!

use crate::{
    Result,
    annotate::{Annotated, AnnotatedStr, AnnotatedString},
    literal,
};

/// A single token extracted from an annotated string, either an unidentified
/// segment, a literal, or a separator
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Unidentified(AnnotatedString),
    StringLiteral(Annotated<String>),
    CharacterLiteral(Annotated<char>),
    FloatLiteral(Annotated<f64>),
    IntegerLiteral(Annotated<u64>),
    BooleanLiteral(Annotated<bool>),
    Comma(Annotated<()>),
    Dot(Annotated<()>),
    Colon(Annotated<()>),
    OpenBracket(Annotated<()>),
    CloseBracket(Annotated<()>),
    OpenSquareBracket(Annotated<()>),
    CloseSquareBracket(Annotated<()>),
    OpenCurlyBracket(Annotated<()>),
    CloseCurlyBracket(Annotated<()>),
}

/// Finds all separators in unidentified segments of a list of literal tokens
///
/// # Parameters
///
/// tokens: A slice of literal tokens to search for separators within
pub(crate) fn find_separators(tokens: Vec<literal::Token>) -> Result<Vec<Token>> {
    let mut result = Vec::new();

    for token in tokens {
        match token {
            literal::Token::StringLiteral(x) => result.push(Token::StringLiteral(x)),
            literal::Token::CharacterLiteral(x) => result.push(Token::CharacterLiteral(x)),
            literal::Token::FloatLiteral(x) => result.push(Token::FloatLiteral(x)),
            literal::Token::IntegerLiteral(x) => result.push(Token::IntegerLiteral(x)),
            literal::Token::BooleanLiteral(x) => result.push(Token::BooleanLiteral(x)),
            literal::Token::Unidentified(x) => result.extend(find_separators_str(&x)?),
        }
    }

    return Ok(result);
}

/// Finds all separators in a single annotated string
///
/// # Parameters
///
/// s: The annotated string to search for separators within
pub(crate) fn find_separators_str(s: &AnnotatedStr) -> Result<Vec<Token>> {
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
            result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..i])));
        }

        result.push(new_token);
        start = i + 1;
    }

    // Add any remaining unidentified segment if any
    if start < s.len() {
        result.push(Token::Unidentified(AnnotatedString::from_str(&s[start..])));
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
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::Dot(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a.b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::Dot(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod comma {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(",");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::Comma(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a,b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::Comma(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod colon {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(":");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::Colon(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a:b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::Colon(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("(");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::OpenBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a(b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::OpenBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new(")");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::CloseBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a)b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::CloseBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_square_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("[");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::OpenSquareBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a[b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::OpenSquareBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_square_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("]");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::CloseSquareBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a]b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::CloseSquareBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod open_curly_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("{");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::OpenCurlyBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a{b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::OpenCurlyBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    mod close_curly_bracket {
        use super::*;

        #[test]
        fn single() {
            let s = AnnotatedString::new("}");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::CloseCurlyBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                })]
            );
        }

        #[test]
        fn imbedded() {
            let s = AnnotatedString::new("a}b");
            let tokens = literal::find_literals(&s).unwrap();
            let result = find_separators(tokens).unwrap();

            assert_eq!(
                result,
                vec![
                    Token::Unidentified(AnnotatedString::from_str(&s[0..1])),
                    Token::CloseCurlyBracket(Annotated {
                        value: (),
                        line: 1,
                        column: 2,
                    }),
                    Token::Unidentified(AnnotatedString::from_str(&s[2..3])),
                ]
            );
        }
    }

    #[test]
    fn empty() {
        let s = AnnotatedString::new("");
        let tokens = literal::find_literals(&s).unwrap();
        let result = find_separators(tokens).unwrap();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn multiple() {
        let s = AnnotatedString::new(".,:()\n[]{}");
        let tokens = literal::find_literals(&s).unwrap();
        let result = find_separators(tokens).unwrap();

        assert_eq!(
            result,
            vec![
                Token::Dot(Annotated {
                    value: (),
                    line: 1,
                    column: 1,
                }),
                Token::Comma(Annotated {
                    value: (),
                    line: 1,
                    column: 2,
                }),
                Token::Colon(Annotated {
                    value: (),
                    line: 1,
                    column: 3,
                }),
                Token::OpenBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 4,
                }),
                Token::CloseBracket(Annotated {
                    value: (),
                    line: 1,
                    column: 5,
                }),
                Token::Unidentified(AnnotatedString::from_str(&s[5..6])),
                Token::OpenSquareBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 1,
                }),
                Token::CloseSquareBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 2,
                }),
                Token::OpenCurlyBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 3,
                }),
                Token::CloseCurlyBracket(Annotated {
                    value: (),
                    line: 2,
                    column: 4,
                }),
            ]
        );
    }
}
