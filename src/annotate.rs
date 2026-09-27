//!
//! Provides structures and methods for handling annotated strings and
//! characters, where each character is associated with its position (line and
//! column) in the source code.
//! 

/// Represents a string where each character is annotated with its position (line and column) in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AnnotatedString {
    pub(crate) characters: Vec<AnnotatedCharacter>,
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

                return Some(AnnotatedCharacter {
                    character,
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
            .filter(|c| !c.character.is_whitespace())
            .collect();

        return AnnotatedString { characters };
    }
}

/// Represents a slice of annotated characters
pub(crate) type AnnotatedStr = [AnnotatedCharacter];

/// Converts a slice of `AnnotatedCharacter` into a regular `String` by
/// extracting the characters
/// 
/// # Parameters
/// 
/// annotated_str: A slice of `AnnotatedCharacter` to be converted into a
/// regular `String`
pub(crate) fn to_string(annotated_str: &AnnotatedStr) -> String {
    return annotated_str.iter().map(|c| c.character).collect();
}

/// Represents a character with its position (line and column) in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AnnotatedCharacter {
    /// The character being annotated
    pub(crate) character: char,
    /// The line number where the character appears
    pub(crate) line: usize,
    /// The column number where the character appears
    pub(crate) column: usize,
}
