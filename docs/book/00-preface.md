# Preface

## Introduction

Programming languages are tools for telling a computer what to do. Most languages were designed for experts first and beginners second. **Lang.P** (spoken name: **Lang**) was designed the other way around: code should read like clear instructions, and the rules should be easy to remember.

*The Lang.P Book* teaches you programming **and** Lang.P at the same time. You do not need prior experience with Python, JavaScript, C, or any other language. If you can read English and follow step-by-step instructions, you can learn Lang.P.

### Why this book exists

Official language documentation should feel like a real book — not a scattered collection of README files. This book is modeled after quality resources such as [*The Rust Programming Language*](https://doc.rust-lang.org/book/), [*A Tour of Go*](https://go.dev/tour/), and the [Python Tutorial](https://docs.python.org/3/tutorial/).

Every chapter covers **only what works in Lang.P v0.2.0**. Features that are experimental are labeled **Experimental (Beta)**. Features that are planned but not built yet appear in [37 — Future Roadmap](37-future-roadmap.md), not in the main teaching path.

### When to use this book

| You want to… | Start here |
|--------------|------------|
| Learn Lang.P from zero | [01 — Installation](01-installation.md) |
| Look up syntax quickly | [34 — Language Reference](34-language-reference.md) |
| Build a complete app | [35 — Complete Projects](35-complete-projects.md) |
| Understand what is stable | [STATUS.md](../../STATUS.md) |

---

## Syntax

This book uses the same punctuation Lang.P uses:

| Symbol | Meaning |
|--------|---------|
| `.` | End of statement |
| `,` | Start of block (after `if`, `function`, etc.) |
| `..` | End of block |
| `@` | Comment |

Code blocks are labeled **Learning version** (with `@` comments) and **Professional version** (production style).

---

## Examples

### Learning version

```lp
@ Greet the reader.
print "Welcome to The Lang.P Book.".
```

### Professional version

```lp
print "Welcome to The Lang.P Book.".
```

---

## Common mistakes

**Mistake:** Skipping chapters because they look familiar from another language.

**Why:** Lang.P looks like Python in places but uses different punctuation (`..` not `:`), different keywords (`otherwise` not `else`), and `with` instead of `+` for strings.

**Correct approach:** Read [03 — Language Basics](03-language-basics.md) even if you have experience elsewhere.

---

## Best practices

- Type every example yourself instead of only reading it.
- Run `lang check file.lp` before `lang run file.lp` while learning.
- Keep a `practice/` folder for exercise solutions.
- Read error messages carefully — Lang.P explains what went wrong.

---

## Exercises

### Beginner

1. Install Lang.P using [01 — Installation](01-installation.md) and run `lang --version`.
2. Open this book's first code example in a file `welcome.lp` and run it.
3. Change the message to include your name using `with`.
4. Add an `@` comment explaining what the program does.
5. Run `lang check welcome.lp` and confirm zero errors.

### Intermediate

1. Compare Lang.P punctuation to a language you know; write a one-page summary.
2. Find three examples in [examples/](../../examples/) and explain each line in plain English.
3. Read [Grammar Freeze v1.0](../spec/GRAMMAR-FREEZE-v1.md) sections 1–4.
4. List five features marked Beta in [STATUS.md](../../STATUS.md).
5. Explain the difference between this book and [34 — Language Reference](34-language-reference.md).

### Advanced

1. Write a learning plan: which chapters you will finish each week for one month.
2. Contribute a typo fix or example improvement via [CONTRIBUTING.md](../../CONTRIBUTING.md).

---

## Summary

You learned what *The Lang.P Book* is, who it is for, and how it is organized. Lang.P prioritizes readability; this book prioritizes honesty about what is implemented today.

**Next:** [01 — Installation](01-installation.md)
