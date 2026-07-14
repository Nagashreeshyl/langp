# 01 — Introduction

**Status: Implemented (v0.1)**

---

## What is Lang.P?

**Lang.P** (spoken name: **Lang**) is a programming language designed to read like clear English instructions while remaining powerful enough for applications, scripts, tools, and (in future releases) browsers, servers, and AI agents.

Programs are saved as `.lp` files and run with:

```bash
lang run hello.lp
```

---

## Why Lang.P exists

Most languages force beginners to learn two languages at once: the programming language and the syntax rules around it (`{}`, `;`, `#`, `//`). Lang.P reduces that burden:

- **Sentences end with `.`** — like written instructions
- **Blocks open with `,` and close with `..`** — visually obvious structure
- **Comments use `@`** — reads as “note” or “annotation”
- **Text is joined with `with`** — not overloaded `+` operators

The goal is a language where a beginner can read most code aloud and understand it, while experienced developers still get functions, types, modules, and systems programming in later releases.

---

## How Lang.P differs from Python

| Topic | Python | Lang.P |
|-------|--------|--------|
| Comments | `# comment` | `@ comment` |
| Statement end | Newline (or `;`) | `.` always |
| Block start | `:` at end of header | `,` at end of header |
| Block end | Indentation only | `..` on dedented line |
| String join | `"a" + b` or f-strings | `"a" with b` |
| Else branch | `elif` / `else:` | `otherwise if` / `otherwise` |
| Run command | `python file.py` | `lang run file.lp` |

Lang.P is **not** a Python clone. It borrows readable assignment and indentation, but punctuation and control-flow keywords are intentionally different.

---

## Your first program

### Learning version

```lp
@ Say hello to the world.
print "Hello, Lang.P!".
```

### Professional version

```lp
print "Hello, Lang.P!".
```

Run:

```bash
lang run hello.lp
```

---

## What works in v0.1

Today you can write real programs with:

- Variables, functions, `print`, `input`
- Math (`+`, `-`, `*`, `/`) and comparisons
- `if` / `otherwise if` / `otherwise`
- `repeat`, `while`, `for`, `try` / `catch`
- Builtins: `len`, `to_string`, `assert`
- Basic file read/write and file management

See [Language Reference v0.1](../guides/LANGUAGE-REFERENCE.md) for the complete list.

---

## What is specified but not yet fully implemented

The [full specification](../spec/LANGP-SPEC.md) describes modules (`use`), classes (`type`), HTTP (`get`, `post`), the Navigator browser framework, the AI framework, and more. These appear in later manual chapters marked **Specification** or **Planned**.

---

## Next steps

- [02 — Language Philosophy](02-philosophy.md)
- [How to Code in Lang.P](../guides/HOW-TO-CODE.md)
- [examples/hello.lp](../../examples/hello.lp)
