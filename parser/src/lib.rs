//! Lang.P parser — syntax analysis producing an AST.

pub mod error;
pub mod parser;

pub use error::{ParseError, ParseErrorKind, ParseResult};
pub use parser::{parse, Parser};
