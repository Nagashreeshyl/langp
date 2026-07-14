//! Lang.P lexer — tokenization for the Lang.P programming language.
//!
//! Converts `.lp` source text into a stream of tokens according to
//! [`docs/grammar/02-lexical-grammar.ebnf`](../../docs/grammar/02-lexical-grammar.ebnf).
//!
//! # Example
//!
//! ```
//! use langp_lexer::{lex, format_tokens};
//!
//! let source = r#"print "Hello, Lang.P!"."#;
//! let tokens = lex(source).unwrap();
//! assert!(format_tokens(&tokens).contains("print"));
//! ```

pub mod error;
pub mod lexer;
pub mod span;
pub mod token;

pub use error::{LexError, LexErrorKind, LexResult};
pub use lexer::{format_tokens, lex, Lexer};
pub use span::Span;
pub use token::{InputTypeKeyword, Keyword, Token, TokenKind};
