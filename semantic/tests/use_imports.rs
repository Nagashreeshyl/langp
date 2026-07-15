use langp_parser::parse;
use langp_semantic::analyze;

#[test]
fn use_import_makes_module_name_available() {
    let src = r#"
use ai.
print "version: " with ai.version.
"#;
    let program = parse(src).unwrap();
    let result = analyze(&program);
    assert!(
        result.is_ok(),
        "expected no errors, got: {:?}",
        result.diagnostics
    );
}
