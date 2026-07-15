# Future Roadmap

## Introduction

This appendix lists **planned and in-progress features that are not fully implemented in v0.2.0**. Main book chapters teach what works today. This chapter answers: "What is coming next?"

**Why separate roadmap from tutorials:** Documenting unfinished features in teaching chapters causes confusion and broken examples.

**When to read:** After you know current Lang.P ([34 — Language Reference](34-language-reference.md)). Before proposing language changes.

Official roadmap: [ROADMAP.md](../../ROADMAP.md). Stability matrix: [STATUS.md](../../STATUS.md).

---

## Syntax

No new syntax in this appendix — future syntax may be proposed in [spec/](../spec/) before implementation.

---

## Version 0.2.x — Bug fixes and polish

| Item | Status |
|------|--------|
| Parser and runtime bug fixes | Ongoing |
| Documentation improvements | Ongoing |
| `lang fmt` real formatter | 🚧 In Progress |
| Static member completion | 🚧 In Progress |
| Visibility enforcement (`public`/`private`) | 📋 Planned |
| Project `.lp` module loading | 🚧 In Progress |

---

## Version 0.3 — Type system and collections

| Feature | Description |
|---------|-------------|
| Stronger generics | Runtime specialization |
| Interfaces | `interface` declarations |
| Properties | Getter/setter syntax |
| Collection enhancements | Performance, more methods |
| `enum` types | Algebraic enums |

See [ROADMAP.md](../../ROADMAP.md) Version 0.3 section.

---

## Version 0.4 — Navigator and AI

| Feature | Description |
|---------|-------------|
| **Navigator** | Desktop browser / webview framework |
| **AI framework** | Assistants, streaming, tool use |
| Event handlers | Full `on` runtime |
| Network client | HTTP verbs beyond stubs |

Today:

```lp
use navigator.
print navigator.version.    @ "0.0.0-stub"

use ai.
print ai.version.           @ "0.0.0-stub"
```

These modules exist only as placeholders ([24 — Modules](24-modules.md)).

---

## Version 0.5 — Developer experience

| Feature | Description |
|---------|-------------|
| Language Server | Richer diagnostics, refactor |
| Formatter | `lang fmt` rewrite rules |
| Debugger | Breakpoints, step execution |
| Lang Studio | Dedicated desktop IDE |

Current LSP and extension are Beta ([STATUS.md](../../STATUS.md)).

---

## Version 1.0 — Production readiness

| Feature | Description |
|---------|-------------|
| Native compiler | Real binary output (not launcher scripts) |
| Official IDE | Polished Lang Studio or equivalent |
| Remote registry | `lang publish` / `lang login` live |
| Stable runtime | Semver guarantees for language and stdlib |

---

## Features in spec but not in v0.2

Do **not** use these in production programs yet:

| Feature | Notes |
|---------|-------|
| `throw` / custom exceptions | Use `try`/`catch` only |
| `async` / `await` | Not implemented |
| Lambdas `(x) => …` | Not implemented |
| HTTP `get` / `post` | Not implemented |
| GUI pickers (`input file`, `input date`, …) | Require GUI; limited |
| `interface` / `property` | Spec only |
| Remote package registry | Local/offline only |
| Full JSON parse/stringify | Stub module |

---

## Examples

### Simple — checking stub modules

```lp
use ai.
use navigator.
use network.

print ai.version.
print navigator.version.
print network.version.
```

All print stub version strings — not real AI or browser functionality.

### Intermediate — what future Navigator might look like *(pseudocode — does not run)*

The following illustrates direction only. **Do not paste into v0.2 expecting results.**

```lp
@ FUTURE — not valid v0.2 program
@ browser = Browser().
@ browser.open("https://example.com").
@ on browser.load,
@     print "Page loaded.".
@ ..
```

### Advanced — tracking roadmap

Read [CHANGELOG.md](../../CHANGELOG.md) when upgrading. Compare [STATUS.md](../../STATUS.md) before and after each release.

---

## Common mistakes

**Mistake:** Copying spec examples from `docs/spec/` into v0.2 programs.

**Why:** Spec describes the full language vision; interpreter may lag.

**Fix:** Use this book and [34 — Language Reference](34-language-reference.md) for runnable code.

---

**Mistake:** Assuming `lang publish` works because the command exists.

**Why:** CLI stub returns "not yet connected".

**Fix:** Use local projects and `lang init` until registry ships.

---

## Best practices

- Watch GitHub releases and CHANGELOG for implemented roadmap items.
- Propose features via spec amendment process ([spec/22-compatibility-versioning.md](../spec/22-compatibility-versioning.md)).
- Label experimental code in your repos when using Beta features.

---

## Exercises

### Beginner

1. List three features marked 📋 Planned in STATUS.md.
2. Run stub modules and record all version strings.
3. Read ROADMAP Version 0.3 — name two features.
4. Explain why Mini Browser is a stub in [35 — Complete Projects](35-complete-projects.md).
5. Find one spec chapter for a feature not yet implemented.

### Intermediate

1. Write a table: Feature | Spec chapter | STATUS label.
2. Compare ROADMAP.md to this appendix — note differences.
3. Identify which v0.5 feature would help you most as a developer.
4. Read KNOWN_LIMITATIONS — link each item to a roadmap version.
5. Draft a v0.3 release note bullet list (hypothetical).

### Advanced

1. Propose a minimal viable Navigator API for v0.4 (design doc in comments).
2. Write a migration guide outline: v0.2 → v0.3 for OOP users.
3. Contribute a roadmap clarification PR if anything here is outdated.

---

## Summary

Future work includes interfaces, Navigator, AI, native compilation, and production tooling. v0.2 delivers a solid readable core: variables, functions, control flow, collections, beta objects and modules, and filesystem I/O. Learn what exists today; return here to see what comes next.

**Previous:** [36 — FAQ](36-faq.md) · **Next:** [README — Book Index](README.md)

**See also:** [ROADMAP.md](../../ROADMAP.md), [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md), [CHANGELOG.md](../../CHANGELOG.md)
