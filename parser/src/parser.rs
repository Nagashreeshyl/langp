//! Recursive-descent parser for Lang.P.

use crate::error::{ParseError, ParseErrorKind, ParseResult};
use langp_ast::{
    span_between, Arg, AssignOp, AssignTarget, BinaryOp, Block, CatchClause, ElifClause,
    EnumDecl, EnumVariant, EventHandler, Expr, FieldDecl, ForBinding, ForStmt, FunctionDecl,
    HttpKind, IfStmt, IoStmt, LambdaBody, ModuleItem, Param, Program, ReadKind, RepeatStmt,
    Stmt, TryStmt, TypeDecl, TypeExpr, TypeMember, UnaryOp, UseDecl, WhileStmt, WriteKind,
};
use langp_lexer::{Keyword, Span, Token, TokenKind};

/// How an indented block is closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockClose {
    /// All blocks close with `..` (Grammar Freeze v1.0).
    DoubleDot,
    /// Inner branch blocks (closed by outer construct's `..`).
    None,
}

/// Parses token streams into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(mut self) -> ParseResult<Program> {
        let start = self.current_span();
        let mut items = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            items.push(self.parse_module_item()?);
        }

        let end = if items.is_empty() {
            start
        } else {
            items.last().map(|i| item_span(i)).unwrap_or(start)
        };

        Ok(Program {
            items,
            span: span_between(start, end),
        })
    }

    fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        if self.check_keyword(Keyword::Use) {
            return Ok(ModuleItem::Use(self.parse_use_decl()?));
        }
        if self.check_keyword(Keyword::Async) || self.check_keyword(Keyword::Function) {
            return Ok(ModuleItem::Function(self.parse_function_decl(false)?));
        }
        if self.check_keyword(Keyword::Type) {
            return Ok(ModuleItem::Type(self.parse_type_decl()?));
        }
        if self.check_keyword(Keyword::Enum) {
            return Ok(ModuleItem::Enum(self.parse_enum_decl()?));
        }
        if self.check_keyword(Keyword::On) {
            return Ok(ModuleItem::EventHandler(self.parse_event_handler()?));
        }
        Ok(ModuleItem::Stmt(self.parse_statement()?))
    }

    fn parse_use_decl(&mut self) -> ParseResult<UseDecl> {
        let start = self.bump_keyword(Keyword::Use)?;
        let mut path = vec![self.expect_ident_raw()?];
        while self.check(&TokenKind::Dot) {
            self.bump();
            path.push(self.expect_ident_raw()?);
        }
        self.expect_stmt_end()?;
        let end = self.previous_span();
        Ok(UseDecl {
            path,
            span: span_between(start, end),
        })
    }

    fn parse_function_decl(&mut self, _nested: bool) -> ParseResult<FunctionDecl> {
        let start = self.current_span();
        let is_async = if self.check_keyword(Keyword::Async) {
            self.bump();
            true
        } else {
            false
        };
        self.expect_keyword(Keyword::Function)?;
        let name = self.expect_ident_raw()?;
        self.expect(&TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&TokenKind::RParen)?;
        let return_type = if self.check(&TokenKind::Arrow) {
            self.bump();
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::DoubleDot)?;
        let span = span_between(start, body.span);
        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
            is_async,
            span,
        })
    }

    fn parse_type_decl(&mut self) -> ParseResult<TypeDecl> {
        let start = self.bump_keyword(Keyword::Type)?;
        let name = self.expect_ident_raw()?;
        let extends = if self.check_ident("extends") {
            self.bump();
            Some(self.expect_ident_raw()?)
        } else {
            None
        };
        self.expect(&TokenKind::Comma)?;
        let members = self.parse_type_members()?;
        Ok(TypeDecl {
            name,
            extends,
            members,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_enum_decl(&mut self) -> ParseResult<EnumDecl> {
        let start = self.bump_keyword(Keyword::Enum)?;
        let name = self.expect_ident_raw()?;
        self.expect(&TokenKind::Comma)?;
        self.skip_newlines();
        self.expect(&TokenKind::Indent)?;
        let mut variants = Vec::new();
        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) {
                break;
            }
            let vstart = self.current_span();
            let vname = self.expect_ident_raw()?;
            self.expect_stmt_end()?;
            variants.push(EnumVariant {
                name: vname,
                span: span_between(vstart, self.previous_span()),
            });
        }
        self.expect(&TokenKind::Dedent)?;
        self.expect_block_close()?;
        Ok(EnumDecl {
            name,
            variants,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_type_members(&mut self) -> ParseResult<Vec<TypeMember>> {
        self.skip_newlines();
        self.expect(&TokenKind::Indent)?;
        let mut members = Vec::new();
        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) {
                break;
            }
            if self.check_keyword(Keyword::Function) {
                members.push(TypeMember::Function(self.parse_function_decl(true)?));
            } else {
                let start = self.current_span();
                let name = self.expect_ident_raw()?;
                let ty = if self.check(&TokenKind::Colon) {
                    self.bump();
                    Some(self.parse_type_expr()?)
                } else {
                    None
                };
                let default = if self.check(&TokenKind::Eq) {
                    self.bump();
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect_stmt_end()?;
                members.push(TypeMember::Field(FieldDecl {
                    name,
                    ty,
                    default,
                    span: span_between(start, self.previous_span()),
                }));
            }
        }
        self.expect(&TokenKind::Dedent)?;
        self.expect_block_close()?;
        Ok(members)
    }

    fn parse_event_handler(&mut self) -> ParseResult<EventHandler> {
        let start = self.bump_keyword(Keyword::On)?;
        let event = self.parse_expr()?;
        let condition = if self.check_ident("where") {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::DoubleDot)?;
        let end = body.span;
        Ok(EventHandler {
            event,
            condition,
            body,
            span: span_between(start, end),
        })
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        let start = self.current_span();

        if self.check_ident("print") {
            return self.parse_print_stmt(start);
        }
        if self.check_keyword(Keyword::Return) {
            self.bump();
            let values = if self.check_stmt_end() {
                Vec::new()
            } else {
                self.parse_expr_list()?
            };
            self.expect_stmt_end()?;
            return Ok(Stmt::Return {
                values,
                span: span_between(start, self.previous_span()),
            });
        }
        if self.check_keyword(Keyword::Break) {
            self.bump();
            self.expect_stmt_end()?;
            return Ok(Stmt::Break {
                span: span_between(start, self.previous_span()),
            });
        }
        if self.check_keyword(Keyword::Continue) {
            self.bump();
            self.expect_stmt_end()?;
            return Ok(Stmt::Continue {
                span: span_between(start, self.previous_span()),
            });
        }
        if self.check_keyword(Keyword::If) {
            return Ok(Stmt::If(self.parse_if_stmt(start)?));
        }
        if self.check_keyword(Keyword::RepeatForever) || self.check_keyword(Keyword::Repeat) {
            return Ok(Stmt::Repeat(self.parse_repeat_stmt(start)?));
        }
        if self.check_keyword(Keyword::For) {
            return Ok(Stmt::For(self.parse_for_stmt(start)?));
        }
        if self.check_keyword(Keyword::While) {
            return Ok(Stmt::While(self.parse_while_stmt(start)?));
        }
        if self.check_keyword(Keyword::Try) {
            return Ok(Stmt::Try(self.parse_try_stmt(start)?));
        }
        if self.check_ident("pass") {
            self.bump();
            self.expect_stmt_end()?;
            return Ok(Stmt::Pass {
                span: span_between(start, self.previous_span()),
            });
        }
        if self.check_ident("write") || self.check_ident("write_bytes") || self.check_ident("append") {
            return self.parse_write_stmt(start);
        }
        if self.check_ident("copy") || self.check_ident("move") || self.check_ident("rename") || self.check_ident("delete") {
            return self.parse_io_stmt(start);
        }

        // Assignment or expression statement
        if self.is_assign_start() {
            return self.parse_assign_or_expr_stmt(start);
        }

        let expr = self.parse_expr()?;
        self.expect_stmt_end()?;
        Ok(Stmt::Expr {
            span: span_between(start, self.previous_span()),
            expr,
        })
    }

    fn parse_print_stmt(&mut self, start: Span) -> ParseResult<Stmt> {
        self.expect_ident("print")?;
        let inline = if self.check_ident("inline") {
            self.bump();
            true
        } else {
            false
        };
        let mut parts = vec![self.parse_expr()?];
        while self.check(&TokenKind::Comma) || self.check_keyword(Keyword::With) {
            self.bump();
            parts.push(self.parse_expr()?);
        }
        self.expect_stmt_end()?;
        Ok(Stmt::Print {
            inline,
            parts,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_write_stmt(&mut self, start: Span) -> ParseResult<Stmt> {
        let kind = match self.peek_ident().as_deref() {
            Some("write_bytes") => WriteKind::WriteBytes,
            Some("append") => WriteKind::Append,
            _ => WriteKind::Write,
        };
        self.bump();
        let value = self.parse_expr()?;
        self.expect_ident("to")?;
        let destination = self.parse_expr()?;
        self.expect_stmt_end()?;
        Ok(Stmt::Write {
            kind,
            value,
            destination,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_io_stmt(&mut self, start: Span) -> ParseResult<Stmt> {
        let name = self.expect_ident_raw()?;
        let stmt = match name.as_str() {
            "copy" => {
                let source = self.parse_expr()?;
                self.expect_ident("to")?;
                let dest = self.parse_expr()?;
                IoStmt::Copy {
                    source,
                    dest,
                    span: span_between(start, self.previous_span()),
                }
            }
            "move" => {
                let source = self.parse_expr()?;
                self.expect_ident("to")?;
                let dest = self.parse_expr()?;
                IoStmt::Move {
                    source,
                    dest,
                    span: span_between(start, self.previous_span()),
                }
            }
            "rename" => {
                let source = self.parse_expr()?;
                self.expect_ident("to")?;
                let dest = self.parse_expr()?;
                IoStmt::Rename {
                    source,
                    dest,
                    span: span_between(start, self.previous_span()),
                }
            }
            "delete" => IoStmt::Delete {
                target: self.parse_expr()?,
                span: span_between(start, self.previous_span()),
            },
            _ => {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    start,
                    format!("unknown io statement '{}'", name),
                ));
            }
        };
        self.expect_stmt_end()?;
        Ok(Stmt::Io(stmt))
    }

    fn parse_if_stmt(&mut self, start: Span) -> ParseResult<IfStmt> {
        self.bump_keyword(Keyword::If)?;
        let condition = self.parse_expr()?;
        self.expect(&TokenKind::Comma)?;
        let then_block = self.parse_indented_block(BlockClose::None)?;

        let mut elif_clauses = Vec::new();
        while self.check_keyword(Keyword::OtherwiseIf) {
            let estart = self.bump_keyword(Keyword::OtherwiseIf)?;
            let cond = self.parse_expr()?;
            self.expect(&TokenKind::Comma)?;
            let block = self.parse_indented_block(BlockClose::None)?;
            let block_span = block.span;
            elif_clauses.push(ElifClause {
                condition: cond,
                block,
                span: span_between(estart, block_span),
            });
        }

        let else_block = if self.check_keyword(Keyword::Otherwise) {
            self.bump();
            self.expect(&TokenKind::Comma)?;
            Some(self.parse_indented_block(BlockClose::None)?)
        } else {
            None
        };

        if self.check(&TokenKind::BlockClose) {
            self.bump();
        } else {
            return Err(self.block_close_error());
        }
        let end = self.previous_span();
        Ok(IfStmt {
            condition,
            then_block,
            elif_clauses,
            else_block,
            span: span_between(start, end),
        })
    }

    fn parse_repeat_stmt(&mut self, start: Span) -> ParseResult<RepeatStmt> {
        if self.check_keyword(Keyword::RepeatForever) {
            self.bump();
            self.expect(&TokenKind::Comma)?;
            let body = self.parse_indented_block(BlockClose::DoubleDot)?;
            let body_span = body.span;
            return Ok(RepeatStmt::Forever {
                body,
                span: span_between(start, body_span),
            });
        }

        self.bump_keyword(Keyword::Repeat)?;
        let count = self.parse_expr()?;
        self.expect_ident("times")?;
        let var = if self.check_keyword(Keyword::As) {
            self.bump();
            Some(self.expect_ident_raw()?)
        } else {
            None
        };
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::DoubleDot)?;
        let body_span = body.span;
        Ok(RepeatStmt::Count {
            count,
            var,
            body,
            span: span_between(start, body_span),
        })
    }

    fn parse_for_stmt(&mut self, start: Span) -> ParseResult<ForStmt> {
        self.bump_keyword(Keyword::For)?;
        let first = self.expect_ident_raw()?;
        let binding = if self.check(&TokenKind::Comma) {
            self.bump();
            let second = self.expect_ident_raw()?;
            ForBinding::KeyValue(first, second, start)
        } else {
            ForBinding::Single(first, start)
        };
        self.expect_keyword(Keyword::In)?;
        let iterable = self.parse_expr()?;
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::DoubleDot)?;
        let body_span = body.span;
        Ok(ForStmt {
            binding,
            iterable,
            body,
            span: span_between(start, body_span),
        })
    }

    fn parse_while_stmt(&mut self, start: Span) -> ParseResult<WhileStmt> {
        self.bump_keyword(Keyword::While)?;
        let condition = self.parse_expr()?;
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::DoubleDot)?;
        let body_span = body.span;
        Ok(WhileStmt {
            condition,
            body,
            span: span_between(start, body_span),
        })
    }

    fn parse_try_stmt(&mut self, start: Span) -> ParseResult<TryStmt> {
        self.bump_keyword(Keyword::Try)?;
        self.expect(&TokenKind::Comma)?;
        let body = self.parse_indented_block(BlockClose::None)?;

        let mut catches = Vec::new();
        while self.check_keyword(Keyword::Catch) {
            let cstart = self.bump_keyword(Keyword::Catch)?;
            let name = self.expect_ident_raw()?;
            let ty = if self.check(&TokenKind::Colon) {
                self.bump();
                Some(self.parse_type_expr()?)
            } else {
                None
            };
            self.expect(&TokenKind::Comma)?;
            let catch_body = self.parse_indented_block(BlockClose::None)?;
            let catch_span = catch_body.span;
            catches.push(CatchClause {
                name,
                ty,
                body: catch_body,
                span: span_between(cstart, catch_span),
            });
        }

        let finally_block = if self.check_keyword(Keyword::Finally) {
            self.bump();
            self.expect(&TokenKind::Comma)?;
            Some(self.parse_indented_block(BlockClose::None)?)
        } else {
            None
        };

        if self.check(&TokenKind::BlockClose) {
            self.bump();
        } else {
            return Err(self.block_close_error());
        }
        Ok(TryStmt {
            body,
            catches,
            finally_block,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_indented_block(&mut self, close: BlockClose) -> ParseResult<Block> {
        self.skip_newlines();
        let start = self.current_span();
        self.expect(&TokenKind::Indent)?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        self.expect(&TokenKind::Dedent)?;

        match close {
            BlockClose::DoubleDot => {
                self.expect_block_close()?;
            }
            BlockClose::None => {}
        }

        Ok(Block {
            statements,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_assign_or_expr_stmt(&mut self, start: Span) -> ParseResult<Stmt> {
        // Lookahead for assignment
        let checkpoint = self.pos;
        if let Ok(target) = self.parse_assign_target() {
            if let Some(op) = self.parse_assign_op() {
                let value = self.parse_expr()?;
                self.expect_stmt_end_after_expr(&value)?;
                return Ok(Stmt::Assign {
                    target,
                    op,
                    value,
                    span: span_between(start, self.previous_span()),
                });
            }
        }
        self.pos = checkpoint;
        let expr = self.parse_expr()?;
        self.expect_stmt_end()?;
        Ok(Stmt::Expr {
            expr,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_assign_target(&mut self) -> ParseResult<AssignTarget> {
        let start = self.current_span();
        if self.is_ident() {
            let name = self.expect_ident_raw()?;
            if self.check(&TokenKind::Comma) {
                let mut names = vec![name];
                while self.check(&TokenKind::Comma) {
                    self.bump();
                    names.push(self.expect_ident_raw()?);
                }
                return Ok(AssignTarget::Tuple(names, start));
            }
            let ty = if self.check(&TokenKind::Colon) {
                self.bump();
                Some(self.parse_type_expr()?)
            } else {
                None
            };
            let target = self.finish_assign_target(
                Expr::Ident {
                    name: name.clone(),
                    span: start,
                },
                start,
            )?;
            return Ok(match target {
                AssignTarget::Name { name, span, .. } => AssignTarget::Name { name, ty, span },
                other => other,
            });
        }
        if self.check_keyword(Keyword::SelfKw) {
            self.bump();
            return self.finish_assign_target(Expr::SelfExpr { span: start }, start);
        }
        Err(ParseError::new(
            ParseErrorKind::InvalidAssignmentTarget,
            start,
            "invalid assignment target",
        ))
    }

    fn finish_assign_target(
        &mut self,
        mut expr: Expr,
        start: Span,
    ) -> ParseResult<AssignTarget> {
        while self.check(&TokenKind::Dot) {
            self.bump();
            let member = self.expect_ident_raw()?;
            expr = Expr::Member {
                object: Box::new(expr),
                name: member,
                span: start,
            };
        }
        if self.check(&TokenKind::LBracket) {
            self.bump();
            let index = self.parse_expr()?;
            self.expect(&TokenKind::RBracket)?;
            return Ok(AssignTarget::Index {
                object: Box::new(expr),
                index: Box::new(index),
                span: start,
            });
        }
        if let Expr::Member { object, name, span } = expr {
            return Ok(AssignTarget::Member {
                object,
                name,
                span,
            });
        }
        if let Expr::Ident { name, span } = expr {
            return Ok(AssignTarget::Name {
                name,
                ty: None,
                span,
            });
        }
        Err(ParseError::new(
            ParseErrorKind::InvalidAssignmentTarget,
            start,
            "invalid assignment target",
        ))
    }

    fn parse_assign_op(&mut self) -> Option<AssignOp> {
        let op = match self.peek_kind() {
            Some(TokenKind::Eq) => AssignOp::Assign,
            Some(TokenKind::PlusEq) => AssignOp::AddAssign,
            Some(TokenKind::MinusEq) => AssignOp::SubAssign,
            Some(TokenKind::StarEq) => AssignOp::MulAssign,
            Some(TokenKind::SlashEq) => AssignOp::DivAssign,
            Some(TokenKind::PercentEq) => AssignOp::ModAssign,
            _ => return None,
        };
        self.bump();
        Some(op)
    }

    fn parse_param_list(&mut self) -> ParseResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(&TokenKind::RParen) {
            return Ok(params);
        }
        loop {
            let start = self.current_span();
            let name = self.expect_ident_raw()?;
            let ty = if self.check(&TokenKind::Colon) {
                self.bump();
                Some(self.parse_type_expr()?)
            } else {
                None
            };
            let default = if self.check(&TokenKind::Eq) {
                self.bump();
                Some(self.parse_expr()?)
            } else {
                None
            };
            params.push(Param {
                name,
                ty,
                default,
                span: span_between(start, self.previous_span()),
            });
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.bump();
            if self.check(&TokenKind::RParen) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_expr_list(&mut self) -> ParseResult<Vec<Expr>> {
        let mut exprs = vec![self.parse_expr()?];
        while self.check(&TokenKind::Comma) {
            self.bump();
            exprs.push(self.parse_expr()?);
        }
        Ok(exprs)
    }

    // --- Expression parsing (precedence climbing) ---

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_with_expr()
    }

    fn parse_with_expr(&mut self) -> ParseResult<Expr> {
        let mut parts = vec![self.parse_or_expr()?];
        while self.check_keyword(Keyword::With) {
            self.bump();
            parts.push(self.parse_or_expr()?);
        }
        if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            let span = span_between(parts.first().unwrap().span(), parts.last().unwrap().span());
            Ok(Expr::With { parts, span })
        }
    }

    fn parse_or_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_and_expr()?;
        while self.check_keyword(Keyword::Or) || self.check(&TokenKind::OrOr) {
            let op_start = self.bump().span;
            let right = self.parse_and_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_not_expr()?;
        while self.check_keyword(Keyword::And) || self.check(&TokenKind::AndAnd) {
            let op_start = self.bump().span;
            let right = self.parse_not_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_not_expr(&mut self) -> ParseResult<Expr> {
        if self.check_keyword(Keyword::Not) || self.check(&TokenKind::Bang) {
            let start = self.bump().span;
            let expr = self.parse_not_expr()?;
            let end = expr.span();
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
                span: span_between(start, end),
            });
        }
        self.parse_comparison_expr()
    }

    fn parse_comparison_expr(&mut self) -> ParseResult<Expr> {
        let left = self.parse_bitwise_or_expr()?;
        if self.check_ident("is") {
            self.bump();
            let negated = if self.check_keyword(Keyword::Not) {
                self.bump();
                true
            } else {
                false
            };
            let left_span = left.span();
            let ty = self.parse_type_expr()?;
            let ty_span = ty.span();
            return Ok(Expr::Is {
                expr: Box::new(left),
                ty,
                negated,
                span: span_between(left_span, ty_span),
            });
        }
        let mut left = left;
        while let Some(op) = self.peek_comparison_op() {
            let op_start = self.bump().span;
            let right = self.parse_bitwise_or_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_bitwise_xor_expr()?;
        while self.check(&TokenKind::Pipe) && !self.check(&TokenKind::OrOr) {
            let op_start = self.bump().span;
            let right = self.parse_bitwise_xor_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op: BinaryOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_bitwise_and_expr()?;
        while self.check(&TokenKind::Caret) {
            let op_start = self.bump().span;
            let right = self.parse_bitwise_and_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op: BinaryOp::BitXor,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_shift_expr()?;
        while self.check(&TokenKind::Amp) && !self.check(&TokenKind::AndAnd) {
            let op_start = self.bump().span;
            let right = self.parse_shift_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op: BinaryOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(op_start, end),
            };
        }
        Ok(left)
    }

    fn parse_shift_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_additive_expr()?;
        while matches!(
            self.peek_kind(),
            Some(TokenKind::Shl) | Some(TokenKind::Shr)
        ) {
            let op = match self.bump().kind {
                TokenKind::Shl => BinaryOp::Shl,
                TokenKind::Shr => BinaryOp::Shr,
                _ => unreachable!(),
            };
            let right = self.parse_additive_expr()?;
            let left_span = left.span();
            let end = right.span();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(left_span, end),
            };
        }
        Ok(left)
    }

    fn parse_additive_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_multiplicative_expr()?;
        while matches!(self.peek_kind(), Some(TokenKind::Plus) | Some(TokenKind::Minus)) {
            let tok = self.bump();
            let op = if tok.kind == TokenKind::Plus {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            let right = self.parse_multiplicative_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(tok.span, end),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_power_expr()?;
        while matches!(
            self.peek_kind(),
            Some(TokenKind::Star)
                | Some(TokenKind::Slash)
                | Some(TokenKind::Percent)
                | Some(TokenKind::IntDiv)
        ) {
            let tok = self.bump();
            let op = match tok.kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                TokenKind::IntDiv => BinaryOp::IntDiv,
                _ => unreachable!(),
            };
            let right = self.parse_power_expr()?;
            let end = right.span();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(tok.span, end),
            };
        }
        Ok(left)
    }

    fn parse_power_expr(&mut self) -> ParseResult<Expr> {
        let left = self.parse_unary_expr()?;
        if self.check(&TokenKind::Pow) {
            self.bump();
            let right = self.parse_power_expr()?;
            let left_span = left.span();
            let end = right.span();
            return Ok(Expr::Binary {
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
                span: span_between(left_span, end),
            });
        }
        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> ParseResult<Expr> {
        if matches!(
            self.peek_kind(),
            Some(TokenKind::Minus) | Some(TokenKind::Plus) | Some(TokenKind::Tilde)
        ) {
            let tok = self.bump();
            let op = match tok.kind {
                TokenKind::Minus => UnaryOp::Neg,
                TokenKind::Plus => UnaryOp::Pos,
                TokenKind::Tilde => UnaryOp::BitNot,
                _ => unreachable!(),
            };
            let expr = self.parse_unary_expr()?;
            let end = expr.span();
            return Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
                span: span_between(tok.span, end),
            });
        }
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary_expr()?;
        loop {
            match self.peek_kind() {
                Some(TokenKind::LParen) => {
                    let start = expr.span();
                    self.bump();
                    let args = self.parse_call_args()?;
                    self.expect(&TokenKind::RParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        span: span_between(start, self.previous_span()),
                    };
                }
                Some(TokenKind::LBracket) => {
                    let start = expr.span();
                    self.bump();
                    let index = self.parse_expr()?;
                    self.expect(&TokenKind::RBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span: span_between(start, self.previous_span()),
                    };
                }
                Some(TokenKind::Dot) => {
                    let start = expr.span();
                    self.bump();
                    let name = self.expect_ident_raw()?;
                    expr = Expr::Member {
                        object: Box::new(expr),
                        name,
                        span: span_between(start, self.previous_span()),
                    };
                }
                Some(TokenKind::NullCoalesce) => {
                    let start = expr.span();
                    self.bump();
                    let right = self.parse_unary_expr()?;
                    let end = right.span();
                    expr = Expr::NullCoalesce {
                        left: Box::new(expr),
                        right: Box::new(right),
                        span: span_between(start, end),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        let tok = self.bump();
        let span = tok.span;

        match tok.kind {
            TokenKind::Int(n) => Ok(Expr::Int { value: n, span }),
            TokenKind::Float(n) => Ok(Expr::Float { value: n, span }),
            TokenKind::String(s) => Ok(Expr::String { value: s, span }),
            TokenKind::Char(c) => Ok(Expr::Char { value: c, span }),
            TokenKind::Bool(b) => Ok(Expr::Bool { value: b, span }),
            TokenKind::Keyword(Keyword::True) => Ok(Expr::Bool { value: true, span }),
            TokenKind::Keyword(Keyword::False) => Ok(Expr::Bool { value: false, span }),
            TokenKind::Keyword(Keyword::Null) => Ok(Expr::Null { span }),
            TokenKind::Keyword(Keyword::SelfKw) => Ok(Expr::SelfExpr { span }),
            TokenKind::Keyword(Keyword::Super) => Ok(Expr::Super { span }),
            TokenKind::Keyword(Keyword::This) => Ok(Expr::This { span }),
            TokenKind::Keyword(Keyword::Input) => self.finish_input_expr(span),
            TokenKind::Keyword(Keyword::If) => self.finish_if_expr(span),
            TokenKind::Keyword(Keyword::WaitFor) => {
                let url = self.parse_or_expr()?;
                let end = url.span();
                Ok(Expr::Http {
                    kind: HttpKind::WaitFor,
                    url: Box::new(url),
                    body: None,
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) if name == "read" => {
                let path = self.expect_string()?;
                Ok(Expr::Read {
                    kind: ReadKind::Text,
                    path,
                    span,
                })
            }
            TokenKind::Ident(name) if name == "read_bytes" => {
                let path = self.expect_string()?;
                Ok(Expr::Read {
                    kind: ReadKind::Bytes,
                    path,
                    span,
                })
            }
            TokenKind::Ident(name) if name == "read_lines" => {
                let path = self.expect_string()?;
                Ok(Expr::Read {
                    kind: ReadKind::Lines,
                    path,
                    span,
                })
            }
            TokenKind::Ident(name) if name == "get" => {
                let url = self.parse_or_expr()?;
                let end = url.span();
                Ok(Expr::Http {
                    kind: HttpKind::Get,
                    url: Box::new(url),
                    body: None,
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) if name == "post" => {
                let url = self.parse_or_expr()?;
                self.expect_keyword(Keyword::With)?;
                let body = self.parse_expr()?;
                let end = body.span();
                Ok(Expr::Http {
                    kind: HttpKind::Post,
                    url: Box::new(url),
                    body: Some(Box::new(body)),
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) if name == "put" => {
                let url = self.parse_or_expr()?;
                let url_end = url.span();
                let body = if self.check_keyword(Keyword::With) {
                    self.bump();
                    Some(Box::new(self.parse_expr()?))
                } else {
                    None
                };
                let end = body.as_ref().map(|b| b.span()).unwrap_or(url_end);
                Ok(Expr::Http {
                    kind: HttpKind::Put,
                    url: Box::new(url),
                    body,
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) if name == "delete" => {
                let url = self.parse_or_expr()?;
                let end = url.span();
                Ok(Expr::Http {
                    kind: HttpKind::Delete,
                    url: Box::new(url),
                    body: None,
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) if name == "patch" => {
                let url = self.parse_or_expr()?;
                let url_end = url.span();
                let body = if self.check_keyword(Keyword::With) {
                    self.bump();
                    Some(Box::new(self.parse_expr()?))
                } else {
                    None
                };
                let end = body.as_ref().map(|b| b.span()).unwrap_or(url_end);
                Ok(Expr::Http {
                    kind: HttpKind::Patch,
                    url: Box::new(url),
                    body,
                    span: span_between(span, end),
                })
            }
            TokenKind::Ident(name) => {
                if self.check(&TokenKind::LParen) {
                    if is_type_name(&name) {
                        return self.parse_object_expr(name, span);
                    }
                    return self.parse_call_expr(name, span);
                }
                Ok(Expr::Ident { name, span })
            }
            TokenKind::LBracket => self.parse_list_expr_inner(span),
            TokenKind::LBrace => self.parse_brace_expr_inner(span),
            TokenKind::LParen => self.parse_group_or_tuple_inner(span),
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                span,
                format!("unexpected token in expression: {}", tok.kind),
            )),
        }
    }

    fn finish_input_expr(&mut self, start: Span) -> ParseResult<Expr> {
        let input_type = if matches!(
            self.peek_kind(),
            Some(TokenKind::InputTypeKeyword(_))
        ) {
            match self.bump().kind {
                TokenKind::InputTypeKeyword(t) => Some(t),
                _ => None,
            }
        } else {
            None
        };
        let prompt = self.expect_string()?;
        Ok(Expr::Input {
            input_type,
            prompt,
            span: span_between(start, self.previous_span()),
        })
    }

    fn finish_if_expr(&mut self, start: Span) -> ParseResult<Expr> {
        let condition = self.parse_expr()?;
        if self.check_ident("then") {
            self.bump();
            let then_expr = self.parse_expr()?;
            self.expect_ident("else")?;
            let else_expr = self.parse_expr()?;
            let end = else_expr.span();
            return Ok(Expr::If {
                condition: Box::new(condition),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                span: span_between(start, end),
            });
        }
        self.expect(&TokenKind::Comma)?;
        let then_expr = self.parse_expr()?;
        self.expect_keyword(Keyword::Otherwise)?;
        self.expect(&TokenKind::Comma)?;
        let else_expr = self.parse_expr()?;
        let end = else_expr.span();
        Ok(Expr::If {
            condition: Box::new(condition),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
            span: span_between(start, end),
        })
    }

    fn parse_call_expr(&mut self, name: String, start: Span) -> ParseResult<Expr> {
        self.expect(&TokenKind::LParen)?;
        let args = if self.check(&TokenKind::RParen) {
            vec![]
        } else {
            self.parse_call_args()?
        };
        self.expect(&TokenKind::RParen)?;
        Ok(Expr::Call {
            callee: Box::new(Expr::Ident { name, span: start }),
            args,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_object_expr(&mut self, name: String, start: Span) -> ParseResult<Expr> {
        let ty = TypeExpr::Named {
            name,
            generics: vec![],
            span: start,
        };
        self.expect(&TokenKind::LParen)?;
        if self.check(&TokenKind::RParen) {
            self.bump();
            if self.check(&TokenKind::Comma) {
                self.bump();
                let fields = self.parse_indented_block(BlockClose::DoubleDot)?;
                let fields_span = fields.span;
                return Ok(Expr::Object {
                    ty,
                    args: vec![],
                    fields: Some(fields),
                    span: span_between(start, fields_span),
                });
            }
            return Ok(Expr::Object {
                ty,
                args: vec![],
                fields: None,
                span: start,
            });
        }
        let args = self.parse_call_args()?;
        self.expect(&TokenKind::RParen)?;
        Ok(Expr::Object {
            ty,
            args,
            fields: None,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_list_expr(&mut self) -> ParseResult<Expr> {
        let start = self.bump_kind(&TokenKind::LBracket)?;
        self.parse_list_expr_inner(start)
    }

    fn parse_list_expr_inner(&mut self, start: Span) -> ParseResult<Expr> {
        let mut elements = Vec::new();
        self.skip_newlines();
        if !self.check(&TokenKind::RBracket) {
            loop {
                elements.push(self.parse_expr()?);
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.bump();
                self.skip_newlines();
                if self.check(&TokenKind::RBracket) {
                    break;
                }
            }
        }
        self.skip_newlines();
        self.expect(&TokenKind::RBracket)?;
        Ok(Expr::List {
            elements,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_dict_expr(&mut self) -> ParseResult<Expr> {
        let start = self.bump_kind(&TokenKind::LBrace)?;
        self.parse_brace_expr_inner(start)
    }

    /// `{a: 1, b: 2}` → Dict; `{1, 2, 3}` → Set (colon disambiguates).
    fn parse_brace_expr_inner(&mut self, start: Span) -> ParseResult<Expr> {
        self.skip_newlines();
        if self.check(&TokenKind::RBrace) {
            self.bump();
            return Ok(Expr::Dict {
                entries: vec![],
                span: span_between(start, self.previous_span()),
            });
        }
        let first = self.parse_expr()?;
        if self.check(&TokenKind::Colon) {
            let mut entries = vec![];
            self.bump();
            self.skip_newlines();
            entries.push((first, self.parse_expr()?));
            while self.check(&TokenKind::Comma) {
                self.bump();
                self.skip_newlines();
                if self.check(&TokenKind::RBrace) {
                    break;
                }
                let key = self.parse_expr()?;
                self.expect(&TokenKind::Colon)?;
                self.skip_newlines();
                entries.push((key, self.parse_expr()?));
            }
            self.skip_newlines();
            self.expect(&TokenKind::RBrace)?;
            return Ok(Expr::Dict {
                entries,
                span: span_between(start, self.previous_span()),
            });
        }
        let mut elements = vec![first];
        while self.check(&TokenKind::Comma) {
            self.bump();
            self.skip_newlines();
            if self.check(&TokenKind::RBrace) {
                break;
            }
            elements.push(self.parse_expr()?);
        }
        self.skip_newlines();
        self.expect(&TokenKind::RBrace)?;
        Ok(Expr::Set {
            elements,
            span: span_between(start, self.previous_span()),
        })
    }

    fn parse_dict_expr_inner(&mut self, start: Span) -> ParseResult<Expr> {
        self.parse_brace_expr_inner(start)
    }

    fn parse_group_or_tuple(&mut self) -> ParseResult<Expr> {
        let start = self.bump_kind(&TokenKind::LParen)?;
        self.parse_group_or_tuple_inner(start)
    }

    fn parse_group_or_tuple_inner(&mut self, start: Span) -> ParseResult<Expr> {
        if self.check(&TokenKind::RParen) {
            self.bump();
            return Ok(Expr::Tuple {
                elements: vec![],
                span: span_between(start, self.previous_span()),
            });
        }
        // Lambda?
        if self.is_lambda_params() {
            let params = self.parse_param_list()?;
            self.expect(&TokenKind::RParen)?;
            if self.check(&TokenKind::FatArrow) {
                self.bump();
                let body = if self.check(&TokenKind::Comma) {
                    self.bump();
                    LambdaBody::Block(self.parse_indented_block(BlockClose::DoubleDot)?)
                } else {
                    LambdaBody::Expr(Box::new(self.parse_expr()?))
                };
                return Ok(Expr::Lambda {
                    params,
                    body,
                    span: span_between(start, self.previous_span()),
                });
            }
        }
        let first = self.parse_expr()?;
        if self.check(&TokenKind::Comma) {
            let mut elements = vec![first];
            while self.check(&TokenKind::Comma) {
                self.bump();
                if self.check(&TokenKind::RParen) {
                    break;
                }
                elements.push(self.parse_expr()?);
            }
            self.expect(&TokenKind::RParen)?;
            return Ok(Expr::Tuple {
                elements,
                span: span_between(start, self.previous_span()),
            });
        }
        self.expect(&TokenKind::RParen)?;
        Ok(first)
    }

    fn parse_call_args(&mut self) -> ParseResult<Vec<Arg>> {
        let mut args = Vec::new();
        if self.check(&TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            let start = self.current_span();
            let (name, value) = if let Some(ident) = self.peek_ident() {
                let checkpoint = self.pos;
                self.bump();
                if self.check(&TokenKind::Eq) {
                    self.bump();
                    (Some(ident), self.parse_expr()?)
                } else {
                    self.pos = checkpoint;
                    (None, self.parse_expr()?)
                }
            } else {
                (None, self.parse_expr()?)
            };
            args.push(Arg {
                name,
                value,
                span: span_between(start, self.previous_span()),
            });
            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.bump();
            if self.check(&TokenKind::RParen) {
                break;
            }
        }
        Ok(args)
    }

    fn parse_type_expr(&mut self) -> ParseResult<TypeExpr> {
        let start = self.current_span();
        let base = if self.check(&TokenKind::LParen) {
            self.bump();
            let mut types = vec![self.parse_type_expr()?];
            while self.check(&TokenKind::Comma) {
                self.bump();
                types.push(self.parse_type_expr()?);
            }
            self.expect(&TokenKind::RParen)?;
            TypeExpr::Tuple {
                types,
                span: span_between(start, self.previous_span()),
            }
        } else {
            let name = self.expect_ident_raw()?;
            let generics = if self.check(&TokenKind::Lt) {
                self.bump();
                let mut gs = vec![self.parse_type_expr()?];
                while self.check(&TokenKind::Comma) {
                    self.bump();
                    gs.push(self.parse_type_expr()?);
                }
                self.expect(&TokenKind::Gt)?;
                gs
            } else {
                vec![]
            };
            TypeExpr::Named {
                name,
                generics,
                span: start,
            }
        };

        if self.check(&TokenKind::Question) {
            self.bump();
            Ok(TypeExpr::Optional {
                inner: Box::new(base),
                span: span_between(start, self.previous_span()),
            })
        } else {
            Ok(base)
        }
    }

    // --- Token helpers ---

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    fn peek_keyword(&self) -> Option<Keyword> {
        match self.peek_kind()? {
            TokenKind::Keyword(k) => Some(*k),
            _ => None,
        }
    }

    fn peek_ident(&self) -> Option<String> {
        match self.peek_kind()? {
            TokenKind::Ident(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn is_lambda_params(&self) -> bool {
        let mut i = self.pos;
        let mut depth = 0;
        while i < self.tokens.len() {
            match &self.tokens[i].kind {
                TokenKind::LParen => depth += 1,
                TokenKind::RParen => {
                    if depth == 0 {
                        i += 1;
                        return self
                            .tokens
                            .get(i)
                            .is_some_and(|t| t.kind == TokenKind::FatArrow);
                    }
                    depth -= 1;
                }
                // Arithmetic / logic at the top level of `( … )` → grouped expr, not lambda.
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::IntDiv
                | TokenKind::Eq
                | TokenKind::EqEq
                | TokenKind::NotEq
                | TokenKind::Lt
                | TokenKind::Gt
                | TokenKind::LtEq
                | TokenKind::GtEq
                | TokenKind::AndAnd
                | TokenKind::OrOr
                | TokenKind::Keyword(Keyword::With)
                | TokenKind::Keyword(Keyword::And)
                | TokenKind::Keyword(Keyword::Or) => {
                    if depth == 0 {
                        return false;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        false
    }

    fn is_assign_start(&self) -> bool {
        self.is_ident() || self.check_keyword(Keyword::SelfKw)
    }

    fn is_ident(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Ident(_)))
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    fn check_keyword(&self, kw: Keyword) -> bool {
        self.peek_keyword() == Some(kw)
    }

    fn check_ident(&self, name: &str) -> bool {
        self.peek_ident().as_deref() == Some(name)
    }

    fn check_stmt_end(&self) -> bool {
        self.check(&TokenKind::StmtEnd)
    }

    fn bump(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        if !matches!(tok.kind, TokenKind::Eof) {
            self.pos += 1;
        }
        tok
    }

    fn bump_kind(&mut self, kind: &TokenKind) -> ParseResult<Span> {
        if self.check(kind) {
            Ok(self.bump().span)
        } else {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                format!("expected {:?}", kind),
            ))
        }
    }

    fn bump_keyword(&mut self, kw: Keyword) -> ParseResult<Span> {
        if self.check_keyword(kw) {
            Ok(self.bump().span)
        } else {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                format!("expected keyword '{}'", kw.as_str()),
            ))
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> ParseResult<()> {
        if self.check(kind) {
            self.bump();
            Ok(())
        } else {
            let found = self
                .peek_kind()
                .map(|k| crate::error::token_label(&k).to_string())
                .unwrap_or_else(|| "end of file".into());
            let expected = crate::error::token_label(kind);
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                format!("expected {expected}, found {found}"),
            ))
        }
    }

    fn expect_keyword(&mut self, kw: Keyword) -> ParseResult<()> {
        if self.check_keyword(kw) {
            self.bump();
            Ok(())
        } else {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                format!("expected keyword '{}'", kw.as_str()),
            ))
        }
    }

    fn expect_ident(&mut self, expected: &str) -> ParseResult<()> {
        let name = self.expect_ident_raw()?;
        if name == expected {
            Ok(())
        } else {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.previous_span(),
                format!("expected '{}', found '{}'", expected, name),
            ))
        }
    }

    fn expect_ident_raw(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::Ident(_)) => {
                let tok = self.bump();
                match tok.kind {
                    TokenKind::Ident(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                "expected identifier",
            )),
        }
    }

    fn expect_string(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::String(_)) => {
                let tok = self.bump();
                match tok.kind {
                    TokenKind::String(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                self.current_span(),
                "expected string literal",
            )),
        }
    }

    fn expect_stmt_end(&mut self) -> ParseResult<()> {
        if self.check(&TokenKind::StmtEnd) {
            self.bump();
            return Ok(());
        }
        Err(self.stmt_end_error())
    }

    /// After `name = User(), ... ..` the block close ends the statement (no extra `.`).
    fn expect_stmt_end_after_expr(&mut self, expr: &Expr) -> ParseResult<()> {
        if self.check_stmt_end() {
            return self.expect_stmt_end();
        }
        if matches!(expr, Expr::Object { fields: Some(_), .. }) {
            return Ok(());
        }
        Err(self.stmt_end_error())
    }

    fn stmt_end_error(&self) -> ParseError {
        let span = self.current_span();
        let message = match self.peek_kind() {
            Some(TokenKind::BlockClose) => {
                "this line should end with `.`, not `..`\n  \
                 help: use `.` to end a statement inside a block; use `..` only on the dedented line to close the block"
                    .into()
            }
            Some(TokenKind::Newline) | None => {
                "every statement must end with `.`\n  \
                 help: add a period at the end of this line"
                    .into()
            }
            Some(found) => format!(
                "every statement must end with `.`, found {}\n  \
                 help: add `.` at the end of this line",
                crate::error::token_label(&found)
            ),
        };
        ParseError::new(ParseErrorKind::MissingStatementEnd, span, message)
    }

    fn block_close_error(&self) -> ParseError {
        let span = self.current_span();
        let message = match self.peek_kind() {
            Some(TokenKind::StmtEnd) => {
                // Common typos: `.` or `.,.` instead of `..`
                "this block must close with `..`, not `.`\n  \
                 help: replace `.` with `..` on this line (e.g. `function greet(name),` … `..`)"
                    .into()
            }
            Some(TokenKind::Comma) => {
                "this block must close with `..`\n  \
                 help: put `..` alone on this dedented line — remove the extra `,`"
                    .into()
            }
            Some(TokenKind::DotDot) => {
                "put `..` at the beginning of this line to close the block\n  \
                 help: remove any `.` or `,` before `..`"
                    .into()
            }
            Some(TokenKind::Newline) | None => {
                "this block is not closed\n  \
                 help: add `..` on a dedented line after the block body"
                    .into()
            }
            Some(found) => format!(
                "this block must close with `..`, found {}\n  \
                 help: add `..` on a dedented line to close the block",
                crate::error::token_label(&found)
            ),
        };
        ParseError::new(ParseErrorKind::MissingBlockClose, span, message)
    }

    fn expect_block_close(&mut self) -> ParseResult<()> {
        if self.check(&TokenKind::BlockClose) {
            self.bump();
            Ok(())
        } else {
            Err(self.block_close_error())
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.bump();
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Eof) | None)
    }

    fn current_span(&self) -> Span {
        self.peek().map(|t| t.span).unwrap_or(Span::default())
    }

    fn previous_span(&self) -> Span {
        self.tokens
            .get(self.pos.saturating_sub(1))
            .map(|t| t.span)
            .unwrap_or_default()
    }

    fn peek_comparison_op(&self) -> Option<BinaryOp> {
        match self.peek_kind()? {
            TokenKind::EqEq => Some(BinaryOp::Eq),
            TokenKind::NotEq => Some(BinaryOp::NotEq),
            TokenKind::Lt => Some(BinaryOp::Lt),
            TokenKind::Gt => Some(BinaryOp::Gt),
            TokenKind::LtEq => Some(BinaryOp::LtEq),
            TokenKind::GtEq => Some(BinaryOp::GtEq),
            _ => None,
        }
    }
}

fn item_span(item: &ModuleItem) -> Span {
    match item {
        ModuleItem::Use(u) => u.span,
        ModuleItem::Function(f) => f.span,
        ModuleItem::Type(t) => t.span,
        ModuleItem::Enum(e) => e.span,
        ModuleItem::EventHandler(e) => e.span,
        ModuleItem::Stmt(s) => stmt_span(s),
    }
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Assign { span, .. } => *span,
        Stmt::Print { span, .. } => *span,
        Stmt::Return { span, .. } => *span,
        Stmt::Break { span } => *span,
        Stmt::Continue { span } => *span,
        Stmt::If(i) => i.span,
        Stmt::Repeat(r) => match r {
            RepeatStmt::Count { span, .. } | RepeatStmt::Forever { span, .. } => *span,
        },
        Stmt::For(f) => f.span,
        Stmt::While(w) => w.span,
        Stmt::Try(t) => t.span,
        Stmt::Write { span, .. } => *span,
        Stmt::Io(i) => match i {
            IoStmt::Copy { span, .. }
            | IoStmt::Move { span, .. }
            | IoStmt::Rename { span, .. }
            | IoStmt::Delete { span, .. } => *span,
        },
        Stmt::Expr { span, .. } => *span,
        Stmt::Pass { span } => *span,
    }
}

trait ExprSpan {
    fn span(&self) -> Span;
}

impl ExprSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Int { span, .. }
            | Expr::Float { span, .. }
            | Expr::String { span, .. }
            | Expr::Char { span, .. }
            | Expr::Bool { span, .. }
            | Expr::Null { span }
            | Expr::Ident { span, .. }
            | Expr::SelfExpr { span }
            | Expr::Super { span }
            | Expr::This { span }
            | Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Call { span, .. }
            | Expr::Member { span, .. }
            | Expr::Index { span, .. }
            | Expr::With { span, .. }
            | Expr::Input { span, .. }
            | Expr::Read { span, .. }
            | Expr::Http { span, .. }
            | Expr::List { span, .. }
            | Expr::Dict { span, .. }
            | Expr::Set { span, .. }
            | Expr::Tuple { span, .. }
            | Expr::Object { span, .. }
            | Expr::Lambda { span, .. }
            | Expr::If { span, .. }
            | Expr::Is { span, .. }
            | Expr::NullCoalesce { span, .. } => *span,
        }
    }
}

trait TypeExprSpan {
    fn span(&self) -> Span;
}

impl TypeExprSpan for TypeExpr {
    fn span(&self) -> Span {
        match self {
            TypeExpr::Named { span, .. } | TypeExpr::Optional { span, .. } | TypeExpr::Tuple { span, .. } => *span,
        }
    }
}

/// PascalCase identifiers followed by `(` create objects; others are calls.
fn is_type_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|c| c.is_uppercase() || c == '_')
}

/// Lex and parse source into a Program AST.
pub fn parse(source: &str) -> ParseResult<Program> {
    let tokens = langp_lexer::lex(source).map_err(|e| {
        ParseError::new(
            ParseErrorKind::UnexpectedToken,
            e.span,
            format!("lexical error: {}", e.message),
        )
    })?;
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hello() {
        let source = r#"print "Hello, Lang.P!"."#;
        let program = parse(source).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn parse_if_block() {
        let source = "if true,\n    print \"yes\".\n..";
        let program = parse(source).unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn parse_print_with_parenthesized_arithmetic() {
        let source = r#"num1 = input number "a".
num2 = input number "b".
print "Sum : " with (num1 + num2)."#;
        let program = parse(source).unwrap();
        assert_eq!(program.items.len(), 3);
    }
}
