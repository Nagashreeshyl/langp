use langp_ast::{Expr, TypeExpr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDesc {
    Int,
    Float,
    Bool,
    String,
    Char,
    Null,
    List(Box<TypeDesc>),
    Dict(Box<TypeDesc>, Box<TypeDesc>),
    Set(Box<TypeDesc>),
    Tuple(Vec<TypeDesc>),
    Any,
}

pub fn from_type_expr(ty: &TypeExpr) -> TypeDesc {
    match ty {
        TypeExpr::Named { name, generics, .. } => match name.as_str() {
            "Int" => TypeDesc::Int,
            "Float" => TypeDesc::Float,
            "Bool" => TypeDesc::Bool,
            "String" => TypeDesc::String,
            "Char" => TypeDesc::Char,
            "Any" => TypeDesc::Any,
            "List" => {
                let elem = generics
                    .first()
                    .map(from_type_expr)
                    .unwrap_or(TypeDesc::Any);
                TypeDesc::List(Box::new(elem))
            }
            "Dictionary" | "Dict" => {
                let key = generics
                    .first()
                    .map(from_type_expr)
                    .unwrap_or(TypeDesc::String);
                let val = generics
                    .get(1)
                    .map(from_type_expr)
                    .unwrap_or(TypeDesc::Any);
                TypeDesc::Dict(Box::new(key), Box::new(val))
            }
            "Set" => {
                let elem = generics
                    .first()
                    .map(from_type_expr)
                    .unwrap_or(TypeDesc::Any);
                TypeDesc::Set(Box::new(elem))
            }
            _ => TypeDesc::Any,
        },
        TypeExpr::Tuple { types, .. } => {
            TypeDesc::Tuple(types.iter().map(from_type_expr).collect())
        }
        TypeExpr::Optional { inner, .. } => from_type_expr(inner),
    }
}

pub fn infer_expr(expr: &Expr) -> TypeDesc {
    match expr {
        Expr::Int { .. } => TypeDesc::Int,
        Expr::Float { .. } => TypeDesc::Float,
        Expr::Bool { .. } => TypeDesc::Bool,
        Expr::String { .. } => TypeDesc::String,
        Expr::Char { .. } => TypeDesc::Char,
        Expr::Null { .. } => TypeDesc::Null,
        Expr::List { elements, .. } => {
            if elements.is_empty() {
                return TypeDesc::List(Box::new(TypeDesc::Any));
            }
            let first = infer_expr(&elements[0]);
            for e in elements.iter().skip(1) {
                let t = infer_expr(e);
                if !compatible(&first, &t) && first != TypeDesc::Any && t != TypeDesc::Any {
                    return TypeDesc::List(Box::new(TypeDesc::Any));
                }
            }
            TypeDesc::List(Box::new(first))
        }
        Expr::Set { elements, .. } => {
            if elements.is_empty() {
                return TypeDesc::Set(Box::new(TypeDesc::Any));
            }
            let first = infer_expr(&elements[0]);
            TypeDesc::Set(Box::new(first))
        }
        Expr::Dict { entries, .. } => {
            if entries.is_empty() {
                return TypeDesc::Dict(Box::new(TypeDesc::String), Box::new(TypeDesc::Any));
            }
            let (_, v0) = &entries[0];
            let val = infer_expr(v0);
            TypeDesc::Dict(Box::new(TypeDesc::String), Box::new(val))
        }
        Expr::Tuple { elements, .. } => {
            TypeDesc::Tuple(elements.iter().map(infer_expr).collect())
        }
        _ => TypeDesc::Any,
    }
}

pub fn compatible(expected: &TypeDesc, actual: &TypeDesc) -> bool {
    if expected == actual {
        return true;
    }
    if *expected == TypeDesc::Any || *actual == TypeDesc::Any {
        return true;
    }
    match (expected, actual) {
        (TypeDesc::List(e), TypeDesc::List(a)) => {
            if **a == TypeDesc::Any && **e != TypeDesc::Any {
                return false;
            }
            compatible(e, a)
        }
        (TypeDesc::Set(e), TypeDesc::Set(a)) => {
            if **a == TypeDesc::Any && **e != TypeDesc::Any {
                return false;
            }
            compatible(e, a)
        }
        (TypeDesc::Dict(ek, ev), TypeDesc::Dict(ak, av)) => {
            if (**ak == TypeDesc::Any && **ek != TypeDesc::Any)
                || (**av == TypeDesc::Any && **ev != TypeDesc::Any)
            {
                return false;
            }
            compatible(ek, ak) && compatible(ev, av)
        }
        (TypeDesc::Tuple(et), TypeDesc::Tuple(at)) => {
            et.len() == at.len() && et.iter().zip(at.iter()).all(|(e, a)| compatible(e, a))
        }
        (TypeDesc::Int, TypeDesc::Float) | (TypeDesc::Float, TypeDesc::Int) => true,
        _ => false,
    }
}

pub fn type_label(ty: &TypeDesc) -> String {
    match ty {
        TypeDesc::Int => "Int".into(),
        TypeDesc::Float => "Float".into(),
        TypeDesc::Bool => "Bool".into(),
        TypeDesc::String => "String".into(),
        TypeDesc::Char => "Char".into(),
        TypeDesc::Null => "Null".into(),
        TypeDesc::Any => "Any".into(),
        TypeDesc::List(e) => format!("List<{}>", type_label(e)),
        TypeDesc::Set(e) => format!("Set<{}>", type_label(e)),
        TypeDesc::Dict(k, v) => format!("Dictionary<{}, {}>", type_label(k), type_label(v)),
        TypeDesc::Tuple(ts) => {
            let inner: Vec<_> = ts.iter().map(type_label).collect();
            format!("({})", inner.join(", "))
        }
    }
}
