use crate::builtins::register_builtins;
use crate::env::Environment;
use langp_ast::*;
use langp_lexer::{InputTypeKeyword, Span};
use langp_runtime::{RuntimeError, RuntimeErrorKind, RuntimeResult, UserFunction, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

pub struct RunResult {
    pub exit_code: i32,
}

enum Flow {
    None,
    Break(Span),
    Continue(Span),
    Return(Vec<Value>, Span),
}

pub struct Interpreter {
    globals: Rc<Environment>,
    functions: HashMap<String, Rc<FunctionDecl>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(Environment::new());
        register_builtins(&globals);
        Self {
            globals,
            functions: HashMap::new(),
        }
    }

    pub fn run_program(&mut self, program: &Program) -> RuntimeResult<RunResult> {
        for item in &program.items {
            if let ModuleItem::Function(f) = item {
                self.functions.insert(f.name.clone(), Rc::new(f.clone()));
                self.globals.define(
                    f.name.clone(),
                    Value::Function(Rc::new(UserFunction {
                        name: f.name.clone(),
                        params: f.params.clone(),
                        decl: Rc::new(f.clone()),
                    })),
                );
            }
        }

        for item in &program.items {
            match item {
                ModuleItem::Stmt(stmt) => {
                    self.exec_stmt(stmt, self.globals.clone())?;
                }
                ModuleItem::Function(f) => {
                    // Top-level function bodies are registered, not executed.
                    let _ = f;
                }
                _ => {}
            }
        }

        Ok(RunResult { exit_code: 0 })
    }

    fn exec_stmt(&mut self, stmt: &Stmt, env: Rc<Environment>) -> RuntimeResult<()> {
        match self.eval_stmt(stmt, env)? {
            Flow::None | Flow::Break(_) | Flow::Continue(_) => Ok(()),
            Flow::Return(_, span) => Err(RuntimeError::new(
                RuntimeErrorKind::Return,
                span,
                "return outside function",
            )),
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        match stmt {
            Stmt::Assign { target, op, value, span } => {
                let val = self.eval_expr(value, env.clone())?;
                self.assign(target, *op, val, env, *span)?;
                Ok(Flow::None)
            }
            Stmt::Print { inline, parts, .. } => {
                let mut chunks = Vec::new();
                for p in parts {
                    chunks.push(self.eval_expr(p, env.clone())?.to_string());
                }
                let text = chunks.join("");
                if *inline {
                    print!("{text}");
                } else {
                    println!("{text}");
                }
                io::stdout().flush().ok();
                Ok(Flow::None)
            }
            Stmt::Return { values, span } => {
                let mut vals = Vec::new();
                for v in values {
                    vals.push(self.eval_expr(v, env.clone())?);
                }
                Ok(Flow::Return(vals, *span))
            }
            Stmt::Break { span } => Ok(Flow::Break(*span)),
            Stmt::Continue { span } => Ok(Flow::Continue(*span)),
            Stmt::If(i) => self.eval_if(i, env),
            Stmt::Repeat(r) => self.eval_repeat(r, env),
            Stmt::For(f) => self.eval_for(f, env),
            Stmt::While(w) => self.eval_while(w, env),
            Stmt::Try(t) => self.eval_try(t, env),
            Stmt::Write { value, destination, span, .. } => {
                let val = self.eval_expr(value, env.clone())?;
                let dest = self.eval_expr(destination, env.clone())?;
                let path = value_to_string(&dest, *span)?;
                std::fs::write(&path, val.to_string()).map_err(|e| {
                    RuntimeError::new(
                        RuntimeErrorKind::IoError,
                        *span,
                        format!("write failed: {e}"),
                    )
                })?;
                Ok(Flow::None)
            }
            Stmt::Io(io) => {
                self.eval_io(io, env)?;
                Ok(Flow::None)
            }
            Stmt::Expr { expr, .. } => {
                self.eval_expr(expr, env)?;
                Ok(Flow::None)
            }
            Stmt::Pass { .. } => Ok(Flow::None),
        }
    }

    fn eval_if(&mut self, stmt: &IfStmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        if self.eval_expr(&stmt.condition, env.clone())?.is_truthy() {
            return self.eval_block_flow(&stmt.then_block, env);
        }
        for e in &stmt.elif_clauses {
            if self.eval_expr(&e.condition, env.clone())?.is_truthy() {
                return self.eval_block_flow(&e.block, env);
            }
        }
        if let Some(b) = &stmt.else_block {
            return self.eval_block_flow(b, env);
        }
        Ok(Flow::None)
    }

    fn eval_repeat(&mut self, stmt: &RepeatStmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        match stmt {
            RepeatStmt::Forever { body, .. } => loop {
                match self.eval_block_flow(body, env.clone())? {
                    Flow::Break(_) => break,
                    Flow::Continue(_) => continue,
                    Flow::Return(v, s) => return Ok(Flow::Return(v, s)),
                    Flow::None => {}
                }
            },
            RepeatStmt::Count { count, var, body, .. } => {
                let n = match self.eval_expr(count, env.clone())? {
                    Value::Int(n) => n,
                    other => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeError,
                            body.span,
                            format!("repeat count must be Int, got {}", other.type_name()),
                        ));
                    }
                };
                for i in 0..n {
                    let loop_env = if let Some(name) = var {
                        let child = Rc::new(Environment::child(env.clone()));
                        child.define(name.clone(), Value::Int(i));
                        child
                    } else {
                        env.clone()
                    };
                    match self.eval_block_flow(body, loop_env)? {
                        Flow::Break(_) => break,
                        Flow::Continue(_) => continue,
                        Flow::Return(v, s) => return Ok(Flow::Return(v, s)),
                        Flow::None => {}
                    }
                }
            }
        }
        Ok(Flow::None)
    }

    fn eval_for(&mut self, stmt: &ForStmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        let iterable = self.eval_expr(&stmt.iterable, env.clone())?;
        let items = value_to_iterable(&iterable, stmt.span)?;
        for item in items {
            let loop_env = Rc::new(Environment::child(env.clone()));
            match &stmt.binding {
                ForBinding::Single(name, _) => {
                    loop_env.define(name.clone(), item);
                }
                ForBinding::KeyValue(k, v, span) => {
                    let (key, val) = item_as_pair(&item, *span)?;
                    loop_env.define(k.clone(), key);
                    loop_env.define(v.clone(), val);
                }
            }
            match self.eval_block_flow(&stmt.body, loop_env)? {
                Flow::Break(_) => break,
                Flow::Continue(_) => continue,
                Flow::Return(vals, s) => return Ok(Flow::Return(vals, s)),
                Flow::None => {}
            }
        }
        Ok(Flow::None)
    }

    fn eval_while(&mut self, stmt: &WhileStmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        while self.eval_expr(&stmt.condition, env.clone())?.is_truthy() {
            match self.eval_block_flow(&stmt.body, env.clone())? {
                Flow::Break(_) => break,
                Flow::Continue(_) => continue,
                Flow::Return(v, s) => return Ok(Flow::Return(v, s)),
                Flow::None => {}
            }
        }
        Ok(Flow::None)
    }

    fn eval_try(&mut self, stmt: &TryStmt, env: Rc<Environment>) -> RuntimeResult<Flow> {
        let result = self.eval_block_flow(&stmt.body, env.clone());
        let mut flow = match result {
            Ok(f) => f,
            Err(e) => {
                let mut handled = false;
                let mut out = Flow::None;
                for c in &stmt.catches {
                    let catch_env = Rc::new(Environment::child(env.clone()));
                    catch_env.define(c.name.clone(), Value::String(e.message.clone()));
                    out = self.eval_block_flow(&c.body, catch_env)?;
                    handled = true;
                    break;
                }
                if !handled {
                    return Err(e);
                }
                out
            }
        };
        if let Some(finally) = &stmt.finally_block {
            let finally_flow = self.eval_block_flow(finally, env)?;
            if matches!(flow, Flow::None) {
                flow = finally_flow;
            }
        }
        Ok(flow)
    }

    fn eval_io(&mut self, io: &IoStmt, env: Rc<Environment>) -> RuntimeResult<()> {
        match io {
            IoStmt::Copy { source, dest, span } => {
                let src = value_to_string(&self.eval_expr(source, env.clone())?, *span)?;
                let dst = value_to_string(&self.eval_expr(dest, env)?, *span)?;
                std::fs::copy(&src, &dst).map_err(|e| io_err(*span, e))?;
            }
            IoStmt::Move { source, dest, span } => {
                let src = value_to_string(&self.eval_expr(source, env.clone())?, *span)?;
                let dst = value_to_string(&self.eval_expr(dest, env)?, *span)?;
                std::fs::rename(&src, &dst).map_err(|e| io_err(*span, e))?;
            }
            IoStmt::Rename { source, dest, span } => {
                let src = value_to_string(&self.eval_expr(source, env.clone())?, *span)?;
                let dst = value_to_string(&self.eval_expr(dest, env)?, *span)?;
                std::fs::rename(&src, &dst).map_err(|e| io_err(*span, e))?;
            }
            IoStmt::Delete { target, span } => {
                let path = value_to_string(&self.eval_expr(target, env)?, *span)?;
                std::fs::remove_file(&path).map_err(|e| io_err(*span, e))?;
            }
        }
        Ok(())
    }

    fn eval_block_flow(&mut self, block: &Block, env: Rc<Environment>) -> RuntimeResult<Flow> {
        for stmt in &block.statements {
            match self.eval_stmt(stmt, env.clone())? {
                Flow::None => {}
                other => return Ok(other),
            }
        }
        Ok(Flow::None)
    }

    fn assign(
        &mut self,
        target: &AssignTarget,
        op: AssignOp,
        value: Value,
        env: Rc<Environment>,
        span: Span,
    ) -> RuntimeResult<()> {
        let new_val = match op {
            AssignOp::Assign => value,
            AssignOp::AddAssign => {
                let cur = self.resolve_target_read(target, env.clone(), span)?;
                binary_op(
                    BinaryOp::Add,
                    cur,
                    value,
                    span,
                )?
            }
            AssignOp::SubAssign => {
                let cur = self.resolve_target_read(target, env.clone(), span)?;
                binary_op(BinaryOp::Sub, cur, value, span)?
            }
            AssignOp::MulAssign => {
                let cur = self.resolve_target_read(target, env.clone(), span)?;
                binary_op(BinaryOp::Mul, cur, value, span)?
            }
            AssignOp::DivAssign => {
                let cur = self.resolve_target_read(target, env.clone(), span)?;
                binary_op(BinaryOp::Div, cur, value, span)?
            }
            AssignOp::ModAssign => {
                let cur = self.resolve_target_read(target, env.clone(), span)?;
                binary_op(BinaryOp::Mod, cur, value, span)?
            }
        };

        match target {
            AssignTarget::Name(name, _) => {
                if env.set(name, new_val.clone()) {
                    return Ok(());
                }
                env.define(name.clone(), new_val);
            }
            AssignTarget::Member { object, name, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                match obj {
                    Value::Dict(d) => {
                        d.borrow_mut().insert(name.clone(), new_val);
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeError,
                            span,
                            "member assignment requires Dict",
                        ));
                    }
                }
            }
            AssignTarget::Index { object, index, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env.clone())?;
                match obj {
                    Value::List(l) => {
                        let i = index_as_usize(&idx, span)?;
                        let mut list = l.borrow_mut();
                        if i >= list.len() {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::IndexOutOfBounds,
                                span,
                                format!("index {i} out of bounds"),
                            ));
                        }
                        list[i] = new_val;
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeError,
                            span,
                            "index assignment requires List",
                        ));
                    }
                }
            }
            AssignTarget::Tuple(names, s) => {
                let Value::List(items) = new_val else {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeError,
                        *s,
                        "tuple assignment requires List value",
                    ));
                };
                let items = items.borrow();
                if items.len() != names.len() {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeError,
                        *s,
                        "tuple assignment length mismatch",
                    ));
                }
                for (name, val) in names.iter().zip(items.iter()) {
                    if env.set(name, val.clone()) {
                        continue;
                    }
                    env.define(name.clone(), val.clone());
                }
            }
        }
        Ok(())
    }

    fn resolve_target_read(
        &mut self,
        target: &AssignTarget,
        env: Rc<Environment>,
        span: Span,
    ) -> RuntimeResult<Value> {
        match target {
            AssignTarget::Name(name, _) => env.get(name).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndefinedVariable,
                    span,
                    format!("undefined variable '{name}'"),
                )
            }),
            AssignTarget::Member { object, name, .. } => {
                let obj = self.eval_expr(object, env)?;
                member_get(&obj, name, span)
            }
            AssignTarget::Index { object, index, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env.clone())?;
                index_get(&obj, &idx, span)
            }
            AssignTarget::Tuple(_, s) => Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation,
                *s,
                "invalid compound assignment to tuple",
            )),
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: Rc<Environment>) -> RuntimeResult<Value> {
        match expr {
            Expr::Int { value, .. } => Ok(Value::Int(*value)),
            Expr::Float { value, .. } => Ok(Value::Float(*value)),
            Expr::String { value, .. } => Ok(Value::String(value.clone())),
            Expr::Char { value, .. } => Ok(Value::Char(*value)),
            Expr::Bool { value, .. } => Ok(Value::Bool(*value)),
            Expr::Null { .. } => Ok(Value::Null),
            Expr::Ident { name, span } => env.get(name).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndefinedVariable,
                    *span,
                    format!("undefined variable '{name}'"),
                )
            }),
            Expr::SelfExpr { .. } => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                expr.span(),
                "self not yet supported in interpreter",
            )),
            Expr::Binary { op, left, right, span } => {
                let l = self.eval_expr(left, env.clone())?;
                let r = self.eval_expr(right, env)?;
                binary_op(*op, l, r, *span)
            }
            Expr::Unary { op, expr, span } => {
                let v = self.eval_expr(expr, env)?;
                unary_op(*op, v, *span)
            }
            Expr::Call { callee, args, span } => {
                let func = self.eval_expr(callee, env.clone())?;
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(&a.value, env.clone())?);
                }
                self.call(func, arg_vals, *span, env)
            }
            Expr::Member { object, name, span } => {
                let obj = self.eval_expr(object, env)?;
                member_get(&obj, name, *span)
            }
            Expr::Index { object, index, span } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env)?;
                index_get(&obj, &idx, *span)
            }
            Expr::With { parts, .. } => {
                let mut out = String::new();
                for p in parts {
                    out.push_str(&self.eval_expr(p, env.clone())?.to_string());
                }
                Ok(Value::String(out))
            }
            Expr::Input { input_type, prompt, span } => self.eval_input(*input_type, prompt, *span),
            Expr::Read { kind, path, span } => self.eval_read(*kind, path, *span),
            Expr::Http { .. } => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                expr.span(),
                "HTTP expressions require network runtime (not yet available)",
            )),
            Expr::List { elements, .. } => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.eval_expr(e, env.clone())?);
                }
                Ok(Value::List(Rc::new(RefCell::new(vals))))
            }
            Expr::Dict { entries, .. } => {
                let mut map = HashMap::new();
                for (k, v) in entries {
                    let key = value_to_string(&self.eval_expr(k, env.clone())?, expr.span())?;
                    map.insert(key, self.eval_expr(v, env.clone())?);
                }
                Ok(Value::Dict(Rc::new(RefCell::new(map))))
            }
            Expr::Tuple { elements, .. } => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.eval_expr(e, env.clone())?);
                }
                Ok(Value::List(Rc::new(RefCell::new(vals))))
            }
            Expr::Object { args, fields, .. } => {
                let mut map = HashMap::new();
                for a in args {
                    if let Some(name) = &a.name {
                        map.insert(
                            name.clone(),
                            self.eval_expr(&a.value, env.clone())?,
                        );
                    }
                }
                if let Some(f) = fields {
                    for stmt in &f.statements {
                        if let Stmt::Assign {
                            target: AssignTarget::Name(name, _),
                            op: AssignOp::Assign,
                            value,
                            ..
                        } = stmt
                        {
                            map.insert(name.clone(), self.eval_expr(value, env.clone())?);
                        }
                    }
                }
                Ok(Value::Dict(Rc::new(RefCell::new(map))))
            }
            Expr::Lambda { .. } => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                expr.span(),
                "lambda not yet supported in interpreter",
            )),
            Expr::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                if self.eval_expr(condition, env.clone())?.is_truthy() {
                    self.eval_expr(then_expr, env)
                } else {
                    self.eval_expr(else_expr, env)
                }
            }
            Expr::Is { .. } => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                expr.span(),
                "is expression not yet supported",
            )),
            Expr::NullCoalesce { left, right, .. } => {
                let l = self.eval_expr(left, env.clone())?;
                if matches!(l, Value::Null) {
                    self.eval_expr(right, env)
                } else {
                    Ok(l)
                }
            }
            Expr::Super { .. } | Expr::This { .. } => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                expr.span(),
                "OOP expressions not yet supported",
            )),
        }
    }

    fn eval_input(
        &mut self,
        input_type: Option<InputTypeKeyword>,
        prompt: &str,
        span: Span,
    ) -> RuntimeResult<Value> {
        print!("{prompt}");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| {
            RuntimeError::new(RuntimeErrorKind::IoError, span, format!("input failed: {e}"))
        })?;
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        match input_type {
            None | Some(InputTypeKeyword::Text) | Some(InputTypeKeyword::Password) => {
                Ok(Value::String(line))
            }
            Some(InputTypeKeyword::Number) => line.trim().parse::<i64>().map(Value::Int).map_err(
                |_| RuntimeError::new(RuntimeErrorKind::TypeError, span, "invalid number input"),
            ),
            Some(InputTypeKeyword::Decimal) => line
                .trim()
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| {
                    RuntimeError::new(RuntimeErrorKind::TypeError, span, "invalid decimal input")
                }),
            Some(InputTypeKeyword::Boolean) => {
                let v = match line.trim().to_lowercase().as_str() {
                    "true" | "yes" | "1" => true,
                    "false" | "no" | "0" => false,
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeError,
                            span,
                            "invalid boolean input",
                        ));
                    }
                };
                Ok(Value::Bool(v))
            }
            Some(_) => Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                span,
                "picker input types require GUI runtime",
            )),
        }
    }

    fn eval_read(&mut self, kind: ReadKind, path: &str, span: Span) -> RuntimeResult<Value> {
        match kind {
            ReadKind::Text => {
                let data = std::fs::read_to_string(path).map_err(|e| io_err(span, e))?;
                Ok(Value::String(data))
            }
            ReadKind::Bytes => {
                let data = std::fs::read(path).map_err(|e| io_err(span, e))?;
                Ok(Value::String(String::from_utf8_lossy(&data).into_owned()))
            }
            ReadKind::Lines => {
                let data = std::fs::read_to_string(path).map_err(|e| io_err(span, e))?;
                let lines: Vec<Value> = data
                    .lines()
                    .map(|l| Value::String(l.to_string()))
                    .collect();
                Ok(Value::List(Rc::new(RefCell::new(lines))))
            }
        }
    }

    fn call(
        &mut self,
        func: Value,
        args: Vec<Value>,
        span: Span,
        caller_env: Rc<Environment>,
    ) -> RuntimeResult<Value> {
        match func {
            Value::NativeFunction(f) => f(&args),
            Value::Function(f) => {
                let decl = f.decl.clone();
                if args.len() < decl.params.len() {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation,
                        span,
                        format!(
                            "function '{}' expects {} arguments, got {}",
                            f.name,
                            decl.params.len(),
                            args.len()
                        ),
                    ));
                }
                let call_env = Rc::new(Environment::child(caller_env));
                for (param, arg) in decl.params.iter().zip(args.iter()) {
                    call_env.define(param.name.clone(), arg.clone());
                }
                match self.eval_block_flow(&decl.body, call_env)? {
                    Flow::Return(vals, _) => Ok(vals.into_iter().next().unwrap_or(Value::Null)),
                    Flow::None => Ok(Value::Null),
                    Flow::Break(s) | Flow::Continue(s) => Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation,
                        s,
                        "break/continue outside loop",
                    )),
                }
            }
            other => Err(RuntimeError::new(
                RuntimeErrorKind::TypeError,
                span,
                format!("{} is not callable", other.type_name()),
            )),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run(program: &Program) -> RuntimeResult<RunResult> {
    Interpreter::new().run_program(program)
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
            | Expr::Tuple { span, .. }
            | Expr::Object { span, .. }
            | Expr::Lambda { span, .. }
            | Expr::If { span, .. }
            | Expr::Is { span, .. }
            | Expr::NullCoalesce { span, .. } => *span,
        }
    }
}

fn io_err(span: Span, e: std::io::Error) -> RuntimeError {
    RuntimeError::new(
        RuntimeErrorKind::IoError,
        span,
        format!("I/O error: {e}"),
    )
}

fn value_to_string(v: &Value, _span: Span) -> RuntimeResult<String> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Ok(other.to_string()),
    }
}

fn index_as_usize(v: &Value, span: Span) -> RuntimeResult<usize> {
    match v {
        Value::Int(n) if *n >= 0 => Ok(*n as usize),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "index must be non-negative Int",
        )),
    }
}

fn member_get(obj: &Value, name: &str, span: Span) -> RuntimeResult<Value> {
    match obj {
        Value::Dict(d) => d.borrow().get(name).cloned().ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::UndefinedVariable,
                span,
                format!("no member '{name}'"),
            )
        }),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "member access requires Dict",
        )),
    }
}

fn index_get(obj: &Value, idx: &Value, span: Span) -> RuntimeResult<Value> {
    match (obj, idx) {
        (Value::List(l), Value::Int(i)) => {
            let i = *i as usize;
            l.borrow().get(i).cloned().ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::IndexOutOfBounds,
                    span,
                    format!("index {i} out of bounds"),
                )
            })
        }
        (Value::String(s), Value::Int(i)) => {
            let i = *i as usize;
            s.chars()
                .nth(i)
                .map(|c| Value::Char(c))
                .ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::IndexOutOfBounds,
                        span,
                        format!("index {i} out of bounds"),
                    )
                })
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "invalid index operation",
        )),
    }
}

fn value_to_iterable(v: &Value, span: Span) -> RuntimeResult<Vec<Value>> {
    match v {
        Value::List(l) => Ok(l.borrow().clone()),
        Value::String(s) => Ok(s.chars().map(|c| Value::Char(c)).collect()),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "for loop requires List or String",
        )),
    }
}

fn item_as_pair(v: &Value, span: Span) -> RuntimeResult<(Value, Value)> {
    match v {
        Value::List(l) => {
            let l = l.borrow();
            if l.len() == 2 {
                Ok((l[0].clone(), l[1].clone()))
            } else {
                Err(RuntimeError::new(
                    RuntimeErrorKind::TypeError,
                    span,
                    "for key, value requires pairs of length 2",
                ))
            }
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "for key, value requires List pairs",
        )),
    }
}

fn unary_op(op: UnaryOp, v: Value, span: Span) -> RuntimeResult<Value> {
    match op {
        UnaryOp::Not => Ok(Value::Bool(!v.is_truthy())),
        UnaryOp::Neg => match v {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            _ => Err(type_err(span, "unary -")),
        },
        UnaryOp::Pos => Ok(v),
        UnaryOp::BitNot => match v {
            Value::Int(n) => Ok(Value::Int(!n)),
            _ => Err(type_err(span, "bitwise not")),
        },
    }
}

fn binary_op(op: BinaryOp, l: Value, r: Value, span: Span) -> RuntimeResult<Value> {
    use BinaryOp::*;
    match op {
        Add => match (&l, &r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
            _ => Err(type_err(span, "+")),
        },
        Sub => num_binop(l, r, span, |a, b| a - b, |a, b| a - b),
        Mul => num_binop(l, r, span, |a, b| a * b, |a, b| a * b),
        Div => num_binop(l, r, span, |a, b| a / b, |a, b| a / b),
        Mod => match (&l, &r) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::DivisionByZero,
                        span,
                        "division by zero",
                    ));
                }
                Ok(Value::Int(a % b))
            }
            _ => Err(type_err(span, "%")),
        },
        IntDiv => match (&l, &r) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::DivisionByZero,
                        span,
                        "division by zero",
                    ));
                }
                Ok(Value::Int(a / b))
            }
            _ => Err(type_err(span, "//")),
        },
        Pow => match (&l, &r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.pow(*b as u32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powi(*b as i32))),
            _ => Err(type_err(span, "**")),
        },
        Eq => Ok(Value::Bool(l == r)),
        NotEq => Ok(Value::Bool(l != r)),
        Lt | Gt | LtEq | GtEq => cmp(l, r, span, op),
        And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
        Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
        BitAnd | BitOr | BitXor | Shl | Shr => match (&l, &r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(match op {
                BitAnd => a & b,
                BitOr => a | b,
                BitXor => a ^ b,
                Shl => a << b,
                Shr => a >> b,
                _ => unreachable!(),
            })),
            _ => Err(type_err(span, "bitwise")),
        },
    }
}

fn num_binop(
    l: Value,
    r: Value,
    span: Span,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> RuntimeResult<Value> {
    match (&l, &r) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(int_op(*a, *b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(float_op(*a, *b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(float_op(*a as f64, *b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(float_op(*a, *b as f64))),
        _ => Err(type_err(span, "numeric op")),
    }
}

fn cmp(l: Value, r: Value, span: Span, op: BinaryOp) -> RuntimeResult<Value> {
    use BinaryOp::*;
    let result = match (&l, &r) {
        (Value::Int(a), Value::Int(b)) => match op {
            Lt => a < b,
            Gt => a > b,
            LtEq => a <= b,
            GtEq => a >= b,
            _ => false,
        },
        (Value::Float(a), Value::Float(b)) => match op {
            Lt => a < b,
            Gt => a > b,
            LtEq => a <= b,
            GtEq => a >= b,
            _ => false,
        },
        (Value::String(a), Value::String(b)) => match op {
            Lt => a < b,
            Gt => a > b,
            LtEq => a <= b,
            GtEq => a >= b,
            _ => false,
        },
        _ => {
            return Err(type_err(span, "comparison"));
        }
    };
    Ok(Value::Bool(result))
}

fn type_err(span: Span, op: &str) -> RuntimeError {
    RuntimeError::new(
        RuntimeErrorKind::TypeError,
        span,
        format!("invalid operands for {op}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use langp_parser::parse;

    #[test]
    fn run_hello() {
        let source = r#"function greet(name),
    print "Hello " with name with "!".
.
greet("World")."#;
        let program = parse(source).unwrap();
        run(&program).unwrap();
    }
}
