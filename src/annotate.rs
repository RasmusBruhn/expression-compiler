//!
//! Provides structures and methods for handling annotated strings and
//! characters, where each character is associated with its position (line and
//! column) in the source code.
//!

use std::{fmt::Debug, ops::Deref};

/// Represents a string where each character is annotated with its position
/// (line and column) in the source code
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AnnotatedString {
    /// The annotated characters that make up the string
    pub(crate) characters: Vec<Annotated<char>>,
}

impl AnnotatedString {
    /// Constructs a new `AnnotatedString` from a regular string
    ///
    /// # Parameters
    ///
    /// str: The input string to be annotated with line and column information
    pub(crate) fn new(str: &str) -> Self {
        let characters = str
            .chars()
            .scan((1, 0), |(line, column), character| {
                // Update location of character
                if character == '\n' {
                    *column = 0;
                    *line += 1;
                } else {
                    *column += 1;
                }

                return Some(Annotated {
                    value: character,
                    line: *line,
                    column: *column,
                });
            })
            .collect();

        return AnnotatedString { characters };
    }

    /// Returns a new `AnnotatedString` with all whitespace characters removed
    pub(crate) fn remove_whitespace(&self) -> AnnotatedString {
        let characters = self
            .characters
            .iter()
            .cloned()
            .filter(|c| !c.value.is_whitespace())
            .collect();

        return AnnotatedString { characters };
    }
}

impl Deref for AnnotatedString {
    type Target = AnnotatedStr;

    fn deref(&self) -> &Self::Target {
        &self.characters
    }
}

/// Represents a slice of annotated characters
pub(crate) type AnnotatedStr = [Annotated<char>];

/// Converts a slice of `AnnotatedCharacter` into a regular `String` by
/// extracting the characters
///
/// # Parameters
///
/// annotated_str: A slice of `AnnotatedCharacter` to be converted into a
/// regular `String`
pub(crate) fn to_string(annotated_str: &AnnotatedStr) -> String {
    return annotated_str.iter().map(|c| c.value).collect();
}

/// Represents a value annotated with its position (line and column) in the
/// source code
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Annotated<T>
where
    T: Debug + Clone + PartialEq,
{
    /// The value being annotated
    pub(crate) value: T,
    /// The line number where the character appears
    pub(crate) line: usize,
    /// The column number where the character appears
    pub(crate) column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_string() {
        let s = "Hello\nWorld\n";
        let annotated = AnnotatedString::new(s);

        assert_eq!(annotated.characters.len(), s.chars().count());
        assert_eq!(annotated.characters[0].line, 1);
        assert_eq!(annotated.characters[0].column, 1);
        assert_eq!(annotated.characters[4].line, 1);
        assert_eq!(annotated.characters[4].column, 5);
        assert_eq!(annotated.characters[5].line, 2);
        assert_eq!(annotated.characters[5].column, 0);
        assert_eq!(annotated.characters[6].line, 2);
        assert_eq!(annotated.characters[6].column, 1);
        assert_eq!(annotated.characters[10].line, 2);
        assert_eq!(annotated.characters[10].column, 5);
        assert_eq!(annotated.characters[11].line, 3);
        assert_eq!(annotated.characters[11].column, 0);
    }

    #[test]
    fn test_remove_whitespace() {
        let s = "Hello World\nTest\tThis\n";
        let annotated = AnnotatedString::new(s);
        let no_whitespace = annotated.remove_whitespace();

        assert_eq!(to_string(&no_whitespace), "HelloWorldTestThis");
    }
}
