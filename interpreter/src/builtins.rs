use langp_runtime::{RuntimeError, RuntimeErrorKind, Value};
use std::rc::Rc;

pub fn register_builtins(env: &super::env::Environment) {
    env.define(
        "len",
        Value::NativeFunction(Rc::new(|args| {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    Default::default(),
                    "len() expects 1 argument",
                ));
            }
            let n = match &args[0] {
                Value::String(s) => s.chars().count() as i64,
                Value::List(l) => l.borrow().len() as i64,
                Value::Dict(d) => d.borrow().len() as i64,
                Value::Set(s) => s.borrow().len() as i64,
                Value::Tuple(t) => t.len() as i64,
                other => {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::TypeError,
                        Default::default(),
                        format!("len() not supported for {}", other.type_name()),
                    ));
                }
            };
            Ok(Value::Int(n))
        })),
    );

    env.define(
        "to_string",
        Value::NativeFunction(Rc::new(|args| {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    Default::default(),
                    "to_string() expects 1 argument",
                ));
            }
            Ok(Value::String(args[0].to_string()))
        })),
    );

    env.define(
        "assert",
        Value::NativeFunction(Rc::new(|args| {
            if args.is_empty() {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    Default::default(),
                    "assert() expects at least 1 argument",
                ));
            }
            if !args[0].is_truthy() {
                let msg = args
                    .get(1)
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "assertion failed".to_string());
                return Err(RuntimeError::new(
                    RuntimeErrorKind::UserError,
                    Default::default(),
                    msg,
                ));
            }
            Ok(Value::Null)
        })),
    );
}
