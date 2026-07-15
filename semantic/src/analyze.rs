use crate::diagnostic::{Diagnostic, DiagnosticKind, Severity};
use crate::types;
use langp_ast::*;
use langp_lexer::Span;
use std::collections::{HashMap, HashSet};

pub struct AnalysisResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisResult {
    pub fn is_ok(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
}

pub fn analyze(program: &Program) -> AnalysisResult {
    let mut checker = Checker::new();
    checker.check_program(program);
    AnalysisResult {
        diagnostics: checker.diagnostics,
    }
}

struct Checker {
    globals: HashSet<String>,
    functions: HashMap<String, Span>,
    diagnostics: Vec<Diagnostic>,
}

impl Checker {
    fn new() -> Self {
        Self {
            globals: HashSet::new(),
            functions: HashMap::new(),
            diagnostics: Vec::new(),
        }
    }

    fn error(&mut self, kind: DiagnosticKind, span: Span, message: impl Into<String>) {
        self.diagnostics
            .push(Diagnostic::error(kind, span, message));
    }

    fn check_program(&mut self, program: &Program) {
        for item in &program.items {
            self.collect_item(item);
        }
        let mut module_locals = HashSet::new();
        for item in &program.items {
            if let ModuleItem::Use(u) = item {
                if let Some(name) = u.path.first() {
                    module_locals.insert(name.clone());
                }
            }
        }
        for item in &program.items {
            match item {
                ModuleItem::Stmt(stmt) => {
                    self.check_stmt(stmt, &module_locals);
                    self.collect_bindings(stmt, &mut module_locals);
                }
                _ => self.check_item(item),
            }
        }
    }

    fn collect_item(&mut self, item: &ModuleItem) {
        match item {
            ModuleItem::Function(f) => {
                if self.functions.contains_key(&f.name) {
                    self.error(
                        DiagnosticKind::DuplicateDefinition,
                        f.span,
                        format!("duplicate function '{}'", f.name),
                    );
                } else {
                    self.functions.insert(f.name.clone(), f.span);
                    self.globals.insert(f.name.clone());
                }
            }
            ModuleItem::Type(t) => {
                if self.globals.contains(&t.name) {
                    self.error(
                        DiagnosticKind::DuplicateDefinition,
                        t.span,
                        format!("duplicate type '{}'", t.name),
                    );
                } else {
                    self.globals.insert(t.name.clone());
                }
            }
            ModuleItem::Enum(e) => {
                if self.globals.contains(&e.name) {
                    self.error(
                        DiagnosticKind::DuplicateDefinition,
                        e.span,
                        format!("duplicate enum '{}'", e.name),
                    );
                } else {
                    self.globals.insert(e.name.clone());
                }
            }
            ModuleItem::Use(u) => {
                if let Some(name) = u.path.first() {
                    self.globals.insert(name.clone());
                }
            }
            _ => {}
        }
    }

    fn check_item(&mut self, item: &ModuleItem) {
        match item {
            ModuleItem::Function(f) => {
                let mut locals = HashSet::new();
                for p in &f.params {
                    locals.insert(p.name.clone());
                }
                self.check_block(&f.body, &locals);
            }
            ModuleItem::Stmt(stmt) => self.check_stmt(stmt, &HashSet::new()),
            _ => {}
        }
    }

    fn check_block(&mut self, block: &Block, locals: &HashSet<String>) {
        let mut scope = locals.clone();
        for stmt in &block.statements {
            self.check_stmt(stmt, &scope);
            self.collect_bindings(stmt, &mut scope);
        }
    }

    fn collect_bindings(&mut self, stmt: &Stmt, scope: &mut HashSet<String>) {
        match stmt {
            Stmt::Assign { target, .. } => match target {
                AssignTarget::Name { name, .. } => {
                    scope.insert(name.clone());
                }
                AssignTarget::Tuple(names, _) => {
                    for n in names {
                        scope.insert(n.clone());
                    }
                }
                _ => {}
            },
            Stmt::For(f) => match &f.binding {
                ForBinding::Single(name, _) => {
                    scope.insert(name.clone());
                }
                ForBinding::KeyValue(k, v, _) => {
                    scope.insert(k.clone());
                    scope.insert(v.clone());
                }
            },
            Stmt::Repeat(r) => {
                if let RepeatStmt::Count { var: Some(name), .. } = r {
                    scope.insert(name.clone());
                }
            },
            Stmt::Try(t) => {
                for c in &t.catches {
                    scope.insert(c.name.clone());
                }
            }
            _ => {}
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, locals: &HashSet<String>) {
        match stmt {
            Stmt::Assign { target, value, .. } => {
                self.check_assign_target(target, locals);
                self.check_expr(value, locals);
                if let AssignTarget::Name { ty: Some(ann), span, .. } = target {
                    let expected = types::from_type_expr(ann);
                    let actual = types::infer_expr(value);
                    if !types::compatible(&expected, &actual) {
                        self.error(
                            DiagnosticKind::TypeMismatch,
                            *span,
                            format!(
                                "type mismatch: expected {}, got {}\n  \
                                 help: adjust the value or annotation",
                                types::type_label(&expected),
                                types::type_label(&actual)
                            ),
                        );
                    }
                }
            }
            Stmt::Print { parts, .. } => {
                for p in parts {
                    self.check_expr(p, locals);
                }
            }
            Stmt::Return { values, .. } => {
                for v in values {
                    self.check_expr(v, locals);
                }
            }
            Stmt::If(i) => {
                self.check_expr(&i.condition, locals);
                self.check_block(&i.then_block, locals);
                for e in &i.elif_clauses {
                    self.check_expr(&e.condition, locals);
                    self.check_block(&e.block, locals);
                }
                if let Some(b) = &i.else_block {
                    self.check_block(b, locals);
                }
            }
            Stmt::Repeat(r) => match r {
                RepeatStmt::Count { count, var, body, .. } => {
                    self.check_expr(count, locals);
                    let mut loop_locals = locals.clone();
                    if let Some(name) = var {
                        loop_locals.insert(name.clone());
                    }
                    self.check_block(body, &loop_locals);
                }
                RepeatStmt::Forever { body, .. } => self.check_block(body, locals),
            },
            Stmt::For(f) => {
                self.check_expr(&f.iterable, locals);
                let mut loop_locals = locals.clone();
                match &f.binding {
                    ForBinding::Single(name, _) => {
                        loop_locals.insert(name.clone());
                    }
                    ForBinding::KeyValue(k, v, _) => {
                        loop_locals.insert(k.clone());
                        loop_locals.insert(v.clone());
                    }
                }
                self.check_block(&f.body, &loop_locals);
            }
            Stmt::While(w) => {
                self.check_expr(&w.condition, locals);
                self.check_block(&w.body, locals);
            }
            Stmt::Try(t) => {
                self.check_block(&t.body, locals);
                for c in &t.catches {
                    let mut catch_locals = locals.clone();
                    catch_locals.insert(c.name.clone());
                    self.check_block(&c.body, &catch_locals);
                }
                if let Some(f) = &t.finally_block {
                    self.check_block(f, locals);
                }
            }
            Stmt::Write { value, destination, .. } => {
                self.check_expr(value, locals);
                self.check_expr(destination, locals);
            }
            Stmt::Io(io) => match io {
                IoStmt::Copy { source, dest, .. }
                | IoStmt::Move { source, dest, .. }
                | IoStmt::Rename { source, dest, .. } => {
                    self.check_expr(source, locals);
                    self.check_expr(dest, locals);
                }
                IoStmt::Delete { target, .. } => self.check_expr(target, locals),
            },
            Stmt::Expr { expr, .. } => self.check_expr(expr, locals),
            _ => {}
        }
    }

    fn check_assign_target(&mut self, target: &AssignTarget, locals: &HashSet<String>) {
        match target {
            AssignTarget::Name { .. } => {
                // Assignment defines the name; no "may be undefined" check on the target.
            }
            AssignTarget::Member { object, .. } | AssignTarget::Index { object, .. } => {
                self.check_expr(object, locals);
            }
            AssignTarget::Tuple(_, _) => {
                // Tuple unpacking defines each name.
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr, locals: &HashSet<String>) {
        match expr {
            Expr::Ident { name, span } => {
                if !locals.contains(name)
                    && !self.globals.contains(name)
                    && !self.functions.contains_key(name)
                    && !self.is_builtin(name)
                {
                    self.error(
                        DiagnosticKind::UndefinedName,
                        *span,
                        format!(
                            "'{name}' is used before it is defined\n  \
                             help: assign it first (e.g. {name} = ...) or check the spelling"
                        ),
                    );
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, locals);
                self.check_expr(right, locals);
            }
            Expr::Unary { expr, .. } => self.check_expr(expr, locals),
            Expr::Call { callee, args, .. } => {
                self.check_expr(callee, locals);
                for a in args {
                    self.check_expr(&a.value, locals);
                }
            }
            Expr::Member { object, .. } => {
                self.check_expr(object, locals);
            }
            Expr::Index { object, index, .. } => {
                self.check_expr(object, locals);
                self.check_expr(index, locals);
            }
            Expr::With { parts, .. } => {
                for p in parts {
                    self.check_expr(p, locals);
                }
            }
            Expr::List { elements, .. } | Expr::Tuple { elements, .. } | Expr::Set { elements, .. } => {
                for e in elements {
                    self.check_expr(e, locals);
                }
            }
            Expr::Dict { entries, .. } => {
                for (k, v) in entries {
                    if !matches!(k, Expr::Ident { .. } | Expr::String { .. }) {
                        self.check_expr(k, locals);
                    }
                    self.check_expr(v, locals);
                }
            }
            Expr::Object { args, fields, .. } => {
                for a in args {
                    self.check_expr(&a.value, locals);
                }
                if let Some(f) = fields {
                    self.check_block(f, locals);
                }
            }
            Expr::Lambda { params, body, .. } => {
                let mut lambda_locals = locals.clone();
                for p in params {
                    lambda_locals.insert(p.name.clone());
                }
                match body {
                    LambdaBody::Expr(e) => self.check_expr(e, &lambda_locals),
                    LambdaBody::Block(b) => self.check_block(b, &lambda_locals),
                }
            }
            Expr::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.check_expr(condition, locals);
                self.check_expr(then_expr, locals);
                self.check_expr(else_expr, locals);
            }
            Expr::Is { expr, .. } => self.check_expr(expr, locals),
            Expr::NullCoalesce { left, right, .. } => {
                self.check_expr(left, locals);
                self.check_expr(right, locals);
            }
            Expr::Http { url, body, .. } => {
                self.check_expr(url, locals);
                if let Some(b) = body {
                    self.check_expr(b, locals);
                }
            }
            _ => {}
        }
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(
            name,
            "len" | "to_string" | "assert" | "read" | "read_bytes" | "read_lines" | "print"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use langp_parser::parse;

    #[test]
    fn analyze_input_assign_no_false_warnings() {
        let source = r#"name = input text "Enter Your Name : ".
age = input number "Enter Your Age : ".
print "Hey " with name."#;
        let program = parse(source).unwrap();
        let result = analyze(&program);
        assert!(
            result.diagnostics.is_empty(),
            "expected no diagnostics, got {:?}",
            result.diagnostics
        );
    }
}
