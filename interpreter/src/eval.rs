use crate::builtins::register_builtins;
use crate::collections;
use crate::env::Environment;
use crate::modules::ModuleLoader;
use crate::objects;
use crate::oop::{self, TypeRegistry};
use langp_ast::*;
use langp_lexer::{InputTypeKeyword, Span};
use langp_runtime::{
    set_insert, InstanceData, RuntimeError, RuntimeErrorKind, RuntimeResult, UserFunction, Value,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
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
    types: TypeRegistry,
    modules: ModuleLoader,
    current_self: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(Environment::new());
        register_builtins(&globals);
        Self {
            globals,
            functions: HashMap::new(),
            types: TypeRegistry::new(),
            modules: ModuleLoader::new(None),
            current_self: None,
        }
    }

    pub fn with_project_root(root: PathBuf) -> Self {
        let mut i = Self::new();
        i.modules = ModuleLoader::new(Some(root));
        i
    }

    pub fn run_program(&mut self, program: &Program) -> RuntimeResult<RunResult> {
        self.types.register_from_program(&program.items)?;

        for item in &program.items {
            if let ModuleItem::Use(u) = item {
                let module = self.modules.load(&u.path)?;
                if let Some(name) = u.path.first() {
                    self.globals.define(name.clone(), module);
                }
            }
        }

        for name in self.types.names().cloned().collect::<Vec<_>>() {
            self.globals
                .define(name.clone(), Value::LangType(name));
        }

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
            Stmt::Write {
                kind,
                value,
                destination,
                span,
            } => {
                let val = self.eval_expr(value, env.clone())?;
                let dest = self.eval_expr(destination, env.clone())?;
                let path = value_to_string(&dest, *span)?;
                match kind {
                    WriteKind::Write => {
                        std::fs::write(&path, val.to_string()).map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::IoError,
                                *span,
                                format!("write failed: {e}"),
                            )
                        })?;
                    }
                    WriteKind::WriteBytes => {
                        std::fs::write(&path, val.to_string().into_bytes()).map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::IoError,
                                *span,
                                format!("write_bytes failed: {e}"),
                            )
                        })?;
                    }
                    WriteKind::Append => {
                        use std::io::Write as _;
                        let mut file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path)
                            .map_err(|e| io_err(*span, e))?;
                        file.write_all(val.to_string().as_bytes())
                            .map_err(|e| io_err(*span, e))?;
                    }
                }
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
        let items = collections::value_to_iterable(&iterable, stmt.span)?;
        for item in items {
            let loop_env = Rc::new(Environment::child(env.clone()));
            match &stmt.binding {
                ForBinding::Single(name, _) => {
                    loop_env.define(name.clone(), item);
                }
                ForBinding::KeyValue(k, v, span) => {
                    let (key, val) = collections::item_as_pair(&item, *span)?;
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
            AssignTarget::Name { name, .. } => {
                if env.set(name, new_val.clone()) {
                    return Ok(());
                }
                env.define(name.clone(), new_val);
            }
            AssignTarget::Member { object, name, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                objects::member_set(&obj, name, new_val, span)?;
            }
            AssignTarget::Index { object, index, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env.clone())?;
                collections::index_set(obj, idx, new_val, span)?;
            }
            AssignTarget::Tuple(names, s) => {
                let items: Vec<Value> = match new_val {
                    Value::List(l) => l.borrow().clone(),
                    Value::Tuple(t) => t.as_ref().clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeError,
                            *s,
                            "tuple assignment requires List or Tuple value",
                        ));
                    }
                };
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
            AssignTarget::Name { name, .. } => env.get(name).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndefinedVariable,
                    span,
                    format!("undefined variable '{name}'"),
                )
            }),
            AssignTarget::Member { object, name, .. } => {
                let obj = self.eval_expr(object, env)?;
                objects::member_get(&obj, name, span, &self.types)
            }
            AssignTarget::Index { object, index, .. } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env.clone())?;
                collections::index_get(&obj, &idx, span)
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
            Expr::SelfExpr { .. } => {
                if let Some(self_val) = &self.current_self {
                    Ok(self_val.clone())
                } else if let Some(self_val) = env.get("self") {
                    Ok(self_val)
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation,
                        expr.span(),
                        "self used outside of method",
                    ))
                }
            }
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
                if let Expr::Member { object, name, .. } = callee.as_ref() {
                    let obj = self.eval_expr(object, env.clone())?;
                    let mut arg_vals = Vec::new();
                    for a in args {
                        arg_vals.push(self.eval_expr(&a.value, env.clone())?);
                    }
                    if let Value::Instance(inst) = &obj {
                        let def = self.types.get(&inst.type_name).ok_or_else(|| {
                            RuntimeError::new(
                                RuntimeErrorKind::TypeError,
                                *span,
                                format!("unknown type '{}'", inst.type_name),
                            )
                        })?;
                        if let Some(method) = self.types.find_method(&def, name) {
                            return self.call_instance_method(obj, method, arg_vals, *span, env);
                        }
                    }
                    return objects::dispatch_method(&obj, name, &arg_vals, *span, &self.types);
                }
                let func = self.eval_expr(callee, env.clone())?;
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(&a.value, env.clone())?);
                }
                self.call(func, arg_vals, *span, env)
            }
            Expr::Member { object, name, span } => {
                let obj = self.eval_expr(object, env)?;
                objects::member_get(&obj, name, *span, &self.types)
            }
            Expr::Index { object, index, span } => {
                let obj = self.eval_expr(object, env.clone())?;
                let idx = self.eval_expr(index, env)?;
                collections::index_get(&obj, &idx, *span)
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
                    let key = dict_key_from_expr(k, env.clone(), self, expr.span())?;
                    map.insert(key, self.eval_expr(v, env.clone())?);
                }
                Ok(Value::Dict(Rc::new(RefCell::new(map))))
            }
            Expr::Set { elements, .. } => {
                let mut vals = Vec::new();
                for e in elements {
                    let v = self.eval_expr(e, env.clone())?;
                    set_insert(&mut vals, v);
                }
                Ok(Value::Set(Rc::new(RefCell::new(vals))))
            }
            Expr::Tuple { elements, .. } => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.eval_expr(e, env.clone())?);
                }
                Ok(Value::Tuple(Rc::new(vals)))
            }
            Expr::Object { ty, args, fields, span } => {
                let type_name = oop::type_name_from_expr(ty).ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::TypeError,
                        *span,
                        "object creation requires a named type",
                    )
                })?;
                let def = self.types.get(&type_name).ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::TypeError,
                        *span,
                        format!("unknown type '{type_name}'"),
                    )
                })?;
                let mut field_values = HashMap::new();
                for fname in self.types.instance_field_names(&def) {
                    if let Some(f) = def.fields.iter().find(|f| f.name == fname) {
                        if let Some(default_expr) = &f.default {
                            field_values.insert(
                                fname.clone(),
                                self.eval_expr(default_expr, env.clone())?,
                            );
                        } else {
                            field_values.insert(fname, Value::Null);
                        }
                    }
                }
                let instance_fields: Vec<_> = def.fields.iter().filter(|f| !f.is_static).collect();
                let mut positional = 0usize;
                for a in args {
                    if let Some(name) = &a.name {
                        field_values.insert(
                            name.clone(),
                            self.eval_expr(&a.value, env.clone())?,
                        );
                    } else if positional < instance_fields.len() {
                        field_values.insert(
                            instance_fields[positional].name.clone(),
                            self.eval_expr(&a.value, env.clone())?,
                        );
                        positional += 1;
                    }
                }
                if let Some(block) = fields {
                    for stmt in &block.statements {
                        if let Stmt::Assign {
                            target: AssignTarget::Name { name, .. },
                            op: AssignOp::Assign,
                            value,
                            ..
                        } = stmt
                        {
                            field_values.insert(
                                name.clone(),
                                self.eval_expr(value, env.clone())?,
                            );
                        }
                    }
                }
                let instance = Value::Instance(Rc::new(InstanceData {
                    type_name: type_name.clone(),
                    fields: RefCell::new(field_values),
                }));
                if let Some(init) = self.types.find_method(&def, "init") {
                    let init_args: Vec<Value> = args
                        .iter()
                        .map(|a| self.eval_expr(&a.value, env.clone()))
                        .collect::<RuntimeResult<_>>()?;
                    self.call_instance_method(
                        instance.clone(),
                        init,
                        init_args,
                        *span,
                        env.clone(),
                    )?;
                }
                Ok(instance)
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

    fn call_instance_method(
        &mut self,
        receiver: Value,
        func: Rc<FunctionDecl>,
        arg_vals: Vec<Value>,
        span: Span,
        caller_env: Rc<Environment>,
    ) -> RuntimeResult<Value> {
        let prev_self = self.current_self.clone();
        self.current_self = Some(receiver.clone());
        let call_env = Rc::new(Environment::child(caller_env));
        call_env.define("self", receiver);
        for (param, arg) in func.params.iter().zip(arg_vals.iter()) {
            call_env.define(param.name.clone(), arg.clone());
        }
        for param in func.params.iter().skip(arg_vals.len()) {
            if let Some(default) = &param.default {
                call_env.define(
                    param.name.clone(),
                    self.eval_expr(default, call_env.clone())?,
                );
            } else {
                call_env.define(param.name.clone(), Value::Null);
            }
        }
        let result = match self.eval_block_flow(&func.body, call_env)? {
            Flow::Return(vals, _) => Ok(vals.into_iter().next().unwrap_or(Value::Null)),
            Flow::None => Ok(Value::Null),
            Flow::Break(s) | Flow::Continue(s) => Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation,
                s,
                "break/continue outside loop",
            )),
        };
        self.current_self = prev_self;
        result
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

fn dict_key_from_expr(
    key: &Expr,
    env: Rc<Environment>,
    interp: &mut Interpreter,
    span: Span,
) -> RuntimeResult<String> {
    match key {
        Expr::Ident { name, .. } => Ok(name.clone()),
        Expr::String { value, .. } => Ok(value.clone()),
        other => value_to_string(&interp.eval_expr(other, env)?, span),
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
..
greet("World")."#;
        let program = parse(source).unwrap();
        run(&program).unwrap();
    }
}
