//!
//! This library provides methods for analyzing expression (string, value, or
//! boolean) strings, compiling them into annotated structured representations
//! of the expressions.
//!

mod annotate;
mod error;
mod literal;
mod operator;
mod separator;
mod types;

pub use annotate::Annotated;
pub use error::{Error, ErrorCore, Result};
pub use operator::{Operator, OperatorBoth, OperatorLeft, OperatorRight};
pub use types::Type;

use annotate::AnnotatedString;

/// A single token extracted from an annotated string
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
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
    Operator(Annotated<String>),
    Symbol(Annotated<String>),
    Function(Annotated<String>),
}

/// A single token extracted from an annotated string which can also be a
/// unidentified string
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TempToken {
    Unidentified(AnnotatedString),
    Token(Token),
}
