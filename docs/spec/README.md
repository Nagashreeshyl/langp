# Lang.P Language Specification

**Version:** 0.1.0  
**Status:** Phase 1 Complete  
**Last updated:** 2026-07-14

This directory contains the authoritative specification for the Lang.P programming language. Implementations of `langc`, the runtime, standard library, and tooling **must** conform to this document.

## How to Read This Spec

The specification is organized into chapters. Each chapter is self-contained but cross-referenced. Normative keywords use [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119) semantics:

- **MUST**, **MUST NOT** — absolute requirements
- **SHOULD**, **SHOULD NOT** — recommended but not mandatory
- **MAY** — optional

Code examples marked `Example` are illustrative. Code marked `Conformance` are required behaviors.

## Chapters

| # | Chapter | File |
|---|---------|------|
| 1 | [Introduction](01-introduction.md) | Philosophy, goals, design principles |
| 2 | [Lexical Structure](02-lexical-structure.md) | Comments, tokens, identifiers, literals |
| 3 | [Program Structure](03-program-structure.md) | Files, modules, entry points |
| 4 | [Types](04-types.md) | Type system, inference, input types, generics |
| 5 | [Variables & Assignment](05-variables-assignment.md) | Bindings, mutability, scope |
| 6 | [Expressions](06-expressions.md) | Operators, `with`, `input`, precedence |
| 7 | [Statements](07-statements.md) | Statement grammar, blocks, termination |
| 8 | [Functions](08-functions.md) | Definition, calls, closures, overloading |
| 9 | [Control Flow](09-control-flow.md) | `if`, loops, pattern matching |
| 10 | [Object Model](10-object-model.md) | `type`, inheritance, interfaces, extensions |
| 11 | [Modules & Imports](11-modules-imports.md) | `use`, packages, visibility |
| 12 | [Events](12-events.md) | Event-driven programming with `on` |
| 13 | [Error Handling](13-error-handling.md) | `try` / `catch` / `finally` |
| 14 | [Concurrency & Async](14-concurrency-async.md) | `wait for`, tasks, synchronization |
| 15 | [I/O & Network](15-io-network.md) | Filesystem, HTTP, streams, standard I/O |
| 16 | [Standard Library](16-standard-library.md) | Core modules overview |
| 17 | [Navigator Framework](17-navigator.md) | Browser & desktop UI framework |
| 18 | [AI Framework](18-ai-framework.md) | LLM providers, agents, RAG |
| 19 | [Runtime & Memory](19-runtime-memory.md) | GC, calling conventions, ABI |
| 20 | [Package System](20-package-system.md) | `lang` package manager |
| 21 | [Tooling](21-tooling.md) | `langc`, Lang LSP, Lang Studio, input quick-fixes |
| 22 | [Compatibility & Versioning](22-compatibility-versioning.md) | Semver, deprecation policy |
| — | [Glossary](glossary.md) | Term definitions |

## Single-Document View

For offline reading or printing, all chapters are concatenated in [LANGP-SPEC.md](LANGP-SPEC.md).

## Conformance

An implementation is **Lang.P 0.1 conformant** if it passes all tests in `/tests/conformance/` and satisfies every **MUST** requirement in this specification.

The formal grammar is defined in [`docs/grammar/`](../grammar/).

## Change Process

1. Propose a change as a spec amendment with rationale and migration notes.
2. Review against philosophy: does it improve readability without adding unnecessary keywords?
3. Update affected chapters, examples, and conformance tests.
4. Bump spec version according to [Chapter 22](22-compatibility-versioning.md).
