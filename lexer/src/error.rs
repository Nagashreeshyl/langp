//! Lexical error types.

use crate::span::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnterminatedString,
    UnterminatedRawString,
    UnterminatedCharacter,
    InvalidEscape,
    InvalidUnicodeEscape,
    InvalidNumber,
    InconsistentIndent,
    TabInIndent,
    IndentNotMultipleOfFour,
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
    pub message: String,
}

impl LexError {
    pub fn new(kind: LexErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self.kind {
            LexErrorKind::InconsistentIndent
            | LexErrorKind::TabInIndent
            | LexErrorKind::IndentNotMultipleOfFour => "E0101",
            LexErrorKind::UnterminatedString
            | LexErrorKind::UnterminatedRawString
            | LexErrorKind::UnterminatedCharacter => "E0102",
            LexErrorKind::InvalidEscape | LexErrorKind::InvalidUnicodeEscape => "E0103",
            LexErrorKind::InvalidNumber => "E0104",
            LexErrorKind::UnexpectedChar(_) | LexErrorKind::UnexpectedEof => "E0100",
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error[{}]: {}\n  --> {}:{}:{}\n   |\n   = {}",
            self.code(),
            self.message,
            "source",
            self.span.line,
            self.span.column,
            self.message
        )
    }
}

pub type LexResult<T> = Result<T, LexError>;
