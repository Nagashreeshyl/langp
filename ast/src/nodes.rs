//! AST node definitions for Lang.P.

use langp_lexer::{InputTypeKeyword, Span};
use serde::{Deserialize, Serialize};

/// A complete Lang.P source file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<ModuleItem>,
    pub span: Span,
}

/// Top-level module item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleItem {
    Use(UseDecl),
    Function(FunctionDecl),
    Type(TypeDecl),
    Enum(EnumDecl),
    EventHandler(EventHandler),
    Stmt(Stmt),
}

/// `use module.name.`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseDecl {
    pub path: Vec<String>,
    pub span: Span,
}

/// `function name(...), ... .`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeExpr>,
    pub body: Block,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Option<TypeExpr>,
    pub default: Option<Expr>,
    pub span: Span,
}

/// `type Name, ... .`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDecl {
    pub name: String,
    pub members: Vec<TypeMember>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeMember {
    Field(FieldDecl),
    Function(FunctionDecl),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: String,
    pub ty: Option<TypeExpr>,
    pub default: Option<Expr>,
    pub span: Span,
}

/// `enum Name, ... .`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub span: Span,
}

/// `on event, ... ..`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventHandler {
    pub event: Expr,
    pub condition: Option<Expr>,
    pub body: Block,
    pub span: Span,
}

/// Indented statement block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// Statement nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Assign {
        target: AssignTarget,
        op: AssignOp,
        value: Expr,
        span: Span,
    },
    Print {
        inline: bool,
        parts: Vec<Expr>,
        span: Span,
    },
    Return {
        values: Vec<Expr>,
        span: Span,
    },
    Break { span: Span },
    Continue { span: Span },
    If(IfStmt),
    Repeat(RepeatStmt),
    For(ForStmt),
    While(WhileStmt),
    Try(TryStmt),
    Write {
        kind: WriteKind,
        value: Expr,
        destination: Expr,
        span: Span,
    },
    Io(IoStmt),
    Expr {
        expr: Expr,
        span: Span,
    },
    Pass { span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssignTarget {
    Name(String, Span),
    Member { object: Box<Expr>, name: String, span: Span },
    Index { object: Box<Expr>, index: Box<Expr>, span: Span },
    Tuple(Vec<String>, Span),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub elif_clauses: Vec<ElifClause>,
    pub else_block: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElifClause {
    pub condition: Expr,
    pub block: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepeatStmt {
    Count {
        count: Expr,
        var: Option<String>,
        body: Block,
        span: Span,
    },
    Forever {
        body: Block,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStmt {
    pub binding: ForBinding,
    pub iterable: Expr,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForBinding {
    Single(String, Span),
    KeyValue(String, String, Span),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryStmt {
    pub body: Block,
    pub catches: Vec<CatchClause>,
    pub finally_block: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub name: String,
    pub ty: Option<TypeExpr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteKind {
    Write,
    WriteBytes,
    Append,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IoStmt {
    Copy { source: Expr, dest: Expr, span: Span },
    Move { source: Expr, dest: Expr, span: Span },
    Rename { source: Expr, dest: Expr, span: Span },
    Delete { target: Expr, span: Span },
}

/// Expression nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Int { value: i64, span: Span },
    Float { value: f64, span: Span },
    String { value: String, span: Span },
    Char { value: char, span: Span },
    Bool { value: bool, span: Span },
    Null { span: Span },
    Ident { name: String, span: Span },
    SelfExpr { span: Span },
    Super { span: Span },
    This { span: Span },

    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Arg>,
        span: Span,
    },
    Member {
        object: Box<Expr>,
        name: String,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    With {
        parts: Vec<Expr>,
        span: Span,
    },
    Input {
        input_type: Option<InputTypeKeyword>,
        prompt: String,
        span: Span,
    },
    Read {
        kind: ReadKind,
        path: String,
        span: Span,
    },
    Http {
        kind: HttpKind,
        url: Box<Expr>,
        body: Option<Box<Expr>>,
        span: Span,
    },
    List {
        elements: Vec<Expr>,
        span: Span,
    },
    Dict {
        entries: Vec<(Expr, Expr)>,
        span: Span,
    },
    Tuple {
        elements: Vec<Expr>,
        span: Span,
    },
    Object {
        ty: TypeExpr,
        args: Vec<Arg>,
        fields: Option<Block>,
        span: Span,
    },
    Lambda {
        params: Vec<Param>,
        body: LambdaBody,
        span: Span,
    },
    If {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: Span,
    },
    Is {
        expr: Box<Expr>,
        ty: TypeExpr,
        negated: bool,
        span: Span,
    },
    NullCoalesce {
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arg {
    pub name: Option<String>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    IntDiv,
    Pow,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Pos,
    Not,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadKind {
    Text,
    Bytes,
    Lines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpKind {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    WaitFor,
}

/// Type expression nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    Named {
        name: String,
        generics: Vec<TypeExpr>,
        span: Span,
    },
    Optional {
        inner: Box<TypeExpr>,
        span: Span,
    },
    Tuple {
        types: Vec<TypeExpr>,
        span: Span,
    },
}
