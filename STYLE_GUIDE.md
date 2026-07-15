# Lang.P Style Guide

Consistent style helps Lang.P feel like a professional language project. When in doubt, match the surrounding code.

---

## Naming conventions

| Item | Convention | Example |
|------|------------|---------|
| Lang.P variables | snake_case | `user_name`, `total_count` |
| Lang.P functions | snake_case | `greet_user`, `calculate_total` |
| Lang.P types | PascalCase | `User`, `BankAccount` |
| Lang.P modules | lowercase | `filesystem`, `navigator` |
| Rust crates | `langp-*` prefix | `langp-lexer`, `langp-parser` |
| Rust functions | snake_case | `parse_type_decl` |
| Rust types | PascalCase | `TypeRegistry`, `RuntimeError` |
| Files | snake_case.rs / kebab-case.md | `collections.rs`, `LANGUAGE-REFERENCE.md` |

---

## Lang.P source formatting

```lp
@ Comment explaining intent.
function greet(name),
    print "Hello " with name with "!".
..

if score >= 90,
    print "A".
otherwise,
    print "B".
..
```

Rules:

- **4 spaces** per indent level (no tabs)
- Every statement ends with **`.`**
- Every block opens with **`,`** and closes with **`..`**
- Use **`with`** for string composition (never `+` for strings)
- Comments start with **`@`**
- One statement per line when possible

---

## Example formatting

Examples in `examples/` must:

- Start with `@ filename.lp — description`
- Run with `lang run` without errors (except interactive demos noted in README)
- Use realistic but minimal code
- Label stub/framework examples clearly in comments

---

## Documentation formatting

- Manual chapters: status badge at top (`Implemented`, `Beta`, `Specification`)
- Code fences use `lp` language tag
- Link to [Grammar Freeze](docs/spec/GRAMMAR-FREEZE-v1.md) for syntax disputes
- Never show `end` or `end.` as block closers
- Prefer `..` in all block examples

---

## Error message style

Diagnostics follow this pattern:

```
error[E0202]: every statement must end with `.`, found NEWLINE
  help: add `.` at the end of this line
  --> source:12:5
```

Guidelines:

- Error code: `E01xx` lex, `E02xx` parse/semantic
- Message: plain English, no jargon
- Always include a **`help:`** line when fix is obvious
- Point to span with `--> file:line:col`

---

## Compiler and runtime diagnostics

- Prefer **specific** errors over generic "invalid syntax"
- Distinguish **parse** vs **semantic** vs **runtime** failures
- Runtime: include `TypeError`, `UndefinedVariable`, `IoError` kinds consistently

---

## Testing style

```rust
#[test]
fn user_constructor_sets_fields() {
    let src = r#"
type User, name. ..
user = User(). user.name = "Naga".
"#;
    run_source(src).expect("should run");
}
```

- Integration tests use inline `.lp` strings or fixtures
- Test names describe behavior, not implementation
- Conformance fixtures: `tests/conformance/parse/valid/` and `invalid/`

---

## Rust code style

- Edition 2021
- Minimize public API surface per crate
- `Result<T, RuntimeError>` for runtime failures
- Avoid unwrap in library code; OK in tests
- Run `cargo fmt` before submitting (if configured)

---

## Git and releases

- [Keep a Changelog](https://keepachangelog.com/) format for CHANGELOG.md
- Tag releases `vMAJOR.MINOR.PATCH`
- Update workspace `version` in root `Cargo.toml`
