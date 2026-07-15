# The Lang.P Book

**Official learning guide for Lang.P v0.2.0**

Welcome to *The Lang.P Book* — the primary resource for learning Lang.P from absolute beginner to confident application developer. Every chapter documents **features that work today** in the interpreter. Experimental features are clearly marked. Planned features appear only in [37 — Future Roadmap](37-future-roadmap.md).

---

## How to use this book

1. Read chapters **in order** if you are new to programming.
2. Each chapter follows the same structure: introduction, syntax, examples, mistakes, best practices, exercises, summary.
3. Run every example with `lang run yourfile.lp` or paste into a `.lp` file.
4. Use [34 — Language Reference](34-language-reference.md) as a lookup while coding.
5. Build real programs in [35 — Complete Projects](35-complete-projects.md).

**Prerequisites:** None. This book assumes zero programming experience.

---

## Part I — Getting started

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 00 | [Preface](00-preface.md) | Why Lang.P exists and how this book is organized |
| 01 | [Installation](01-installation.md) | Install `lang`, IDE extension, verify setup |
| 02 | [Your First Program](02-your-first-program.md) | Write, run, and check a `.lp` file |
| 03 | [Language Basics](03-language-basics.md) | Statements, blocks, punctuation rules |
| 04 | [Comments](04-comments.md) | Document code with `@` |

## Part II — Core language

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 05 | [Variables](05-variables.md) | Names, assignment, scope |
| 06 | [Data Types](06-data-types.md) | Int, Float, String, Bool, Null |
| 07 | [Operators](07-operators.md) | Math, comparison, logic |
| 08 | [Input](08-input.md) | Read from the user |
| 09 | [Output](09-output.md) | Print to the terminal |
| 10 | [Strings](10-strings.md) | Text and the `with` operator |
| 11 | [Functions](11-functions.md) | Reusable blocks of code |
| 12 | [Conditionals](12-conditionals.md) | `if`, `otherwise if`, `otherwise` |
| 13 | [Loops](13-loops.md) | `repeat`, `while`, `for` |

## Part III — Collections

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 14 | [Collections Overview](14-collections.md) | Lists, dicts, sets, tuples compared |
| 15 | [Dictionaries](15-dictionaries.md) | Key-value maps |
| 16 | [Sets](16-sets.md) | Unique values and set algebra |
| 17 | [Tuples](17-tuples.md) | Fixed immutable groups |

## Part IV — Object-oriented programming *(Experimental: Beta)*

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 18 | [Types and OOP](18-type-oop.md) | Why objects exist in Lang.P |
| 19 | [Objects](19-objects.md) | Fields and instances |
| 20 | [Constructors](20-constructors.md) | The `init` method |
| 21 | [Methods](21-methods.md) | Behavior on objects |
| 22 | [Inheritance](22-inheritance.md) | `extends` and overriding |
| 23 | [Static Members](23-static-members.md) | Shared type-level data *(partial)* |

## Part V — Modules and projects

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 24 | [Modules](24-modules.md) | Standard library modules |
| 25 | [Imports](25-imports.md) | `use` and dot access |
| 26 | [Filesystem](26-filesystem.md) | Files, paths, I/O |
| 27 | [Error Handling](27-error-handling.md) | `try`, `catch`, `finally` |
| 28 | [Package Manager](28-package-manager.md) | `lang init`, `install`, lock file |
| 29 | [Project Structure](29-project-structure.md) | Layout of a Lang.P app |

## Part VI — Professional development

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 30 | [Debugging](30-debugging.md) | Find and fix problems |
| 31 | [Testing](31-testing.md) | Verify programs work |
| 32 | [Best Practices](32-best-practices.md) | Write maintainable code |
| 33 | [Common Mistakes](33-common-mistakes.md) | Errors beginners make |

## Part VII — Reference and projects

| # | Chapter | What you will learn |
|---|---------|---------------------|
| 34 | [Language Reference](34-language-reference.md) | Keywords, builtins, CLI, errors |
| 35 | [Complete Projects](35-complete-projects.md) | Build apps step by step |
| 36 | [FAQ](36-faq.md) | Frequently asked questions |
| 37 | [Future Roadmap](37-future-roadmap.md) | Planned features (not in main chapters) |

---

## Quick links

- [Grammar Freeze v1.0](../spec/GRAMMAR-FREEZE-v1.md) — official syntax rules
- [STATUS.md](../../STATUS.md) — feature stability matrix
- [examples/](../../examples/) — runnable sample programs
- [Contributing](../../CONTRIBUTING.md) — improve Lang.P and this book

---

*The Lang.P Book is maintained alongside Lang.P releases. Report documentation issues on GitHub.*
