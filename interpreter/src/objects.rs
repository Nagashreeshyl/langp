//! Unified member access and method dispatch for objects and modules.

use crate::collections;
use crate::oop::TypeRegistry;
use langp_lexer::Span;
use langp_runtime::{RuntimeError, RuntimeErrorKind, RuntimeResult, Value};

pub fn member_get(
    obj: &Value,
    name: &str,
    span: Span,
    types: &TypeRegistry,
) -> RuntimeResult<Value> {
    match obj {
        Value::Instance(inst) => inst
            .fields
            .borrow()
            .get(name)
            .cloned()
            .ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndefinedVariable,
                    span,
                    format!("instance has no field '{name}'"),
                )
            }),
        Value::LangType(type_name) => {
            let def = types.get(type_name).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::TypeError,
                    span,
                    format!("unknown type '{type_name}'"),
                )
            })?;
            if let Some(v) = def.static_fields.get(name) {
                return Ok(v.clone());
            }
            if def.methods.contains_key(name) {
                return Ok(Value::String(format!("{type_name}.{name}")));
            }
            Err(RuntimeError::new(
                RuntimeErrorKind::UndefinedVariable,
                span,
                format!("type '{type_name}' has no static member '{name}'"),
            ))
        }
        Value::Module(m) => m.exports.borrow().get(name).cloned().ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::UndefinedVariable,
                span,
                format!("module '{}' has no export '{name}'", m.name),
            )
        }),
        _ => collections::member_get(obj, name, span),
    }
}

pub fn member_set(obj: &Value, name: &str, value: Value, span: Span) -> RuntimeResult<()> {
    match obj {
        Value::Instance(inst) => {
            inst.fields.borrow_mut().insert(name.to_string(), value);
            Ok(())
        }
        Value::Dict(d) => {
            d.borrow_mut().insert(name.to_string(), value);
            Ok(())
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("cannot assign member on {}", obj.type_name()),
        )),
    }
}

pub fn dispatch_method(
    receiver: &Value,
    method: &str,
    args: &[Value],
    span: Span,
    types: &TypeRegistry,
) -> RuntimeResult<Value> {
    match receiver {
        Value::Instance(inst) => {
            let def = types.get(&inst.type_name).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::TypeError,
                    span,
                    format!("unknown type '{}'", inst.type_name),
                )
            })?;
            if types.find_method(&def, method).is_some() {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    span,
                    format!(
                        "call instance method '{method}' via interpreter (internal error)"
                    ),
                ));
            }
            Err(RuntimeError::new(
                RuntimeErrorKind::TypeError,
                span,
                format!("{} has no method '{method}'", inst.type_name),
            ))
        }
        Value::Module(m) => {
            let func = m.exports.borrow().get(method).cloned().ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::TypeError,
                    span,
                    format!("module '{}' has no method '{method}'", m.name),
                )
            })?;
            call_native(func, args, span)
        }
        _ => collections::dispatch_method(receiver, method, args, span),
    }
}

pub fn call_native(func: Value, args: &[Value], span: Span) -> RuntimeResult<Value> {
    match func {
        Value::NativeFunction(f) => f(args),
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "expected native function",
        )),
    }
}
