//! Token definitions for Lang.P.

use crate::span::Span;
use std::fmt;

/// Keyword tokens reserved by the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    And,
    As,
    Async,
    Await,
    Break,
    Catch,
    Continue,
    Else,
    Enum,
    False,
    Finally,
    For,
    Forever,
    Function,
    If,
    In,
    Input,
    Interface,
    Let,
    Match,
    Not,
    Null,
    On,
    Or,
    Otherwise,
    Repeat,
    Return,
    SelfKw,
    Static,
    Super,
    This,
    True,
    Try,
    Type,
    Use,
    Wait,
    While,
    With,
    // Compound keywords (also emitted as dedicated kinds)
    OtherwiseIf,
    RepeatForever,
    WaitFor,
}

impl Keyword {
    pub fn from_ident(s: &str) -> Option<Self> {
        match s {
            "and" => Some(Self::And),
            "as" => Some(Self::As),
            "async" => Some(Self::Async),
            "await" => Some(Self::Await),
            "break" => Some(Self::Break),
            "catch" => Some(Self::Catch),
            "continue" => Some(Self::Continue),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "false" => Some(Self::False),
            "finally" => Some(Self::Finally),
            "for" => Some(Self::For),
            "forever" => Some(Self::Forever),
            "function" => Some(Self::Function),
            "if" => Some(Self::If),
            "in" => Some(Self::In),
            "input" => Some(Self::Input),
            "interface" => Some(Self::Interface),
            "let" => Some(Self::Let),
            "match" => Some(Self::Match),
            "not" => Some(Self::Not),
            "null" => Some(Self::Null),
            "on" => Some(Self::On),
            "or" => Some(Self::Or),
            "otherwise" => Some(Self::Otherwise),
            "repeat" => Some(Self::Repeat),
            "return" => Some(Self::Return),
            "self" => Some(Self::SelfKw),
            "static" => Some(Self::Static),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::True),
            "try" => Some(Self::Try),
            "type" => Some(Self::Type),
            "use" => Some(Self::Use),
            "wait" => Some(Self::Wait),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::As => "as",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Catch => "catch",
            Self::Continue => "continue",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::False => "false",
            Self::Finally => "finally",
            Self::For => "for",
            Self::Forever => "forever",
            Self::Function => "function",
            Self::If => "if",
            Self::In => "in",
            Self::Input => "input",
            Self::Interface => "interface",
            Self::Let => "let",
            Self::Match => "match",
            Self::Not => "not",
            Self::Null => "null",
            Self::On => "on",
            Self::Or => "or",
            Self::Otherwise => "otherwise",
            Self::Repeat => "repeat",
            Self::Return => "return",
            Self::SelfKw => "self",
            Self::Static => "static",
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Try => "try",
            Self::Type => "type",
            Self::Use => "use",
            Self::Wait => "wait",
            Self::While => "while",
            Self::With => "with",
            Self::OtherwiseIf => "otherwise if",
            Self::RepeatForever => "repeat forever",
            Self::WaitFor => "wait for",
        }
    }
}

/// Contextual keyword used after `input`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum InputTypeKeyword {
    Text,
    Number,
    Decimal,
    Boolean,
    Password,
    File,
    Folder,
    Date,
    Color,
}

impl InputTypeKeyword {
    pub fn from_ident(s: &str) -> Option<Self> {
        match s {
            "text" => Some(Self::Text),
            "number" => Some(Self::Number),
            "decimal" => Some(Self::Decimal),
            "boolean" => Some(Self::Boolean),
            "password" => Some(Self::Password),
            "file" => Some(Self::File),
            "folder" => Some(Self::Folder),
            "date" => Some(Self::Date),
            "color" => Some(Self::Color),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Decimal => "decimal",
            Self::Boolean => "boolean",
            Self::Password => "password",
            Self::File => "file",
            Self::Folder => "folder",
            Self::Date => "date",
            Self::Color => "color",
        }
    }
}

/// All token kinds emitted by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Keyword(Keyword),
    InputTypeKeyword(InputTypeKeyword),

    // Identifiers and literals
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),

    // Synthetic / structural
    Newline,
    Indent,
    Dedent,
    StmtEnd,
    BlockClose,
    Eof,

    // Operators (multi-char)
    EqEq,
    NotEq,
    LtEq,
    GtEq,
    Shl,
    Shr,
    Pow,
    IntDiv,
    AndAnd,
    OrOr,
    DotDot,
    DotDotLt,
    DotDotDot,
    Arrow,
    FatArrow,
    NullCoalesce,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,

    // Single-char operators and delimiters
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Lt,
    Gt,
    Bang,
    Question,
    Dot,
    Comma,
    Colon,
    Eq,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

impl TokenKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, TokenKind::Newline)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            lexeme: lexeme.into(),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword(k) => write!(f, "{}", k.as_str()),
            TokenKind::InputTypeKeyword(k) => write!(f, "{}", k.as_str()),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Int(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s.escape_default()),
            TokenKind::Char(c) => write!(f, "'{}'", c),
            TokenKind::Bool(b) => write!(f, "{}", b),
            TokenKind::Newline => write!(f, "NEWLINE"),
            TokenKind::Indent => write!(f, "INDENT"),
            TokenKind::Dedent => write!(f, "DEDENT"),
            TokenKind::StmtEnd => write!(f, "STMT_END"),
            TokenKind::BlockClose => write!(f, "BLOCK_CLOSE"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::LtEq => write!(f, "<="),
            TokenKind::GtEq => write!(f, ">="),
            TokenKind::Shl => write!(f, "<<"),
            TokenKind::Shr => write!(f, ">>"),
            TokenKind::Pow => write!(f, "**"),
            TokenKind::IntDiv => write!(f, "//"),
            TokenKind::AndAnd => write!(f, "&&"),
            TokenKind::OrOr => write!(f, "||"),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::DotDotLt => write!(f, "..<"),
            TokenKind::DotDotDot => write!(f, "..."),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::NullCoalesce => write!(f, "??"),
            TokenKind::PlusEq => write!(f, "+="),
            TokenKind::MinusEq => write!(f, "-="),
            TokenKind::StarEq => write!(f, "*="),
            TokenKind::SlashEq => write!(f, "/="),
            TokenKind::PercentEq => write!(f, "%="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Amp => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Dot => write!(f, "DOT"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}", self.kind, self.span)
    }
}
