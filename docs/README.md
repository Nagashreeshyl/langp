# Documentation

Language specification, guides, and reference material for Lang.P.

## Start learning

**[The Lang.P Book](book/README.md)** is the **primary learning resource** for Lang.P v0.2.0 — a full tutorial from installation through complete projects, written for readers with zero prior programming experience.

→ [Open The Lang.P Book](book/README.md)

---

## Contents

| Path | Description |
|------|-------------|
| **[`book/`](book/)** | **The Lang.P Book** — official 38-chapter tutorial (beginner → advanced) |
| **[`manual/`](manual/)** | Lang.P Manual — 25-chapter professional guide |
| [`spec/GRAMMAR-FREEZE-v1.md`](spec/GRAMMAR-FREEZE-v1.md) | **Grammar Freeze v1.0** — official syntax (single source of truth) |
| [`spec/KEYWORDS.md`](spec/KEYWORDS.md) | Reserved keywords reference |
| [`spec/DESIGN-DECISIONS.md`](spec/DESIGN-DECISIONS.md) | Why Lang.P syntax works this way |
| [`spec/LANGUAGE-PHILOSOPHY.md`](spec/LANGUAGE-PHILOSOPHY.md) | Language design principles |
| [`manual/README.md`](manual/README.md) | Manual index with implementation status badges |
| [`guides/HOW-TO-CODE.md`](guides/HOW-TO-CODE.md) | Hands-on beginner tutorial |
| [`guides/LANGUAGE-REFERENCE.md`](guides/LANGUAGE-REFERENCE.md) | **All commands & functions implemented in v0.2** |
| [`../STATUS.md`](../STATUS.md) | Public feature stability matrix |
| [`../CHANGELOG.md`](../CHANGELOG.md) | Release history |
| [`../CONTRIBUTING.md`](../CONTRIBUTING.md) | How to contribute |
| [`TECH-STACK.md`](TECH-STACK.md) | **Full tech stack** — architecture, crates, IDE, CI, and roadmap |
| [`ARCHITECTURE-v0.2.md`](ARCHITECTURE-v0.2.md) | **v0.2 architecture** — OOP, modules, package manager |
| [`spec/`](spec/) | Authoritative language specification (22 chapters) |
| [`spec/LANGP-SPEC.md`](spec/LANGP-SPEC.md) | Single-document specification for offline reading |
| [`spec/glossary.md`](spec/glossary.md) | Term definitions |
| [`grammar/`](grammar/) | Formal EBNF grammar (Phase 2) |
| [`grammar/LANGP-EBNF.md`](grammar/LANGP-EBNF.md) | Combined grammar reference |

## Specification

The language specification is the foundation of the Lang.P project. All compiler, runtime, stdlib, and tooling implementations MUST conform to it.

- **Version:** 0.2.0
- **Status:** Phase 1 complete; v0.2 runtime features in beta
- **Grammar:** Phase 2 complete — [`grammar/README.md`](grammar/README.md)
- **Index:** [`spec/README.md`](spec/README.md)

## Future Additions

| Path | Phase | Description |
|------|-------|-------------|
| `guides/` | 7+ | Tutorials and how-to guides (**HOW-TO-CODE**, **LANGUAGE-REFERENCE**) |
| `grammar/` | 2 | Formal EBNF grammar (**complete**) |
| `migrations/` | 1.0+ | Version migration guides |
| `api/` | 7+ | Standard library API reference |

## Contributing to Docs

Spec changes follow the amendment process in [Chapter 22](spec/22-compatibility-versioning.md):

1. Propose change with rationale
2. Update affected chapters
3. Bump spec version
4. Add conformance tests
