//! Lang.P abstract syntax tree definitions.

pub mod nodes;

pub use nodes::*;

use langp_lexer::Span;

/// Pretty-print AST as JSON (for `langc --emit ast`).
pub fn to_json(program: &Program) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(program)
}

/// Merge spans from two nodes.
pub fn span_between(a: Span, b: Span) -> Span {
    Span::new(a.start.min(b.start), a.end.max(b.end), a.line, a.column)
}
