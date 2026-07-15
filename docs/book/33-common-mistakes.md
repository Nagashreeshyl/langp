# Common Mistakes

## Introduction

Every Lang.P beginner hits the same errors — missing periods, wrong block closers, string concatenation with `+`, and variables used outside their scope. This chapter collects the **most frequent mistakes** with wrong/right pairs so you can fix them quickly.

**Why a mistakes chapter:** Error messages like **E0200** and **E0202** make more sense when you recognize the pattern ([30 — Debugging](30-debugging.md)).

**When to read it:** When `lang check` or `lang run` fails and the fix is not obvious — or proactively before your first project ([29 — Project Structure](29-project-structure.md)).

---

## Syntax

Lang.P mistakes usually fall into four categories:

| Category | Symptom | Typical code |
|----------|---------|--------------|
| Punctuation | `expected StmtEnd`, E0200 | Missing `.` |
| Blocks | `expected ..`, E0201 | `.` instead of `..` |
| Strings | Type/runtime confusion | `"a" + "b"` |
| Names | `undefined name`, E0202 | Typo or scope error |

There is no special syntax for mistakes — they are violations of normal rules from [03 — Language Basics](03-language-basics.md).

---

## Examples

### Simple — missing period

**Learning version (wrong):**

```lp
print "Hello"
```

**Learning version (right):**

```lp
print "Hello".
```

**Professional version:** Always terminate statements with `.` — error **E0200** / **E0202**.

---

### Intermediate — wrong block close

**Learning version (wrong):**

```lp
if true,
    print "Yes".
.    @ single dot — wrong
```

**Learning version (right):**

```lp
if true,
    print "Yes".
..    @ double dot closes entire if chain
```

Applies to **`function`**, **`type`**, **`repeat`**, **`try`**, and all blocks ([13 — Loops](13-loops.md), [27 — Error Handling](27-error-handling.md)).

**Professional version:**

```lp
function greet(name),
    print "Hello " with name.
..    @ not .
```

---

### Advanced — strings and undefined names

**Learning version (wrong):**

```lp
name = "World".
message = "Hello " + name.    @ + is for numbers, not strings

repeat 5 times as i,
    print i.
..
print i.    @ i undefined here — E0202
```

**Learning version (right):**

```lp
name = "World".
message = "Hello " with name.

repeat 5 times as i,
    print i.
..
@ use i only inside the block
```

**Professional version:**

```lp
message = "Hello " with name.
print message.
```

---

## Common Mistakes

**Mistake:** Missing period `.`

```lp
x = 5    @ wrong
```

**Fix:** `x = 5.`

---

**Mistake:** Missing `..` on blocks

```lp
function f(),
    print "ok".
.    @ wrong
```

**Fix:** Close with `..`

---

**Mistake:** Using `+` instead of `with` for strings

```lp
msg = "Hi " + name    @ wrong
```

**Fix:** `msg = "Hi " with name.`

---

**Mistake:** Using undefined variables

```lp
print total.    @ never assigned — E0202
```

**Fix:** Assign first: `total = 0.`

---

**Mistake:** Using `end` or `}` (other languages)

Lang.P uses **`..`** only — not `end`, `end.`, or `}`.

---

**Mistake:** Calling unimplemented APIs

```lp
use network.
@ HTTP not implemented in v0.2.0
```

**Fix:** Check [24 — Modules](24-modules.md) and [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md).

---

## Best Practices

- Run **`lang check`** after every edit — errors show line and column ([30 — Debugging](30-debugging.md)).
- Keep [32 — Best Practices](32-best-practices.md) open while coding.
- When stuck, compare your block structure to a working example in [examples/](../../examples/).
- Read the **error code** in brackets — E0200 vs E0202 tells you parse vs. name issues.
- Reduce the file to the smallest broken example before asking for help.

---

## Exercises

### Beginner

1. Fix five statements missing `.`.
2. Fix an `if` block closed with `.` instead of `..`.
3. Change `"A" + "B"` to use `with`.
4. Fix `print mesage.` (typo) — note E0202.
5. Run `lang check` on your fixed file until clean.

### Intermediate

1. Find loop variable used outside block — rewrite to store result in outer variable.
2. Fix wrong indentation after `if cond,` ([03 — Language Basics](03-language-basics.md)).
3. Fix `if x = 5,` — use `==` for comparison ([07 — Operators](07-operators.md)).
4. Collect three errors from [Manual: Error Messages](../manual/25-error-messages.md) and reproduce each.
5. Debug [examples/modules.lp](../../examples/modules.lp) after removing `use filesystem.`

### Advanced

1. Write a "lint checklist" poster: `.`, `..`, `with`, scope, imports.
2. Given a broken 20-line program, fix all errors using only `lang check` output.

---

## Summary

Watch for **missing `.`**, **`.` instead of `..`**, **`+` for strings**, and **undefined names**. Error codes **E0200** (parse) and **E0202** (semantic) point the way. Follow [32 — Best Practices](32-best-practices.md) to avoid repeating these patterns.

**Previous:** [32 — Best Practices](32-best-practices.md) · **Next:** [34 — Language Reference](34-language-reference.md)

**See also:** [03 — Language Basics](03-language-basics.md), [07 — Operators](07-operators.md), [10 — Strings](10-strings.md), [30 — Debugging](30-debugging.md), [32 — Best Practices](32-best-practices.md), [Manual: Common Mistakes](../manual/24-common-mistakes.md), [Manual: Error Messages](../manual/25-error-messages.md)
