use langp_lexer::{lex, TokenKind};
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/conformance/parse")
}

#[test]
fn valid_fixtures_lex_without_error() {
    let valid_dir = fixture_root().join("valid");
    let files = list_lp_files(&valid_dir);
    assert!(!files.is_empty(), "expected valid parse fixtures");

    for path in files {
        let source = fs::read_to_string(&path).unwrap();
        match lex(&source) {
            Ok(tokens) => {
                assert!(
                    tokens.last().map(|t| t.kind == TokenKind::Eof).unwrap_or(false),
                    "{}: missing EOF",
                    path.display()
                );
            }
            Err(e) => panic!("valid fixture failed to lex: {} — {}", path.display(), e),
        }
    }
}

#[test]
fn hello_fixture_has_expected_tokens() {
    let path = fixture_root().join("valid/hello.lp");
    let source = fs::read_to_string(&path).unwrap();
    let tokens = lex(&source).unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| t.kind.to_string()).collect();
    assert!(kinds.iter().any(|k| k == "print"));
    assert!(kinds.iter().any(|k| k == "input"));
    assert!(kinds.iter().any(|k| k == "with"));
}

fn list_lp_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "lp"))
        .collect();
    files.sort();
    files
}
