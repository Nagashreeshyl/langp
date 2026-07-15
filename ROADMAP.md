# Lang.P Roadmap

This roadmap describes planned work after **v0.2.0**. Dates are targets, not commitments. The [grammar remains frozen](docs/spec/GRAMMAR-FREEZE-v1.md) — new features use existing syntax.

---

## v0.2.x — Stabilization (current)

**Theme:** Harden v0.2 foundations.

- Bug fixes from community reports
- Complete project `.lp` module evaluation
- Package manager: git clone install path
- Documentation alignment
- Extension polish

---

## v0.3 — Type system and collections

**Theme:** Stronger static guarantees.

- Generic runtime specialization (where feasible)
- Interface structural checking
- Property getters/setters
- Collection performance improvements
- `match` expression runtime
- Expanded semantic warnings (unused imports, dead code)

---

## v0.4 — Frameworks

**Theme:** Navigator and AI.

- **Navigator** — embedded browser, tabs, events
- **AI framework** — Assistant, providers, `on user.message`
- **Network** — HTTP client and server runtime
- **Database** — driver module stubs to real connectors

---

## v0.5 — Developer experience

**Theme:** Professional tooling.

- **Language Server** — unified diagnostics, completions, rename
- **Formatter** (`langfmt`) — canonical style enforcement
- **Debugger** — DAP integration
- **REPL** (`lang-repl`)
- Lang Studio desktop shell (optional)

---

## v1.0 — Production release

**Theme:** Compile, ship, registry.

- **Production compiler** — native codegen (LLVM or equivalent)
- **Official IDE** — Lang Studio 1.0
- **Remote package registry** — `lang publish`, `lang search` live
- **Stable runtime ABI**
- Long-term support policy

---

## How to influence the roadmap

1. Read [STATUS.md](STATUS.md) and [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md)
2. Open a [GitHub issue](https://github.com/Nagashreeshyl/langp/issues) with use case and priority
3. Submit a PR following [CONTRIBUTING.md](CONTRIBUTING.md)

Features that require **grammar changes** need a [Grammar Freeze amendment](docs/spec/22-compatibility-versioning.md) and will not land before v1.0 without explicit version bump.
