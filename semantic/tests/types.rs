use langp_parser::parse;
use langp_semantic::analyze;

#[test]
fn list_int_annotation_ok() {
    let src = "nums: List<Int> = [1, 2, 3].";
    let program = parse(src).unwrap();
    let result = analyze(&program);
    assert!(result.is_ok(), "{:?}", result.diagnostics);
}

#[test]
fn list_int_annotation_mismatch() {
    let src = r#"nums: List<Int> = [1, "x", 3]."#;
    let program = parse(src).unwrap();
    let result = analyze(&program);
    assert!(!result.is_ok());
}

#[test]
fn dictionary_string_int_annotation() {
    let src = r#"scores: Dictionary<String, Int> = { a : 1, b : 2 }."#;
    let program = parse(src).unwrap();
    let result = analyze(&program);
    assert!(result.is_ok(), "{:?}", result.diagnostics);
}
