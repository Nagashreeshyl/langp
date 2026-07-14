use langp_parser::parse;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/conformance/parse")
}

#[test]
fn valid_fixtures_parse_without_error() {
    let valid_dir = fixture_root().join("valid");
    for path in list_lp_files(&valid_dir) {
        let source = fs::read_to_string(&path).unwrap();
        parse(&source).unwrap_or_else(|e| {
            panic!("valid fixture failed to parse: {} — {}", path.display(), e);
        });
    }
}

#[test]
fn parse_hello_fixture() {
    let path = fixture_root().join("valid/hello.lp");
    let source = fs::read_to_string(&path).unwrap();
    let program = parse(&source).unwrap();
    assert!(!program.items.is_empty());
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
