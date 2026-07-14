use langp_lexer::Span;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    DuplicateDefinition,
    UndefinedName,
    TypeMismatch,
    InvalidOperation,
    UnusedValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub kind: DiagnosticKind,
    pub span: Span,
    pub message: String,
}

impl Diagnostic {
    pub fn error(kind: DiagnosticKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn warning(kind: DiagnosticKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            kind,
            span,
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self.kind {
            DiagnosticKind::DuplicateDefinition => "E0201",
            DiagnosticKind::UndefinedName => "E0202",
            DiagnosticKind::TypeMismatch => "E0203",
            DiagnosticKind::InvalidOperation => "E0204",
            DiagnosticKind::UnusedValue => "W0101",
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        write!(
            f,
            "{level}[{}]: {}\n  --> {}:{}:{}",
            self.code(),
            self.message,
            "source",
            self.span.line,
            self.span.column
        )
    }
}
