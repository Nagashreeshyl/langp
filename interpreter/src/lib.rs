//! Lang.P interpreter — executes programs by walking the AST.

mod env;
mod eval;
mod builtins;
mod collections;
mod objects;
mod oop;
mod modules;
mod stdlib;

pub use eval::{run, RunResult};
