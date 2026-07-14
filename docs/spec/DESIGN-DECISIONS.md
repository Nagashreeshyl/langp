# Lang.P Design Decisions

**Status:** Official reference for contributors  
**Grammar version:** 1.0 (see [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md))

This document records **why** Lang.P syntax works the way it does. Read it before proposing syntax changes.

---

## Why `.` ends statements

Human instructions end with a period. Lang.P treats each line of code as a sentence:

```lp
name = "Naga".
print name.
```

**Benefits:**

- Visual rhythm: one instruction, one full stop
- Clear parse boundaries for beginners and tooling
- No semicolon culture (`;`) borrowed from C/Java

**What `.` does NOT do:** close blocks. That is `..`'s job exclusively (Grammar Freeze v1.0).

---

## Why `,` opens blocks

A comma suggests *continuation* — "more follows below":

```lp
if ready,
    start().
..
```

**Benefits:**

- Reads naturally: "If ready, then …"
- Distinct from statement-ending `.`
- Works for all block kinds with one rule (functions, loops, types, conditionals)

**Rejected alternatives:** `{` braces (C-style), `:` colons alone (Python — Lang.P uses `,` after the header instead).

---

## Why `..` closes blocks

A double period marks the **end of a paragraph** of instructions — the block is complete:

```lp
function greet(name),
    print "Hello " with name.
..
```

**Grammar Freeze v1.0 decision:** There is **only one** block close form. Previously some docs used `.` to close `function` and `type` blocks. That created two rules and confused beginners ("Is this `.` a statement or a block?"). Unified rule: **all blocks close with `..`**.

**Benefits:**

- One rule to teach, one rule for the parser, one rule for the IDE de-indent
- `..` cannot be mistaken for a float literal or statement end
- Auto-indent in editors triggers on `,` and de-indents on `..`

---

## Why `with` replaces string `+`

In Python and JavaScript, `+` means both addition and string concatenation:

```lp
@ Lang.P — unambiguous
message = "Hello " with name.
total = price + tax.
```

**Benefits:**

- Beginners never wonder "does `+` add numbers or glue text?"
- `with` reads as English: `print "Hello " with name`
- Arithmetic operators stay numeric only

---

## Why Python-style variable assignment

```lp
name = "Naga".
count += 1.
```

**Benefits:**

- Familiar to the largest beginner audience
- No `let` / `var` / `const` ceremony for first programs
- Typed variants can be added later without changing the shape

---

## Why functions use `function greet(name),` instead of braces

```lp
function greet(name),
    print "Hello " with name.
..
```

**Benefits:**

- Header reads as English: "function greet(name), …"
- No `{` `}` nesting noise
- Same block model as `if` and `repeat` — one grammar for everything
- Indentation carries structure (like Python), but blocks are explicitly opened/closed with `,` / `..`

**Rejected:** `function greet(name) { ... }` — adds a second block syntax.

---

## Why comments begin with `@`

```lp
@ Explain the next step for learners.
name = input text "Name: ".
```

**Benefits:**

- Visually distinct from code (not `#` or `//` shared with a dozen languages)
- Reads as "annotation" or "note to reader"
- Easy to syntax-highlight in soft gray without clashing with operators

**Rejected:** `#` (conflicts with future hex or shebang), `//` (C legacy).

---

## Why Lang.P avoids unnecessary punctuation

Lang.P deliberately limits sigils:

| Lang.P | Avoided |
|--------|---------|
| `with` | `$` interpolation, `%` formatting |
| `,` / `..` | `{` `}` `[` `]` for blocks |
| `function` | `fn`, `def`, `fun` |
| `otherwise` | `else` (reserved word collision with "else if" confusion) |

**Principle:** Every symbol should earn its place. If two symbols do the same job, remove one.

---

## Decision log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-07-15 | Grammar Freeze v1.0 — all blocks use `..` | Single block rule; eliminate declaration vs control-flow split |
| 2026-07-14 | `otherwise if` not `else if` | Two-token keyword; readable English |
| 2026-07-14 | `with` for strings | Disambiguate from numeric `+` |

---

## Before changing syntax

1. Read [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md)
2. Document rationale in this file
3. Follow [Chapter 22 — Compatibility](22-compatibility-versioning.md)
4. Update parser, tests, manual, and IDE snippets in the **same change**

Do not merge documentation-only syntax drift.
