//! Collection method dispatch for List, Dict, Set, and Tuple.

use langp_lexer::Span;
use langp_runtime::{
    set_contains, set_insert, RuntimeError, RuntimeErrorKind, RuntimeResult, Value,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn dispatch_method(
    receiver: &Value,
    method: &str,
    args: &[Value],
    span: Span,
) -> RuntimeResult<Value> {
    match receiver {
        Value::List(list) => list_method(list, method, args, span),
        Value::Dict(dict) => dict_method(dict, method, args, span),
        Value::Set(set) => set_method(set, method, args, span),
        Value::Tuple(tuple) => tuple_method(tuple, method, args, span),
        other => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("{} has no method '{method}'", other.type_name()),
        )),
    }
}

fn expect_args(method: &str, args: &[Value], count: usize, span: Span) -> RuntimeResult<()> {
    if args.len() != count {
        return Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation,
            span,
            format!("{method}() expects {count} argument(s), got {}", args.len()),
        ));
    }
    Ok(())
}

fn list_method(
    list: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    args: &[Value],
    span: Span,
) -> RuntimeResult<Value> {
    match method {
        "append" => {
            expect_args("append", args, 1, span)?;
            list.borrow_mut().push(args[0].clone());
            Ok(Value::Null)
        }
        "insert" => {
            expect_args("insert", args, 2, span)?;
            let idx = index_as_usize(&args[0], span)?;
            let mut items = list.borrow_mut();
            if idx > items.len() {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::IndexOutOfBounds,
                    span,
                    format!("insert index {idx} out of bounds"),
                ));
            }
            items.insert(idx, args[1].clone());
            Ok(Value::Null)
        }
        "remove" => {
            expect_args("remove", args, 1, span)?;
            let mut items = list.borrow_mut();
            let pos = items
                .iter()
                .position(|v| v == &args[0])
                .ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation,
                        span,
                        "list.remove: value not found",
                    )
                })?;
            items.remove(pos);
            Ok(Value::Null)
        }
        "pop" => {
            let mut items = list.borrow_mut();
            if items.is_empty() {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    span,
                    "pop from empty list",
                ));
            }
            if args.is_empty() {
                Ok(items.pop().unwrap())
            } else if args.len() == 1 {
                let idx = index_as_usize(&args[0], span)?;
                if idx >= items.len() {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::IndexOutOfBounds,
                        span,
                        format!("pop index {idx} out of bounds"),
                    ));
                }
                Ok(items.remove(idx))
            } else {
                Err(RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    span,
                    "pop() expects 0 or 1 arguments",
                ))
            }
        }
        "clear" => {
            expect_args("clear", args, 0, span)?;
            list.borrow_mut().clear();
            Ok(Value::Null)
        }
        "sort" => {
            expect_args("sort", args, 0, span)?;
            list.borrow_mut().sort_by(|a, b| compare_values(a, b));
            Ok(Value::Null)
        }
        "reverse" => {
            expect_args("reverse", args, 0, span)?;
            list.borrow_mut().reverse();
            Ok(Value::Null)
        }
        "contains" => {
            expect_args("contains", args, 1, span)?;
            Ok(Value::Bool(list.borrow().iter().any(|v| v == &args[0])))
        }
        "length" => {
            expect_args("length", args, 0, span)?;
            Ok(Value::Int(list.borrow().len() as i64))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("List has no method '{method}'"),
        )),
    }
}

fn dict_method(
    dict: &Rc<RefCell<HashMap<String, Value>>>,
    method: &str,
    args: &[Value],
    span: Span,
) -> RuntimeResult<Value> {
    match method {
        "keys" => {
            expect_args("keys", args, 0, span)?;
            let keys: Vec<Value> = dict
                .borrow()
                .keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(keys))))
        }
        "values" => {
            expect_args("values", args, 0, span)?;
            let vals: Vec<Value> = dict.borrow().values().cloned().collect();
            Ok(Value::List(Rc::new(RefCell::new(vals))))
        }
        "items" => {
            expect_args("items", args, 0, span)?;
            let pairs: Vec<Value> = dict
                .borrow()
                .iter()
                .map(|(k, v)| {
                    Value::List(Rc::new(RefCell::new(vec![
                        Value::String(k.clone()),
                        v.clone(),
                    ])))
                })
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(pairs))))
        }
        "remove" => {
            expect_args("remove", args, 1, span)?;
            let key = key_from_value(&args[0], span)?;
            dict.borrow_mut().remove(&key).ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::InvalidOperation,
                    span,
                    format!("dict.remove: key '{key}' not found"),
                )
            })?;
            Ok(Value::Null)
        }
        "contains" => {
            expect_args("contains", args, 1, span)?;
            let key = key_from_value(&args[0], span)?;
            Ok(Value::Bool(dict.borrow().contains_key(&key)))
        }
        "clear" => {
            expect_args("clear", args, 0, span)?;
            dict.borrow_mut().clear();
            Ok(Value::Null)
        }
        "length" => {
            expect_args("length", args, 0, span)?;
            Ok(Value::Int(dict.borrow().len() as i64))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("Dict has no method '{method}'"),
        )),
    }
}

fn set_method(
    set: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    args: &[Value],
    span: Span,
) -> RuntimeResult<Value> {
    match method {
        "add" => {
            expect_args("add", args, 1, span)?;
            set_insert(set.borrow_mut().as_mut(), args[0].clone());
            Ok(Value::Null)
        }
        "remove" => {
            expect_args("remove", args, 1, span)?;
            let mut items = set.borrow_mut();
            let pos = items
                .iter()
                .position(|v| v == &args[0])
                .ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::InvalidOperation,
                        span,
                        "set.remove: value not found",
                    )
                })?;
            items.remove(pos);
            Ok(Value::Null)
        }
        "contains" => {
            expect_args("contains", args, 1, span)?;
            Ok(Value::Bool(set_contains(set.borrow().as_slice(), &args[0])))
        }
        "clear" => {
            expect_args("clear", args, 0, span)?;
            set.borrow_mut().clear();
            Ok(Value::Null)
        }
        "union" => {
            expect_args("union", args, 1, span)?;
            let other = expect_set(&args[0], span)?;
            Ok(set_binary_op(set, &other, SetOp::Union))
        }
        "intersection" => {
            expect_args("intersection", args, 1, span)?;
            let other = expect_set(&args[0], span)?;
            Ok(set_binary_op(set, &other, SetOp::Intersection))
        }
        "difference" => {
            expect_args("difference", args, 1, span)?;
            let other = expect_set(&args[0], span)?;
            Ok(set_binary_op(set, &other, SetOp::Difference))
        }
        "length" => {
            expect_args("length", args, 0, span)?;
            Ok(Value::Int(set.borrow().len() as i64))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("Set has no method '{method}'"),
        )),
    }
}

enum SetOp {
    Union,
    Intersection,
    Difference,
}

fn set_binary_op(a: &Rc<RefCell<Vec<Value>>>, b: &Rc<RefCell<Vec<Value>>>, op: SetOp) -> Value {
    let a = a.borrow();
    let b = b.borrow();
    let mut out = Vec::new();
    match op {
        SetOp::Union => {
            for v in a.iter().chain(b.iter()) {
                set_insert(&mut out, v.clone());
            }
        }
        SetOp::Intersection => {
            for v in a.iter() {
                if set_contains(b.as_slice(), v) {
                    set_insert(&mut out, v.clone());
                }
            }
        }
        SetOp::Difference => {
            for v in a.iter() {
                if !set_contains(b.as_slice(), v) {
                    set_insert(&mut out, v.clone());
                }
            }
        }
    }
    Value::Set(Rc::new(RefCell::new(out)))
}

fn tuple_method(
    tuple: &Rc<Vec<Value>>,
    method: &str,
    args: &[Value],
    span: Span,
) -> RuntimeResult<Value> {
    match method {
        "length" => {
            expect_args("length", args, 0, span)?;
            Ok(Value::Int(tuple.len() as i64))
        }
        "contains" => {
            expect_args("contains", args, 1, span)?;
            Ok(Value::Bool(tuple.iter().any(|v| v == &args[0])))
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("Tuple has no method '{method}'"),
        )),
    }
}

fn expect_set(v: &Value, span: Span) -> RuntimeResult<Rc<RefCell<Vec<Value>>>> {
    match v {
        Value::Set(s) => Ok(Rc::clone(s)),
        other => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("expected Set, got {}", other.type_name()),
        )),
    }
}

fn key_from_value(v: &Value, _span: Span) -> RuntimeResult<String> {
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

fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::Int(x), Value::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::Float(x), Value::Int(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        _ => a.to_string().cmp(&b.to_string()),
    }
}

pub fn index_get(obj: &Value, idx: &Value, span: Span) -> RuntimeResult<Value> {
    match (obj, idx) {
        (Value::List(l), idx) => {
            let i = index_as_usize(idx, span)?;
            l.borrow().get(i).cloned().ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::IndexOutOfBounds,
                    span,
                    format!("index {i} out of bounds"),
                )
            })
        }
        (Value::Tuple(t), idx) => {
            let i = index_as_usize(idx, span)?;
            t.get(i).cloned().ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::IndexOutOfBounds,
                    span,
                    format!("index {i} out of bounds"),
                )
            })
        }
        (Value::String(s), idx) => {
            let i = index_as_usize(idx, span)?;
            s.chars()
                .nth(i)
                .map(Value::Char)
                .ok_or_else(|| {
                    RuntimeError::new(
                        RuntimeErrorKind::IndexOutOfBounds,
                        span,
                        format!("index {i} out of bounds"),
                    )
                })
        }
        (Value::Dict(d), key) => {
            let k = key_from_value(key, span)?;
            d.borrow().get(&k).cloned().ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndefinedVariable,
                    span,
                    format!("no key '{k}'"),
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

pub fn index_set(obj: Value, idx: Value, new_val: Value, span: Span) -> RuntimeResult<()> {
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
            Ok(())
        }
        Value::Dict(d) => {
            let k = key_from_value(&idx, span)?;
            d.borrow_mut().insert(k, new_val);
            Ok(())
        }
        Value::Tuple(_) => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "Tuple is immutable",
        )),
        other => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            format!("index assignment not supported on {}", other.type_name()),
        )),
    }
}

pub fn member_get(obj: &Value, name: &str, span: Span) -> RuntimeResult<Value> {
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
            format!("member access requires Dict, not {}", obj.type_name()),
        )),
    }
}

pub fn value_to_iterable(v: &Value, span: Span) -> RuntimeResult<Vec<Value>> {
    match v {
        Value::List(l) => Ok(l.borrow().clone()),
        Value::Set(s) => Ok(s.borrow().clone()),
        Value::Tuple(t) => Ok(t.as_ref().clone()),
        Value::String(s) => Ok(s.chars().map(Value::Char).collect()),
        Value::Dict(d) => {
            let pairs: Vec<Value> = d
                .borrow()
                .iter()
                .map(|(k, v)| {
                    Value::List(Rc::new(RefCell::new(vec![
                        Value::String(k.clone()),
                        v.clone(),
                    ])))
                })
                .collect();
            Ok(pairs)
        }
        _ => Err(RuntimeError::new(
            RuntimeErrorKind::TypeError,
            span,
            "for loop requires List, Set, Tuple, Dict, or String",
        )),
    }
}

pub fn item_as_pair(v: &Value, span: Span) -> RuntimeResult<(Value, Value)> {
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
