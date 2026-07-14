# 02 — Language Philosophy

**Status: Implemented (v0.1)**

---

## Readability first

Lang.P code should read as a sequence of instructions a human can follow:

```lp
@ Ask for the user's age and classify them.
age = input number "Enter your age: ".

if age >= 18,
    print "Adult".
otherwise,
    print "Minor".
..
```

Design choices follow one question: *Can a beginner read this without memorizing special symbols?*

---

## Beginner-first, not beginner-only

Lang.P starts simple but does not cap complexity:

- **v0.1** — scripts, input/output, control flow, files
- **Future** — types, modules, async, networking, UI, AI

Syntax stays stable as features grow. New capabilities add keywords and libraries; they do not replace the `.` / `,` / `..` model.

---

## Why `with` exists (not `+` for text)

In many languages, `+` means both addition **and** string concatenation. That overload confuses beginners:

```lp
@ Lang.P — text uses with
greeting = "Hello " with name with "!".

@ Lang.P — numbers use +
total = price + tax.
```

**Rule:** Use `with` to compose strings. Use `+ - * /` for numbers.

---

## The three punctuation marks

### `.` — end of statement

Every instruction is a sentence. It ends with a period:

```lp
name = "Naga".
print name.
```

Missing `.` is one of the most common errors. The compiler reports `expected StmtEnd`.

### `,` — open a block

After block headers (`if`, `repeat`, `function`, …), a comma opens the indented body:

```lp
repeat 3 times,
    print "Hello".
..
```

### `..` — close every block

**Grammar Freeze v1.0:** All blocks — `if`, `repeat`, `function`, `type`, `enum`, `try`, `on` — close with `..`:

```lp
function greet(name),
    print "Hello " with name.
..

type Point,
    x.
    y.
..
```

| Construct | Opens with | Closes with |
|-----------|------------|-------------|
| **Every block** | `,` | `..` |
| **Every statement** | — | `.` |

Official reference: [GRAMMAR-FREEZE-v1.md](../spec/GRAMMAR-FREEZE-v1.md)

---

## Comments with `@`

```lp
@ This explains the next line for learners.
@ Professional codebases use @ sparingly — for non-obvious intent only.
factor = 1.05.
```

There is no `#` or `//` comment syntax in Lang.P source files.

---

## Explicit over implicit

Lang.P prefers clear keywords over punctuation shortcuts:

| Instead of | Lang.P uses |
|------------|-------------|
| `else if` | `otherwise if` |
| `else` | `otherwise` |
| `def` | `function` |
| `True` / `False` | `true` / `false` |

---

## Next steps

- [03 — Operators](03-operators.md)
- [Lexical structure (spec)](../spec/02-lexical-structure.md)
