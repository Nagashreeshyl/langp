//! Parser error types.

use langp_lexer::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnexpectedEof,
    MissingBlockClose,
    MissingStatementEnd,
    InvalidAssignmentTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
    pub message: String,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self.kind {
            ParseErrorKind::UnexpectedToken | ParseErrorKind::UnexpectedEof => "E0200",
            ParseErrorKind::MissingBlockClose => "E0201",
            ParseErrorKind::MissingStatementEnd => "E0202",
            ParseErrorKind::InvalidAssignmentTarget => "E0203",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error[{}]: {}\n  --> {}:{}:{}",
            self.code(),
            self.message,
            "source",
            self.span.line,
            self.span.column
        )
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
