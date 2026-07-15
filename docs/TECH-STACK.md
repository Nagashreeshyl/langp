# Lang.P Tech Stack

**Version:** 0.1.x  
**Last updated:** 2026-07-15  
**Audience:** Contributors, compiler engineers, and anyone extending Lang.P tooling

This document describes the **complete technology stack** used to build Lang.P today — from source text to running programs and IDE integration — plus the planned stack for future phases.

---

## 1. Stack at a glance

| Layer | Technology | Status |
|-------|------------|--------|
| **Host language** | Rust 2021 (stable) | Implemented |
| **Build system** | Cargo workspace | Implemented |
| **Lexer** | Hand-written recursive scanner | Implemented |
| **Parser** | Hand-written recursive descent | Implemented |
| **AST** | Rust enums + `serde` JSON export | Implemented |
| **Semantic analysis** | Tree walk + type inference | Implemented |
| **Execution** | Tree-walking interpreter | Implemented |
| **Native codegen** | — | Planned |
| **LSP server** | `tower-lsp` + Tokio | Partial |
| **IDE extension** | TypeScript + VS Code API | Implemented |
| **Syntax highlighting** | TextMate grammar (JSON) | Implemented |
| **CI / releases** | GitHub Actions | Implemented |
| **Docs** | Markdown + EBNF | Implemented |
| **Package manager** | — | Planned |

---

## 2. Architecture overview

Lang.P follows a **classic multi-phase compiler pipeline**. Each phase is a separate Rust crate with a narrow public API, so lexer/parser/semantic work can be tested independently of execution.

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────────┐
│  Source     │───▶│   Lexer     │───▶│   Parser    │───▶│     AST      │
│  (.lp file) │    │ langp-lexer │    │ langp-parser│    │  langp-ast   │
└─────────────┘    └─────────────┘    └─────────────┘    └──────┬───────┘
                                                                 │
                    ┌─────────────┐    ┌─────────────┐           │
                    │  lang check │◀───│  Semantic   │◀──────────┘
                    │  (CLI/IDE)  │    │ langp-semantic
                    └─────────────┘    └──────┬──────┘
                                              │
                    ┌─────────────┐    ┌──────▼──────┐
                    │   Runtime   │◀───│ Interpreter │
                    │ langp-runtime│   │langp-interpreter
                    └─────────────┘    └─────────────┘
```

**Data flow for `lang run hello.lp`:**

1. Read source from disk
2. `langp_lexer::lex` → token stream
3. `langp_parser::parse` → `Program` AST
4. `langp_semantic::analyze` → diagnostics (errors block run)
5. `langp_interpreter::run` → execute AST using `langp_runtime::Value`

**Debug / inspect path (`langc`):**

```bash
langc --emit tokens hello.lp   # token dump
langc --emit ast hello.lp      # JSON AST (serde)
langc hello.lp --mode check    # semantic only
```

---

## 3. Rust workspace

The repository is a **Cargo workspace** with nine member crates. Shared version, edition, license, and path dependencies are declared in the root `Cargo.toml`.

```
/langp
├── lexer/           langp-lexer
├── ast/             langp-ast
├── parser/          langp-parser
├── semantic/        langp-semantic
├── runtime/         langp-runtime
├── interpreter/     langp-interpreter
├── langc/           langc CLI + shared CLI library
├── lang/            lang CLI (user-facing runner)
└── lang-lsp/        lang-lsp binary
```

### 3.1 Crate dependency graph

```
lang, langc ──▶ langc (lib) ──▶ interpreter, semantic, parser, lexer, ast
lang-lsp ──────▶ semantic, parser, lexer, ast
interpreter ───▶ runtime, ast, lexer
semantic ──────▶ ast, lexer
parser ────────▶ ast, lexer
runtime ───────▶ ast, lexer
ast ───────────▶ lexer, serde, serde_json
lexer ─────────▶ serde
```

### 3.2 Build profiles

| Profile | Purpose | Key settings |
|---------|---------|--------------|
| `dev` | Local development | `opt-level = 1`, deps at `opt-level = 2` |
| `release` | Distribution binaries | `lto = thin`, `strip = true`, `opt-level = 3` |
| `release-fast` | CI + install scripts | Inherits release, `lto = false`, `codegen-units = 256` |

Daily development:

```bash
./scripts/build-fast.sh          # lang + langc + lang-lsp (release-fast)
cargo test                       # full workspace tests
cargo build --release -p langc   # smallest production binary
```

---

## 4. Core compiler crates

### 4.1 Lexer (`langp-lexer`)

| Item | Detail |
|------|--------|
| **Approach** | Hand-written scanner (not `logos` / `nom`) |
| **Indentation** | Python-style `INDENT` / `DEDENT` tokens (4-space rule) |
| **Delimiter tracking** | Nesting depth for `()`, `[]`, `{}` — indentation suppressed inside |
| **Block close** | `..` recognized at line start as block terminator |
| **Comments** | `@` to end of line |
| **Output** | `Vec<Token>` with `Span` (byte offset, line, column) |
| **Dependencies** | `serde` (token serialization) |

Key files: `lexer/src/lexer.rs`, `lexer/src/token.rs`, `lexer/src/span.rs`

### 4.2 Parser (`langp-parser`)

| Item | Detail |
|------|--------|
| **Approach** | Recursive descent, single-pass |
| **Grammar source** | Frozen in `docs/spec/GRAMMAR-FREEZE-v1.md` + `docs/grammar/` EBNF |
| **Disambiguation** | `{a: 1}` → Dict; `{1, 2}` → Set (colon after first element) |
| **Blocks** | `INDENT` … `DEDENT` with `..` close validation |
| **Output** | `langp_ast::Program` |
| **Errors** | Structured `ParseError` with beginner-friendly messages |

Key file: `parser/src/parser.rs`

### 4.3 AST (`langp-ast`)

| Item | Detail |
|------|--------|
| **Representation** | Rust `enum` / `struct` tree |
| **Serialization** | `serde` + `serde_json` for `--emit ast` |
| **Nodes** | `Program`, `Stmt`, `Expr`, `TypeExpr`, `FunctionDecl`, collections, etc. |
| **Spans** | Every node carries `Span` for diagnostics |

Key file: `ast/src/nodes.rs`

### 4.4 Semantic analyzer (`langp-semantic`)

| Item | Detail |
|------|--------|
| **Approach** | AST walk with scoped symbol tables |
| **Checks** | Undefined names, duplicate definitions, type mismatches |
| **Types** | `List<T>`, `Dictionary<K,V>`, `Set<T>`, primitives; inference + annotation validation |
| **Output** | `Vec<Diagnostic>` with severity, span, error codes (E01xx) |

Key files: `semantic/src/analyze.rs`, `semantic/src/types.rs`, `semantic/src/diagnostic.rs`

### 4.5 Runtime (`langp-runtime`)

| Item | Detail |
|------|--------|
| **Values** | `Int`, `Float`, `Bool`, `String`, `Null`, `List`, `Dict`, `Set`, `Tuple`, `Function`, `NativeFunction` |
| **Errors** | `RuntimeError` / `RuntimeErrorKind` (type, index, undefined, etc.) |
| **Mutability** | `Rc<RefCell<>>` for lists/dicts/sets; tuples are immutable `Rc<Vec>` |

Key file: `runtime/src/value.rs`

### 4.6 Interpreter (`langp-interpreter`)

| Item | Detail |
|------|--------|
| **Approach** | Tree-walking evaluator |
| **Environment** | Lexical scopes via `Rc<Environment>` |
| **Builtins** | `print`, `input`, `read`, `write`, `len` |
| **Collections** | Method dispatch in `collections.rs` (append, keys, union, etc.) |
| **Control flow** | `Flow` enum for break/continue/return |

Key files: `interpreter/src/eval.rs`, `interpreter/src/collections.rs`, `interpreter/src/builtins.rs`

---

## 5. CLI binaries

Two user-facing binaries share logic in `langc/src/lib.rs`:

| Binary | Role | Typical commands |
|--------|------|------------------|
| **`lang`** | Package manager / runner (beginner-facing) | `lang run file.lp`, `lang check file.lp`, `lang file.lp` |
| **`langc`** | Compiler tooling (advanced) | `langc --emit ast`, `langc --emit tokens`, `lang build` |
| **`lang-lsp`** | Language Server Protocol | Started by IDE extension (optional) |

**Pipeline inside CLI:**

```rust
lex → parse → analyze → interpret | emit | build
```

**Build mode (`lang build`):** Generates a platform launcher script (`.sh` / `.bat`) that invokes `lang run` on the embedded source path — not native machine code yet.

---

## 6. Language Server (`lang-lsp`)

| Item | Detail |
|------|--------|
| **Framework** | [`tower-lsp`](https://github.com/ebkalderon/tower-lsp) 0.20 |
| **Async runtime** | Tokio (full features) |
| **Capabilities** | Document sync, hover (partial), go-to-definition (functions) |
| **Diagnostics** | Disabled in LSP — extension runs `lang check` instead (single source of truth) |
| **Transport** | stdio (VS Code Language Client) |

Key files: `lang-lsp/src/main.rs`, `lang-lsp/src/server.rs`

**Design decision:** IDE squiggles and help text come from `lang check` subprocess output parsed by the extension, not from LSP publishDiagnostics. This avoids duplicate/stale diagnostics and keeps error messages consistent with the CLI.

---

## 7. IDE & editor stack

Lang.P targets all **VS Code–compatible editors**: VS Code, Cursor, Windsurf, Antigravity.

### 7.1 Two-extension model

| Extension | Package | Purpose |
|-----------|---------|---------|
| **Lang.P Grammar** | `editors/langp-grammar` | Syntax colors, file icon, snippets, auto-indent rules |
| **Lang.P Services** | `editors/vscode-langp` | Diagnostics, autocomplete, run command, optional LSP |

Services depends on Grammar (`extensionDependencies`).

### 7.2 Grammar extension stack

| Component | Technology |
|-----------|------------|
| Syntax highlighting | TextMate grammar (`syntaxes/langp.tmLanguage.json`) |
| Language config | `language-configuration.json` (brackets, comments, indent rules) |
| Snippets | `snippets/langp.json` |
| File icon | SVG + VS Code icon theme contribution |
| Manifest | Autocomplete metadata in `langp-manifest.json` |

### 7.3 Services extension stack

| Component | Technology |
|-----------|------------|
| Language | TypeScript 5.x |
| VS Code API | `@types/vscode` ^1.85 |
| LSP client | `vscode-languageclient` ^9 |
| Packaging | `@vscode/vsce` |
| Diagnostics | Spawn `lang check`, parse stdout |
| Completions | Built-in keyword/snippet/manifest + variable extraction |
| Commands | Run file, check file, set language mode |

Key files: `editors/vscode-langp/src/extension.ts`, `editors/vscode-langp/src/langp-api.ts`

### 7.4 Supported editor features (v0.1)

- Syntax highlighting and `.lp` file association
- Auto-indent after `,`; de-indent on `..`
- Error/warning underlines via `lang check`
- Ctrl+Space autocomplete (keywords, snippets, builtins, locals)
- Optional LSP hover / go-to-definition
- Run current file command

---

## 8. Specification & documentation stack

| Asset | Location | Role |
|-------|----------|------|
| Grammar freeze | `docs/spec/GRAMMAR-FREEZE-v1.md` | **Single source of truth** for surface syntax |
| Full spec (22 chapters) | `docs/spec/` | Semantic rules, stdlib, tooling roadmap |
| EBNF grammar | `docs/grammar/` | Machine-readable formal grammar |
| Language manual | `docs/manual/` (25 chapters) | User-facing professional guide |
| Quick reference | `docs/guides/LANGUAGE-REFERENCE.md` | Implemented features only |
| Beginner tutorial | `docs/guides/HOW-TO-CODE.md` | Hands-on learning |
| Cursor rules | `.cursor/rules/langp.mdc` | AI assistant conventions |

**Rule:** Implementations MUST NOT change frozen grammar syntax. Spec amendments follow `docs/spec/22-compatibility-versioning.md`.

---

## 9. Testing stack

| Layer | Framework | Location |
|-------|-----------|----------|
| Unit / integration | Rust built-in `#[test]` | `*/tests/`, `*/src/` |
| Lexer conformance | Fixture-driven | `lexer/tests/conformance.rs` → `tests/conformance/parse/` |
| Parser conformance | Fixture-driven | `parser/tests/conformance.rs` → `tests/conformance/parse/` |
| Interpreter integration | End-to-end run | `interpreter/tests/collections.rs`, etc. |
| Semantic types | Type validation | `semantic/tests/types.rs` |
| Examples | Manual smoke | `examples/*.lp` |

**Conformance layout:**

```
tests/conformance/parse/
├── valid/       # must parse (and usually lex) without error
└── invalid/     # must produce expected errors
```

Run everything:

```bash
cargo test                    # all workspace tests
lang run examples/hello.lp    # smoke test
lang check examples/*.lp      # semantic validation
```

---

## 10. CI/CD & distribution

### 10.1 GitHub Actions

Workflow: `.github/workflows/release.yml`

| Trigger | `push` tags `v*` or manual dispatch |
|---------|---------------------------------------|
| **Build matrix** | Linux x64/ARM64, macOS x64/ARM64, Windows x64 |
| **Profile** | `release-fast` |
| **Artifacts** | `lang`, `langc`, `lang-lsp` per target triple |
| **Extension** | Node 20 → `npm run compile` → VSIX packaging |

### 10.2 Install scripts

| Platform | Script | Install location |
|----------|--------|------------------|
| macOS / Linux | `scripts/install.sh` | `~/.local/bin` |
| Windows | `scripts/install.ps1` | `%USERPROFILE%\.local\bin` |

Install flow:

1. Download pre-built binaries from GitHub Releases (or build from source via Cargo)
2. Add install dir to PATH
3. Copy VS Code extensions to `~/.vscode/extensions` and `~/.cursor/extensions`

Uninstall: `scripts/uninstall.sh` / `scripts/uninstall.ps1`

### 10.3 Release targets

| Triple | Platform |
|--------|----------|
| `x86_64-unknown-linux-gnu` | Linux 64-bit |
| `aarch64-unknown-linux-gnu` | Linux ARM64 |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-apple-darwin` | macOS Apple Silicon |
| `x86_64-pc-windows-msvc` | Windows 64-bit |

---

## 11. Third-party dependencies

### 11.1 Rust (runtime / compiler)

| Crate | Used by | Purpose |
|-------|---------|---------|
| `serde` | lexer, ast | Serialize tokens / AST |
| `serde_json` | ast, lang-lsp | JSON AST export, LSP payloads |
| `tower-lsp` | lang-lsp | LSP server framework |
| `tokio` | lang-lsp | Async I/O for LSP |
| `glob` | lexer, parser (dev) | Conformance test file discovery |

No external parser generator (no LALRPOP, pest, etc.) — full control over error messages and grammar freeze compliance.

### 11.2 TypeScript (IDE)

| Package | Purpose |
|---------|---------|
| `typescript` | Compile extension |
| `vscode-languageclient` | LSP client |
| `@vscode/vsce` | Package VSIX |
| `@types/node`, `@types/vscode` | Type definitions |

---

## 12. Platform support matrix

| Feature | macOS | Linux | Windows |
|---------|-------|-------|---------|
| `lang` / `langc` / `lang-lsp` | Yes | Yes | Yes |
| One-line install script | Yes | Yes | Yes (PowerShell) |
| IDE extension | Yes | Yes | Yes |
| Build bundles (`.sh` / `.bat`) | Yes | Yes | Yes |
| Pre-built release binaries | Yes | Yes | Yes |

---

## 13. Planned stack (not yet implemented)

These appear in the language specification and roadmap but are **not** in the current codebase:

| Component | Planned technology / approach |
|-----------|------------------------------|
| **Native codegen** | LLVM or Cranelift backend from AST/IR |
| **Bytecode VM** | Custom bytecode + interpreter |
| **Package manager** | `lang install`, lock files, registry |
| **Formatter** | `langfmt` (likely AST-based pretty-printer) |
| **REPL** | `lang-repl` (line-at-a-time parse + eval) |
| **Debugger** | DAP (Debug Adapter Protocol) |
| **Lang Studio** | Standalone desktop IDE (beyond VS Code extension) |
| **Navigator** | Embedded Chromium for browser automation |
| **AI framework** | Agent/tool-calling runtime |
| **Package registry** | `langp.dev` or GitHub-based |

Current execution is **interpreted only**. `langc build` produces launcher scripts, not native executables.

---

## 14. Development workflow

### 14.1 Prerequisites

- **Rust** stable (via [rustup](https://rustup.rs))
- **Node.js** 20+ (for IDE extension only)
- **Git**

### 14.2 Clone → build → run

```bash
git clone https://github.com/Nagashreeshyl/langp.git
cd langp
./scripts/build-fast.sh
./target/release-fast/lang run examples/hello.lp
```

### 14.3 Working on a compiler phase

| Task | Crate to edit | Test command |
|------|---------------|--------------|
| Token / indent rules | `lexer/` | `cargo test -p langp-lexer` |
| Syntax / AST shape | `parser/`, `ast/` | `cargo test -p langp-parser` |
| Type checking | `semantic/` | `cargo test -p langp-semantic` |
| Runtime behavior | `interpreter/`, `runtime/` | `cargo test -p langp-interpreter` |
| CLI flags | `langc/` | `cargo run -p langc -- --help` |
| IDE behavior | `editors/vscode-langp/` | `npm run compile && code --install-extension *.vsix` |

### 14.4 Adding a language feature (checklist)

1. Update spec / grammar freeze (if syntax changes — rare; grammar is frozen)
2. Lexer: new tokens if needed
3. Parser + AST: new nodes
4. Semantic: validation rules
5. Runtime + interpreter: evaluation
6. Conformance fixtures in `tests/conformance/parse/`
7. Integration test + example in `examples/`
8. Document in `docs/guides/LANGUAGE-REFERENCE.md`

---

## 15. Repository layout (full)

```
/langp
├── ast/                 # Abstract syntax tree
├── lexer/               # Tokenizer
├── parser/              # Parser
├── semantic/            # Static analysis + types
├── runtime/             # Value types, runtime errors
├── interpreter/         # Tree-walking evaluator
├── langc/               # Compiler CLI + shared CLI library
├── lang/                # User-facing runner CLI
├── lang-lsp/            # Language Server
├── editors/
│   ├── langp-grammar/   # Syntax-only VS Code extension
│   └── vscode-langp/    # Services VS Code extension
├── scripts/             # install, build, IDE setup
├── examples/            # Sample .lp programs
├── tests/conformance/   # Parse/lex fixtures
├── docs/
│   ├── spec/            # Language specification
│   ├── grammar/         # EBNF
│   ├── manual/          # 25-chapter manual
│   └── guides/          # Tutorials + reference
├── filesystem/          # (planned) stdlib filesystem module docs
├── ai/                  # (planned) AI framework docs
└── .github/workflows/   # CI / release
```

---

## 16. Key design principles

1. **Spec-first** — grammar and semantics are documented before (or alongside) implementation.
2. **Modular crates** — each pipeline stage is independently testable.
3. **Beginner-friendly errors** — parse and semantic diagnostics include help text.
4. **Single diagnostic source** — IDE uses `lang check`, not duplicate LSP analysis.
5. **Frozen grammar** — `.` / `,` / `..` block rules are not changed casually.
6. **No syntax changes for features** — collections, loops, etc. use existing grammar productions.

---

## 17. Related documents

| Document | Description |
|----------|-------------|
| [README.md](../README.md) | Project overview and quick install |
| [Grammar Freeze v1](spec/GRAMMAR-FREEZE-v1.md) | Official syntax rules |
| [Tooling spec (Ch. 21)](spec/21-tooling.md) | Planned toolchain (future-facing) |
| [Language Reference](guides/LANGUAGE-REFERENCE.md) | What works in v0.1 |
| [Manual index](manual/README.md) | Full user manual |

---

## 18. Summary

Lang.P is built as a **Rust-native, spec-driven compiler toolchain** with a hand-written lexer/parser, tree-walking interpreter, and TypeScript VS Code extensions. The stack prioritizes **readability, modular testing, and beginner-friendly tooling** over premature optimization. Native compilation, package management, and advanced IDE features are specified and planned but not yet part of the implemented stack.

For questions or amendments to this document, open an issue or PR against the main repository.
