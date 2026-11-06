//!
//! Handles tokenization of tokens (variables and functions) from raw text.
//!

use super::{TempToken, Token};
use crate::{Annotated, Error, ErrorCore, Result, annotate::to_string, symbol::is_name_valid};

/// Checks all unidentified tokens and converts them to symbols
///
/// # Parameters
///
/// tokens: The tokens to parse through
pub(super) fn find_symbols(tokens: Vec<TempToken>) -> Result<Vec<Token>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod find_symbols {
        use super::*;
        use crate::annotate::AnnotatedString;

        #[test]
        fn valid() {
            let s = AnnotatedString::new("test_1");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_symbols(tokens).unwrap();

            assert_eq!(
                result,
                vec![Token::Symbol(Annotated {
                    value: to_string(&s),
                    line: 1,
                    column: 1,
                }),]
            );
        }

        #[test]
        fn empty() {
            let s = AnnotatedString::new("");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_symbols(tokens).unwrap();

            assert_eq!(result, vec![]);
        }

        #[test]
        fn invalid() {
            let s = AnnotatedString::new("test-1");
            let tokens = vec![TempToken::Unidentified(s.clone())];
            let result = find_symbols(tokens).unwrap_err();

            assert_eq!(
                result,
                Error {
                    error: ErrorCore::InvalidSymbolName(to_string(&s)),
                    line: 1,
                    column: 1,
                }
            );
        }
    }
}
