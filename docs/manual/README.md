# Lang.P Manual

Professional documentation for the Lang.P programming language (`.lp` files).

This manual is organized like [The Python Tutorial](https://docs.python.org/3/tutorial/) and [The Rust Book](https://doc.rust-lang.org/book/): short chapters, progressive examples, and a clear split between **what works today** and **what the language specification defines for future releases**.

---

## How to read this manual

| Badge | Meaning |
|-------|---------|
| **Implemented (v0.2)** | Works in the current interpreter — run with `lang run file.lp` |
| **Beta (v0.2)** | Partial runtime; usable with documented limits |
| **Specification** | Defined in spec; parser may accept syntax; runtime incomplete |
| **Planned** | Design documented; not yet available |

**Start here if you are new:** [01 — Introduction](01-introduction.md) → [02 — Language Philosophy](02-philosophy.md) → [How to Code in Lang.P](../guides/HOW-TO-CODE.md)

**Quick lookup for working features:** [Language Reference v0.2](../guides/LANGUAGE-REFERENCE.md)

**Normative specification:** [Specification Index](../spec/README.md)

---

## Manual contents

| # | Chapter | Status |
|---|---------|--------|
| 01 | [Introduction](01-introduction.md) | Implemented |
| 02 | [Language Philosophy](02-philosophy.md) | Implemented |
| 03 | [Operators](03-operators.md) | Implemented |
| 04 | [Data Types](04-datatypes.md) | Implemented |
| 05 | [Strings](05-strings.md) | Implemented |
| 06 | [Variables](06-variables.md) | Implemented |
| 07 | [Functions](07-functions.md) | Implemented |
| 08 | [Loops](08-loops.md) | Implemented |
| 09 | [Conditions](09-conditions.md) | Implemented |
| 10 | [Input](10-input.md) | Partial |
| 11 | [Output](11-output.md) | Implemented |
| 12 | [Classes (`type`)](12-classes.md) | Beta |
| 13 | [Modules](13-modules.md) | Beta |
| 14 | [Error Handling](14-error-handling.md) | Implemented |
| 15 | [Async Programming](15-async.md) | Specification |
| 16 | [File System](16-filesystem.md) | Implemented |
| 17 | [Networking](17-networking.md) | Specification |
| 18 | [JSON](18-json.md) | Specification |
| 19 | [Collections](19-collections.md) | Implemented |
| 20 | [Standard Library](20-standard-library.md) | Partial |
| 21 | [Navigator Framework](21-navigator.md) | Specification |
| 22 | [AI Framework](22-ai-framework.md) | Specification |
| 23 | [Best Practices](23-best-practices.md) | Implemented |
| 24 | [Common Mistakes](24-common-mistakes.md) | Implemented |
| 25 | [Error Messages](25-error-messages.md) | Implemented |

---

## Official punctuation rules

These rules are **final** and used consistently across this manual:

| Symbol | Role |
|--------|------|
| `@` | Comment (start of line or end of line) |
| `.` | Ends every **statement** |
| `,` | Opens **every block** |
| `..` | Closes **every block** — no exceptions |

**Grammar Freeze v1.0:** There is only one block rule. Blocks never close with a lone `.` See [GRAMMAR-FREEZE-v1.md](../spec/GRAMMAR-FREEZE-v1.md).

---

## Contributing

When adding a language feature:

1. Implement it in the interpreter
2. Add it to `editors/langp-manifest.json`
3. Update [LANGUAGE-REFERENCE.md](../guides/LANGUAGE-REFERENCE.md)
4. Update the relevant manual chapter and change its status badge

Spec changes follow [Chapter 22](../spec/22-compatibility-versioning.md).
