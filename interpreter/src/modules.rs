//! Module loading, caching, and circular dependency detection.

use langp_runtime::{ModuleData, RuntimeError, RuntimeErrorKind, RuntimeResult, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::stdlib;

pub struct ModuleLoader {
    cache: HashMap<String, Rc<ModuleData>>,
    loading: RefCell<HashSet<String>>,
    project_root: Option<PathBuf>,
    packages_dir: PathBuf,
}

impl ModuleLoader {
    pub fn new(project_root: Option<PathBuf>) -> Self {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        Self {
            cache: HashMap::new(),
            loading: RefCell::new(HashSet::new()),
            project_root,
            packages_dir: PathBuf::from(home).join(".cache/langp/packages"),
        }
    }

    pub fn load(&mut self, path: &[String]) -> RuntimeResult<Value> {
        let key = path.join(".");
        if let Some(m) = self.cache.get(&key) {
            return Ok(Value::Module(m.clone()));
        }
        if self.loading.borrow().contains(&key) {
            return Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation,
                langp_lexer::Span::default(),
                format!("circular import detected: {key}"),
            ));
        }
        self.loading.borrow_mut().insert(key.clone());

        let module = if path.len() == 1 {
            self.load_single(&path[0])?
        } else {
            return Err(RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                langp_lexer::Span::default(),
                format!("nested module import '{key}' not yet supported"),
            ));
        };

        self.loading.borrow_mut().remove(&key);
        self.cache.insert(key, module.clone());
        Ok(Value::Module(module))
    }

    fn load_single(&self, name: &str) -> RuntimeResult<Rc<ModuleData>> {
        if let Some(m) = stdlib::builtin_module(name) {
            return Ok(m);
        }
        if let Some(root) = &self.project_root {
            let candidates = [
                root.join("src").join(format!("{name}.lp")),
                root.join(format!("{name}.lp")),
                self.packages_dir.join(name).join("lib.lp"),
                self.packages_dir
                    .join(format!("{name}-0.1.0"))
                    .join("lib.lp"),
            ];
            for path in candidates {
                if path.is_file() {
                    return self.load_lp_module(name, &path);
                }
            }
        }
        Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation,
            langp_lexer::Span::default(),
            format!("module '{name}' not found (stdlib, src/, or packages cache)"),
        ))
    }

    fn load_lp_module(&self, name: &str, path: &Path) -> RuntimeResult<Rc<ModuleData>> {
        let _source = std::fs::read_to_string(path).map_err(|e| {
            RuntimeError::new(
                RuntimeErrorKind::IoError,
                langp_lexer::Span::default(),
                format!("failed to read module '{}': {e}", path.display()),
            )
        })?;
        // v0.2: project .lp modules re-use stdlib stub exports until full module eval lands
        stdlib::builtin_module(name).ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::NotImplemented,
                langp_lexer::Span::default(),
                format!(
                    "module file '{}' found but per-file module eval is not yet enabled",
                    path.display()
                ),
            )
        })
    }
}
