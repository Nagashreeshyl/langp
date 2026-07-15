# Changelog

All notable changes to Lang.P are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - 2026-07-15

### Added

- **Collections:** List, Dictionary, Set, Tuple with methods, iteration, and generic type annotations
- **Object system (beta):** `type` declarations, fields, methods, `init`, `self`, single inheritance (`extends`)
- **Modules (beta):** `use` imports, module cache, circular import detection, stdlib modules
- **Filesystem stdlib:** `use filesystem.` with `exists`, `list`, `create_folder`, `remove_folder`, and related APIs
- **Package manager (beta):** `langpm` crate — `lang init`, `install`, `remove`, `update`, `search`, `build`, `test`, `fmt`, `doctor`
- **Manifest and lock file:** `langp.toml`, `langp.lock`, offline package cache
- **Documentation:** `STATUS.md`, `KNOWN_LIMITATIONS.md`, `ROADMAP.md`, `CONTRIBUTING.md`, `STYLE_GUIDE.md`, `ARCHITECTURE-v0.2.md`, `TECH-STACK.md`
- **Examples:** `collections.lp`, `oop.lp`, `modules.lp`, `filesystem_demo.lp`
- **Tests:** collections, OOP, modules, filesystem, package manager integration tests
- **Lexer:** delimiter-depth tracking for multiline collection literals
- **Windows install** documented in README

### Changed

- Workspace version bumped to **0.2.0**
- `AssignTarget::Name` AST node now supports optional type annotations
- Brace expressions disambiguate Dict vs Set via colon
- Manual chapter 19 (Collections) rewritten for implemented features
- Language Reference expanded with collections and filesystem sections
- Stub examples (`agent.lp`, `browser.lp`, `server.lp`) rewritten to run on v0.2

### Fixed

- Dict literal keys: identifier keys treated as labels, not variable references
- Index assignment parsing (`nums[1] = 20.`)
- Mixed-type list validation against `List<T>` annotations
- `write_bytes` and `append` statement kinds honored at runtime
- Multiline `{ }` and `[ ]` literals inside expressions

### Removed

- Nothing in this release

### Known issues

- Interfaces, properties, and abstract types are spec-only
- `navigator`, `ai`, `network`, `database` modules are stubs
- No native code generation; interpreter-only execution
- `lang publish` and `lang login` not connected to remote registry
- Project `.lp` module files are located but not fully evaluated as separate units
- LSP diagnostics disabled; extension uses `lang check`

### Future work

See [ROADMAP.md](ROADMAP.md).

---

## [0.1.0] - 2026-07-14

### Added

- Initial public release: lexer, parser, AST, semantic analyzer, interpreter
- Grammar Freeze v1.0
- Functions, control flow, strings, basic I/O
- VS Code / Cursor extension with syntax highlighting and diagnostics
- `lang`, `langc`, `lang-lsp` binaries
- Install scripts for macOS, Linux, and Windows
- 25-chapter manual and language specification

[0.2.0]: https://github.com/Nagashreeshyl/langp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Nagashreeshyl/langp/releases/tag/v0.1.0
