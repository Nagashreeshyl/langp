# Contributing to Lang.P

Thank you for helping build Lang.P. This project follows practices similar to Rust and Go: spec-first design, comprehensive tests, and honest documentation.

---

## Before you start

1. Read [STATUS.md](STATUS.md) — know what is stable vs beta
2. Read [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md) — do not claim features work if they do not
3. Read [Grammar Freeze v1.0](docs/spec/GRAMMAR-FREEZE-v1.md) — **do not change surface syntax** without an amendment process

---

## Building

**Requirements:** Rust stable (via [rustup](https://rustup.rs)), Node.js 20+ (extension only)

```bash
git clone https://github.com/Nagashreeshyl/langp.git
cd langp
./scripts/build-fast.sh          # lang, langc, lang-lsp
cargo build --workspace          # debug build
```

Run a program:

```bash
./target/release-fast/lang run examples/hello.lp
```

---

## Testing

```bash
cargo test                       # full workspace
cargo test -p langp-interpreter  # interpreter integration tests
cargo test -p langp-parser       # parser conformance
./target/release-fast/lang check examples/hello.lp
```

Every new feature needs tests. Prefer integration tests that parse and run `.lp` source.

---

## Repository structure

```
lexer/          Tokenizer
parser/         Recursive descent parser
ast/            AST definitions (serde JSON export)
semantic/       Static analysis
runtime/        Value types and errors
interpreter/    Tree-walking evaluator
langc/          Compiler CLI library
lang/           User-facing runner
langpm/         Package manager
lang-lsp/       Language Server
editors/        VS Code extensions
examples/       Runnable .lp programs
tests/          Conformance fixtures
docs/           Spec, manual, guides
```

See [TECH-STACK.md](docs/TECH-STACK.md) and [ARCHITECTURE-v0.2.md](docs/ARCHITECTURE-v0.2.md).

---

## Coding standards

See [STYLE_GUIDE.md](STYLE_GUIDE.md). Summary:

- **Rust:** match existing crate style; minimal scope diffs
- **No parser generators** — hand-written lexer/parser for error quality
- **Errors:** beginner-friendly messages with `help:` lines
- **No grammar changes** in drive-by PRs

---

## Documentation rules

When adding or changing behavior:

1. Update [LANGUAGE-REFERENCE.md](docs/guides/LANGUAGE-REFERENCE.md) if user-visible
2. Update the relevant manual chapter in `docs/manual/`
3. Add or fix an example in `examples/`
4. Update [STATUS.md](STATUS.md) if stability level changes
5. Add a [CHANGELOG.md](CHANGELOG.md) entry under `[Unreleased]` or the target version

All code examples must use frozen grammar: statements end with `.`, blocks close with `..`.

---

## Commit message style

Use imperative mood, concise subject (50 chars), optional body:

```
feat(interpreter): add filesystem.exists module export

fix(parser): allow newlines inside dict literals

docs(manual): mark modules chapter as beta for v0.2

test(oop): cover extends inheritance
```

Prefixes: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `release`.

Release commits may use: `feat(v0.2): …`

---

## Branch strategy

- `main` — stable development; must pass `cargo test`
- Feature branches: `feat/short-name`, `fix/issue-123`
- Release tags: `v0.2.0`, `v0.2.1`, etc.

---

## Pull request guidelines

1. One logical change per PR when possible
2. All tests pass locally
3. No unrelated formatting or drive-by refactors
4. Link related issues
5. Update docs if behavior changes
6. Do not mark features **Stable** in STATUS.md unless tested end-to-end

---

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## Security

See [SECURITY.md](SECURITY.md) for reporting vulnerabilities.
