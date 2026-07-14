use langp_lexer::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeErrorKind {
    UndefinedVariable,
    UndefinedFunction,
    TypeError,
    DivisionByZero,
    IndexOutOfBounds,
    InvalidOperation,
    Break,
    Continue,
    Return,
    UserError,
    IoError,
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub span: Span,
    pub message: String,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self.kind {
            RuntimeErrorKind::UndefinedVariable => "E0300",
            RuntimeErrorKind::UndefinedFunction => "E0301",
            RuntimeErrorKind::TypeError => "E0302",
            RuntimeErrorKind::DivisionByZero => "E0303",
            RuntimeErrorKind::IndexOutOfBounds => "E0304",
            RuntimeErrorKind::InvalidOperation => "E0305",
            RuntimeErrorKind::Break => "E0306",
            RuntimeErrorKind::Continue => "E0307",
            RuntimeErrorKind::Return => "E0308",
            RuntimeErrorKind::UserError => "E0309",
            RuntimeErrorKind::IoError => "E0310",
            RuntimeErrorKind::NotImplemented => "E0399",
        }
    }
}

impl fmt::Display for RuntimeError {
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

pub type RuntimeResult<T> = Result<T, RuntimeError>;
