//!
//! This library provides methods for analyzing expression (string, value, or
//! boolean) strings, compiling them into annotated structured representations
//! of the expressions.
//!

mod annotate;
mod error;
mod operator;
mod symbol;
mod types;
mod token;

pub use annotate::Annotated;
pub use error::{Error, ErrorCore, Result};
pub use operator::{Operator, OperatorBoth, OperatorLeft, OperatorRight};
pub use symbol::{Function, Variable};
pub use types::{Type, TypeAlias};
