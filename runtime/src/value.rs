use langp_ast::{FunctionDecl, Param};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Null,
    List(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
    Set(Rc<RefCell<Vec<Value>>>),
    Tuple(Rc<Vec<Value>>),
    /// OOP instance: `user.name`
    Instance(Rc<InstanceData>),
    /// Type constructor reference: `User()`
    LangType(String),
    /// Imported module namespace: `filesystem.exists(...)`
    Module(Rc<ModuleData>),
    Function(Rc<UserFunction>),
    NativeFunction(NativeFn),
}

#[derive(Debug, Clone)]
pub struct InstanceData {
    pub type_name: String,
    pub fields: RefCell<HashMap<String, Value>>,
}

#[derive(Debug, Clone)]
pub struct ModuleData {
    pub name: String,
    pub exports: RefCell<HashMap<String, Value>>,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub name: String,
    pub params: Vec<Param>,
    pub decl: Rc<FunctionDecl>,
}

pub type NativeFn = Rc<dyn Fn(&[Value]) -> crate::RuntimeResult<Value>>;

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.borrow().is_empty(),
            Value::Dict(d) => !d.borrow().is_empty(),
            Value::Set(s) => !s.borrow().is_empty(),
            Value::Tuple(t) => !t.is_empty(),
            Value::Instance(_) => true,
            Value::LangType(_) => true,
            Value::Module(m) => !m.exports.borrow().is_empty(),
            _ => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::Bool(_) => "Bool",
            Value::String(_) => "String",
            Value::Char(_) => "Char",
            Value::Null => "Null",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::Set(_) => "Set",
            Value::Tuple(_) => "Tuple",
            Value::Instance(_) => "Instance",
            Value::LangType(_) => "Type",
            Value::Module(_) => "Module",
            Value::Function(_) => "Function",
            Value::NativeFunction(_) => "Function",
        }
    }

    pub fn collection_length(&self) -> Option<usize> {
        match self {
            Value::List(l) => Some(l.borrow().len()),
            Value::Dict(d) => Some(d.borrow().len()),
            Value::Set(s) => Some(s.borrow().len()),
            Value::Tuple(t) => Some(t.len()),
            Value::String(s) => Some(s.chars().count()),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Char(c) => write!(f, "{c}"),
            Value::Null => write!(f, "null"),
            Value::List(items) => {
                let parts: Vec<String> = items.borrow().iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", parts.join(", "))
            }
            Value::Dict(map) => {
                let parts: Vec<String> = map
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
            Value::Set(items) => {
                let parts: Vec<String> = items.borrow().iter().map(|v| v.to_string()).collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
            Value::Tuple(items) => {
                let parts: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", parts.join(", "))
            }
            Value::Instance(d) => {
                let parts: Vec<String> = d
                    .fields
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect();
                write!(f, "{}({})", d.type_name, parts.join(", "))
            }
            Value::LangType(name) => write!(f, "<type {name}>"),
            Value::Module(m) => write!(f, "<module {}>", m.name),
            Value::Function(func) => write!(f, "<function {}>", func.name),
            Value::NativeFunction(_) => write!(f, "<native function>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::List(b)) => *a.borrow() == *b.borrow(),
            (Value::Dict(a), Value::Dict(b)) => *a.borrow() == *b.borrow(),
            (Value::Set(a), Value::Set(b)) => set_eq(a.borrow().as_slice(), b.borrow().as_slice()),
            (Value::Tuple(a), Value::Tuple(b)) => *a == *b,
            (Value::Instance(a), Value::Instance(b)) => {
                a.type_name == b.type_name && *a.fields.borrow() == *b.fields.borrow()
            }
            (Value::LangType(a), Value::LangType(b)) => a == b,
            (Value::Module(a), Value::Module(b)) => {
                a.name == b.name && *a.exports.borrow() == *b.exports.borrow()
            }
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                *a as f64 == *b
            }
            _ => false,
        }
    }
}

fn set_eq(a: &[Value], b: &[Value]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().all(|v| b.iter().any(|u| v == u))
}

pub fn set_contains(items: &[Value], item: &Value) -> bool {
    items.iter().any(|v| v == item)
}

pub fn set_insert(items: &mut Vec<Value>, item: Value) -> bool {
    if set_contains(items, &item) {
        false
    } else {
        items.push(item);
        true
    }
}
