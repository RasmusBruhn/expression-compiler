//!
//! Provides structures and methods for extracting string literals from
//! annotated strings.
//! 

use crate::annotate::AnnotatedString;


enum Expression {
    Unidentified(AnnotatedString),
    StringLiteral(AnnotatedString),
}

//pub(crate) fn extract_strings()
