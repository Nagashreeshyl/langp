# Lang.P Project Status

**Version:** 0.2.0  
**Release date:** 2026-07-15  
**Grammar:** [Frozen v1.0](docs/spec/GRAMMAR-FREEZE-v1.md)

This document is the public-facing feature matrix for Lang.P v0.2.0. Status labels match how Rust, Go, and Python mark stability in release notes.

| Label | Meaning |
|-------|---------|
| ✅ **Stable** | Works in the current release; covered by tests; documented |
| 🟡 **Beta** | Usable but incomplete; APIs may change |
| 🚧 **In Progress** | Partial implementation; not production-ready |
| 📋 **Planned** | Specified or designed; not implemented |

---

## Core language

| Feature | Status | Notes |
|---------|--------|-------|
| Grammar (frozen) | ✅ Stable | `.` `,` `..` — no exceptions |
| Lexer | ✅ Stable | Indentation, collections, block close |
| Parser | ✅ Stable | Conformance fixtures in `tests/conformance/` |
| Semantic analyzer | ✅ Stable | Names, types, assignments |
| Tree-walking interpreter | ✅ Stable | Primary execution engine |
| Native compiler | 📋 Planned | `langc build` emits launcher scripts only |

---

## Language features

| Feature | Status | Notes |
|---------|--------|-------|
| Variables & assignment | ✅ Stable | Typed annotations (`x: Int = 1.`) |
| Functions | ✅ Stable | `function name(),` … `..` |
| Control flow (if/repeat/for/while) | ✅ Stable | |
| Strings (`with`) | ✅ Stable | |
| Input / output | ✅ Stable | Text/number/boolean; pickers need GUI |
| Error handling (try/catch) | ✅ Stable | |
| Collections (List/Dict/Set/Tuple) | ✅ Stable | Methods, generics validation |
| Objects (`type`) | 🟡 Beta | Fields, methods, `init`, `extends` |
| Modules (`use`) | 🟡 Beta | Stdlib stubs; project `.lp` eval pending |
| Generics | 🟡 Beta | Semantic validation; no runtime specialization |
| Interfaces | 📋 Planned | Spec only |
| Properties | 📋 Planned | Spec only |
| Events (`on`) | 🚧 In Progress | Parsed; runtime limited |
| Async | 📋 Planned | Spec only |

---

## Standard library & tooling

| Feature | Status | Notes |
|---------|--------|-------|
| Filesystem (statements) | ✅ Stable | `read`, `write`, `copy`, `move`, `delete` |
| Filesystem (module) | ✅ Stable | `use filesystem.` — `exists`, `list`, etc. |
| Math module | 🟡 Beta | `abs`, `min`, `max` |
| JSON module | 🟡 Beta | Stub `parse`/`stringify` |
| Navigator | 📋 Planned | Module stub only |
| AI framework | 📋 Planned | Module stub only |
| Network / HTTP | 📋 Planned | Module stub only |
| Database | 📋 Planned | Module stub only |
| Package manager (`lang`) | 🟡 Beta | `init`, `install`, `lock`, offline registry |
| Language Server | 🟡 Beta | Hover/go-to-def; diagnostics via extension |
| VS Code / Cursor extension | ✅ Stable | Syntax, check, autocomplete |
| Formatter (`lang fmt`) | 🚧 In Progress | Delegates to `lang check` |
| Debugger | 📋 Planned | |
| Lang Studio (desktop IDE) | 📋 Planned | VS Code extension ships today |

---

## Platforms

| Platform | Status |
|----------|--------|
| macOS (x64, ARM64) | ✅ Stable |
| Linux (x64, ARM64) | ✅ Stable |
| Windows (x64) | ✅ Stable |

---

## Documentation

| Asset | Status |
|-------|--------|
| **The Lang.P Book** | ✅ Published (`docs/book/`) |
| README | ✅ Updated for v0.2 |
| Manual (25 chapters) | ✅ Maintained |
| Language Reference | ✅ Updated |
| Specification | ✅ Phase 1–2 complete |
| Architecture docs | ✅ v0.2 architecture published |

---

## Quick links

- [Known Limitations](KNOWN_LIMITATIONS.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [How to install](README.md#install-one-line)
