use langp_interpreter::run;
use langp_parser::parse;

#[test]
fn read_write_copy_delete() {
    let dir = tempfile::tempdir().unwrap();
    let src_path = dir.path().join("source.txt");
    let dst_path = dir.path().join("dest.txt");
    std::fs::write(&src_path, "content").unwrap();

    let src = format!(
        r#"
text = read "{}".
write text to "{}".
copy "{}" to "{}".
print text.
"#,
        src_path.display(),
        dst_path.display(),
        src_path.display(),
        dir.path().join("copy.txt").display()
    );
    let program = parse(&src).unwrap();
    run(&program).expect("filesystem io should work");
}

#[test]
fn append_write_bytes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("log.txt");

    let src = format!(
        r#"
write "a" to "{}".
append "b" to "{}".
"#,
        path.display(),
        path.display()
    );
    let program = parse(&src).unwrap();
    run(&program).expect("append should work");
    let data = std::fs::read_to_string(&path).unwrap();
    assert_eq!(data, "ab");
}
