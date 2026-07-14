//! Lang.P interpreter — executes programs by walking the AST.

mod env;
mod eval;
mod builtins;
mod collections;

pub use eval::{run, RunResult};
