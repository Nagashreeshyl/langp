//! Filesystem standard library module.

use langp_runtime::{ModuleData, RuntimeError, RuntimeErrorKind, RuntimeResult, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub fn module() -> Rc<ModuleData> {
    let mut map = HashMap::new();
    map.insert("read".to_string(), nf(native_read));
    map.insert("read_bytes".to_string(), nf(native_read_bytes));
    map.insert("write".to_string(), nf(native_write));
    map.insert("append".to_string(), nf(native_append));
    map.insert("copy".to_string(), nf(native_copy));
    map.insert("move".to_string(), nf(native_move));
    map.insert("delete".to_string(), nf(native_delete));
    map.insert("exists".to_string(), nf(native_exists));
    map.insert("list".to_string(), nf(native_list));
    map.insert("create_folder".to_string(), nf(native_create_folder));
    map.insert("remove_folder".to_string(), nf(native_remove_folder));
    Rc::new(ModuleData {
        name: "filesystem".to_string(),
        exports: RefCell::new(map),
    })
}

fn nf(f: fn(&[Value]) -> RuntimeResult<Value>) -> Value {
    Value::NativeFunction(Rc::new(f))
}

fn path_arg(args: &[Value], index: usize) -> RuntimeResult<String> {
    match args.get(index) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(v) => Ok(v.to_string()),
        None => Err(RuntimeError::new(
            RuntimeErrorKind::InvalidOperation,
            langp_lexer::Span::default(),
            "missing path argument",
        )),
    }
}

fn io_error(e: std::io::Error) -> RuntimeError {
    RuntimeError::new(
        RuntimeErrorKind::IoError,
        langp_lexer::Span::default(),
        e.to_string(),
    )
}

fn native_read(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    Ok(Value::String(fs::read_to_string(&path).map_err(io_error)?))
}

fn native_read_bytes(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    let bytes = fs::read(&path).map_err(io_error)?;
    Ok(Value::String(String::from_utf8_lossy(&bytes).into_owned()))
}

fn native_write(args: &[Value]) -> RuntimeResult<Value> {
    let content = args
        .first()
        .cloned()
        .unwrap_or(Value::Null)
        .to_string();
    let path = path_arg(args, 1)?;
    fs::write(&path, content).map_err(io_error)?;
    Ok(Value::Null)
}

fn native_append(args: &[Value]) -> RuntimeResult<Value> {
    use std::io::Write;
    let content = args
        .first()
        .cloned()
        .unwrap_or(Value::Null)
        .to_string();
    let path = path_arg(args, 1)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(io_error)?;
    file.write_all(content.as_bytes()).map_err(io_error)?;
    Ok(Value::Null)
}

fn native_copy(args: &[Value]) -> RuntimeResult<Value> {
    let src = path_arg(args, 0)?;
    let dst = path_arg(args, 1)?;
    fs::copy(&src, &dst).map_err(io_error)?;
    Ok(Value::Null)
}

fn native_move(args: &[Value]) -> RuntimeResult<Value> {
    let src = path_arg(args, 0)?;
    let dst = path_arg(args, 1)?;
    fs::rename(&src, &dst).map_err(io_error)?;
    Ok(Value::Null)
}

fn native_delete(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    if Path::new(&path).is_dir() {
        fs::remove_dir_all(&path).map_err(io_error)?;
    } else {
        fs::remove_file(&path).map_err(io_error)?;
    }
    Ok(Value::Null)
}

fn native_exists(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    Ok(Value::Bool(Path::new(&path).exists()))
}

fn native_list(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    let entries = fs::read_dir(&path).map_err(io_error)?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(io_error)?;
        if let Some(name) = entry.file_name().to_str() {
            names.push(Value::String(name.to_string()));
        }
    }
    Ok(Value::List(Rc::new(RefCell::new(names))))
}

fn native_create_folder(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    fs::create_dir_all(&path).map_err(io_error)?;
    Ok(Value::Null)
}

fn native_remove_folder(args: &[Value]) -> RuntimeResult<Value> {
    let path = path_arg(args, 0)?;
    fs::remove_dir_all(&path).map_err(io_error)?;
    Ok(Value::Null)
}
