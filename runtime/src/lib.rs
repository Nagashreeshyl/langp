//! Lang.P runtime — values, errors, and display helpers.

mod error;
mod value;

pub use error::{RuntimeError, RuntimeErrorKind, RuntimeResult};
pub use value::{UserFunction, Value};
