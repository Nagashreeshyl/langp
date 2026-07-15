//! OOP runtime: type registry, instance construction, method dispatch.

use langp_ast::*;
use langp_lexer::Span;
use langp_runtime::{
    InstanceData, RuntimeError, RuntimeErrorKind, RuntimeResult, Value,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FieldMeta {
    pub name: String,
    pub default: Option<Expr>,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub fields: Vec<FieldMeta>,
    pub methods: HashMap<String, Rc<FunctionDecl>>,
    pub static_fields: HashMap<String, Value>,
    pub parent: Option<String>,
}

pub struct TypeRegistry {
    types: HashMap<String, Rc<TypeDef>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn register_from_program(&mut self, items: &[ModuleItem]) -> RuntimeResult<()> {
        let mut extends_map: HashMap<String, String> = HashMap::new();
        for item in items {
            if let ModuleItem::Type(t) = item {
                let mut methods = HashMap::new();
                let mut fields = Vec::new();
                for m in &t.members {
                    match m {
                        TypeMember::Field(f) => {
                            let (name, is_static) = parse_visibility_and_name(&f.name);
                            fields.push(FieldMeta {
                                name,
                                default: f.default.clone(),
                                is_static,
                            });
                        }
                        TypeMember::Function(f) => {
                            let (name, _) = parse_visibility_and_name(&f.name);
                            methods.insert(name, Rc::new(f.clone()));
                        }
                    }
                }
                if let Some(p) = &t.extends {
                    extends_map.insert(t.name.clone(), p.clone());
                }
                self.types.insert(
                    t.name.clone(),
                    Rc::new(TypeDef {
                        name: t.name.clone(),
                        fields,
                        methods,
                        static_fields: HashMap::new(),
                        parent: t.extends.clone(),
                    }),
                );
            }
        }
        for (child, parent) in extends_map {
            self.merge_inheritance(&child, &parent)?;
        }
        Ok(())
    }

    fn merge_inheritance(&mut self, child: &str, parent: &str) -> RuntimeResult<()> {
        let parent_def = self.types.get(parent).cloned().ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::TypeError,
                Span::default(),
                format!("unknown parent type '{parent}'"),
            )
        })?;
        let child_def = self.types.get(child).cloned().ok_or_else(|| {
            RuntimeError::new(
                RuntimeErrorKind::TypeError,
                Span::default(),
                format!("unknown type '{child}'"),
            )
        })?;
        let mut merged_fields = parent_def.fields.clone();
        for f in &child_def.fields {
            if !merged_fields.iter().any(|x| x.name == f.name) {
                merged_fields.push(f.clone());
            } else if let Some(slot) = merged_fields.iter_mut().find(|x| x.name == f.name) {
                *slot = f.clone();
            }
        }
        let mut merged_methods = parent_def.methods.clone();
        merged_methods.extend(child_def.methods.clone());
        self.types.insert(
            child.to_string(),
            Rc::new(TypeDef {
                name: child_def.name.clone(),
                fields: merged_fields,
                methods: merged_methods,
                static_fields: child_def.static_fields.clone(),
                parent: Some(parent.to_string()),
            }),
        );
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Rc<TypeDef>> {
        self.types.get(name).cloned()
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.types.keys()
    }

    pub fn find_method(&self, def: &TypeDef, name: &str) -> Option<Rc<FunctionDecl>> {
        if let Some(m) = def.methods.get(name) {
            return Some(m.clone());
        }
        def.parent
            .as_ref()
            .and_then(|p| self.types.get(p))
            .and_then(|parent| self.find_method(parent, name))
    }

    pub fn instance_field_names(&self, def: &TypeDef) -> Vec<String> {
        def.fields
            .iter()
            .filter(|f| !f.is_static)
            .map(|f| f.name.clone())
            .collect()
    }
}

fn parse_visibility_and_name(raw: &str) -> (String, bool) {
    let s = raw
        .strip_prefix("public ")
        .or_else(|| raw.strip_prefix("private "))
        .or_else(|| raw.strip_prefix("internal "))
        .unwrap_or(raw);
    if let Some(rest) = s.strip_prefix("static ") {
        (rest.to_string(), true)
    } else {
        (s.to_string(), false)
    }
}

pub fn new_instance_data(type_name: &str) -> InstanceData {
    InstanceData {
        type_name: type_name.to_string(),
        fields: RefCell::new(HashMap::new()),
    }
}

pub fn type_name_from_expr(ty: &TypeExpr) -> Option<String> {
    match ty {
        TypeExpr::Named { name, .. } => Some(name.clone()),
        _ => None,
    }
}
