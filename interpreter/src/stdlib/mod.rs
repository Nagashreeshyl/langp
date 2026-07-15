//! Standard library modules built into the interpreter.

mod filesystem;

use langp_runtime::{ModuleData, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn builtin_module(name: &str) -> Option<Rc<ModuleData>> {
    match name {
        "filesystem" => Some(filesystem::module()),
        "math" => Some(math_module()),
        "json" => Some(json_module()),
        "navigator" | "ai" | "database" | "network" => Some(stub_module(name)),
        _ => None,
    }
}

fn stub_module(name: &str) -> Rc<ModuleData> {
    let mut map = HashMap::new();
    map.insert(
        "version".to_string(),
        Value::String("0.0.0-stub".to_string()),
    );
    Rc::new(ModuleData {
        name: name.to_string(),
        exports: RefCell::new(map),
    })
}

fn math_module() -> Rc<ModuleData> {
    let mut map = HashMap::new();
    map.insert(
        "abs".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            let n = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 0,
            };
            Ok(Value::Int(n.abs()))
        })),
    );
    map.insert(
        "min".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            let a = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 0,
            };
            let b = match args.get(1) {
                Some(Value::Int(n)) => *n,
                _ => 0,
            };
            Ok(Value::Int(a.min(b)))
        })),
    );
    map.insert(
        "max".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            let a = match args.first() {
                Some(Value::Int(n)) => *n,
                _ => 0,
            };
            let b = match args.get(1) {
                Some(Value::Int(n)) => *n,
                _ => 0,
            };
            Ok(Value::Int(a.max(b)))
        })),
    );
    Rc::new(ModuleData {
        name: "math".to_string(),
        exports: RefCell::new(map),
    })
}

fn json_module() -> Rc<ModuleData> {
    let mut map = HashMap::new();
    map.insert(
        "stringify".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::String("null".to_string()));
            }
            Ok(Value::String(args[0].to_string()))
        })),
    );
    map.insert(
        "parse".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Null);
            }
            Ok(Value::String(args[0].to_string()))
        })),
    );
    Rc::new(ModuleData {
        name: "json".to_string(),
        exports: RefCell::new(map),
    })
}
