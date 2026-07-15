# Known Limitations — Lang.P v0.2.0

This document lists honest limitations of the **current** release. If something is not listed here but fails at runtime, please [open an issue](https://github.com/Nagashreeshyl/langp/issues).

---

## Execution model

- **Interpreted only.** There is no native machine-code compiler. `langc build` generates a shell/batch launcher, not a standalone binary.
- **Single-process.** No multi-threaded runtime, no async/await execution.
- **No JIT.** Performance is suitable for learning and prototyping, not high-throughput production workloads.

---

## Object-oriented programming (beta)

| Limitation | Detail |
|------------|--------|
| Interfaces | Not enforced at runtime |
| Properties | Getter/setter syntax not implemented |
| Abstract types | Not implemented |
| Full generic specialization | Types checked semantically; runtime is dynamic |
| `super` calls | Parsed; limited runtime support |
| Visibility | `public`/`private`/`internal` parsed but not enforced |

---

## Modules (beta)

| Limitation | Detail |
|------------|--------|
| Project `.lp` files | Loader finds files but does not fully evaluate separate compilation units |
| Nested imports | `use http.server.` not supported |
| Re-exports | Not implemented |
| Wildcard imports | Forbidden by design (not a bug) |

Stdlib modules `navigator`, `ai`, `network`, and `database` are **stubs** returning a version string only.

---

## Package manager (beta)

| Limitation | Detail |
|------------|--------|
| Remote registry | Offline index only; no live `langp.dev` |
| `lang publish` / `lang login` | Not connected to a server |
| Git dependencies | Lock file records git URL; clone/install is minimal |
| Dependency graph | Single-level resolution; no workspace monorepos |

---

## Standard library gaps

| Area | Status |
|------|--------|
| HTTP (`get`, `post`, …) | Not implemented — returns error if used |
| JSON | Stub module; no real parser |
| Database drivers | Not implemented |
| GUI pickers (`input file`, etc.) | Require GUI runtime (not shipped) |
| Regex, datetime, crypto | Spec only |

---

## Parser and semantic analyzer

- **`match` expression** — specified; use `if`/`otherwise if` chains today.
- **`interface` declaration** — keyword reserved; parser support incomplete.
- **`lambda`** — parsed; interpreter returns not implemented.
- **Unused import warnings** — not emitted yet.

---

## IDE and LSP

- LSP diagnostics are **disabled** in the server; the VS Code extension runs `lang check` instead.
- LSP is **off by default** in the extension (v0.2.8+).
- No debugger integration.

---

## Platform

- Windows: PATH setup may require shell restart after install.
- Sandboxed CI environments may block file I/O examples writing to cwd.
- `input_demo.lp` requires interactive stdin (not suitable for unattended CI run).

---

## Documentation

- Chapters marked **Specification** in the manual describe future design; see [STATUS.md](STATUS.md) for what runs today.
- The full 22-chapter spec describes the long-term language; not every feature is implemented.

---

## Future work

See [ROADMAP.md](ROADMAP.md) for planned releases.
