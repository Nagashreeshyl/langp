//! Lang.P interpreter — executes programs by walking the AST.

mod env;
mod eval;
mod builtins;

pub use eval::{run, RunResult};
