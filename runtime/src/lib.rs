//! Lang.P runtime — values, errors, and display helpers.

mod error;
mod value;

pub use error::{RuntimeError, RuntimeErrorKind, RuntimeResult};
pub use value::{set_contains, set_insert, UserFunction, Value};
