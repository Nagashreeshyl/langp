//! Lang.P semantic analyzer — name resolution and basic type checking.

mod analyze;
mod diagnostic;

pub use analyze::analyze;
pub use diagnostic::{Diagnostic, DiagnosticKind, Severity};
