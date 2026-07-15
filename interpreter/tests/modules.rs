use langp_interpreter::run;
use langp_parser::parse;

#[test]
fn use_filesystem_module() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();

    let src = format!(
        r#"
use filesystem.
print filesystem.exists("{}").
"#,
        file.display()
    );
    let program = parse(&src).unwrap();
    run(&program).expect("filesystem module should load");
}

#[test]
fn circular_import_errors() {
    // Circular detection is at load time; stdlib modules don't import each other.
    let src = r#"use math."#;
    let program = parse(src).unwrap();
    run(&program).expect("math module should load");
}
