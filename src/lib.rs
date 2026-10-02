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
pub use types::Type;
