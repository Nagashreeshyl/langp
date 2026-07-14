use langp_runtime::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    values: Rc<RefCell<HashMap<String, Value>>>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        }
    }

    pub fn child(parent: Rc<Environment>) -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(parent),
        }
    }

    pub fn define(&self, name: impl Into<String>, value: Value) {
        self.values.borrow_mut().insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.values.borrow().get(name) {
            return Some(v.clone());
        }
        self.parent.as_ref().and_then(|p| p.get(name))
    }

    pub fn set(&self, name: &str, value: Value) -> bool {
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), value);
            return true;
        }
        if let Some(p) = &self.parent {
            return p.set(name, value);
        }
        false
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
