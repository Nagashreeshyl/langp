# Conditionals

## Introduction

**Conditionals** let your program choose different paths based on true/false tests. Lang.P uses readable English keywords: `if`, `otherwise if`, and `otherwise` — not `else` or `elif`.

**Why this syntax:** It reads like spoken logic: "if this, do that; otherwise if that, do something else; otherwise, do the default."

**When to use conditionals:** User choices, validation after [08 — Input](08-input.md), scoring, permissions, and any branch where exactly one path should run. Use `pass.` when a branch must exist but do nothing.

---

## Syntax

```lp
if condition,
    statement.
otherwise if other_condition,
    statement.
otherwise,
    statement.
..
```

| Keyword | Role |
|---------|------|
| `if condition,` | First branch |
| `otherwise if condition,` | Additional branches |
| `otherwise,` | Default branch |
| `..` | Closes the entire if chain |
| `pass.` | No-op placeholder |

Every branch body contains statements ending with `.`. One `..` closes the whole chain.

**Inline conditional (expression form):**

```lp
status = if score >= 60, "Pass", otherwise, "Fail".
```

---

## Examples

### Simple — age groups

**Learning version:**

```lp
@ Classify by age thresholds.
age = 20.

if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

**Professional version:**

```lp
age = 20.
if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

### Intermediate — input-driven

**Learning version:**

```lp
@ Branch based on typed boolean input.
ready = input boolean "Ready to start? ".

if ready,
    print "Starting...".
otherwise,
    print "Aborted.".
..
```

**Professional version:**

```lp
if input boolean "Ready to start? ",
    print "Starting...".
otherwise,
    print "Aborted.".
..
```

### Advanced — nested conditions and pass

**Learning version:**

```lp
@ Nested if inside outer branches; pass for empty branch.
logged_in = true.
is_admin = false.

if logged_in,
    if is_admin,
        print "Admin panel".
    otherwise,
        pass.    @ logged in but not admin — no action yet
    ..
otherwise,
    print "Please log in".
..
```

**Professional version:**

```lp
if logged_in,
    if is_admin,
        print "Admin panel".
    otherwise,
        pass.
    ..
otherwise,
    print "Please log in".
..
```

---

## Common Mistakes

**Mistake:** Using `else` instead of `otherwise`

```lp
if x > 0,
    print "pos".
else,    @ wrong keyword
    print "non-pos".
..
```

**Fix:**

```lp
if x > 0,
    print "pos".
otherwise,
    print "non-pos".
..
```

---

**Mistake:** Forgetting `..` at the end

```lp
if score >= 60,
    print "Pass".
@ missing ..
```

**Fix:** Add `..` after the last branch.

---

**Mistake:** Comma after `otherwise` missing

```lp
otherwise    @ wrong — need comma
    print "Done".
..
```

**Fix:**

```lp
otherwise,
    print "Done".
..
```

---

**Mistake:** Empty branch without `pass.`

```lp
if debug,
    print "Debug on".
otherwise,
    @ nothing here — invalid empty branch
..
```

**Fix:**

```lp
if debug,
    print "Debug on".
otherwise,
    pass.
..
```

---

## Best Practices

- Order `otherwise if` branches from most specific to most general.
- Use meaningful conditions — extract complex tests into named `Bool` variables.
- Prefer `otherwise` for the unexpected case; log or print a helpful message.
- Use `pass.` sparingly — often a comment explains why the branch is intentionally empty.
- Keep nesting shallow; extract inner `if` chains into functions ([11 — Functions](11-functions.md)).

---

## Exercises

### Beginner

1. Print `"Positive"` if `n > 0`, otherwise `"Not positive"`.
2. Add a third branch: `otherwise if n == 0`, print `"Zero"`.
3. Fix an `if` block missing `..`
4. Replace `else` with `otherwise` in a broken sample.
5. Use `if` with `input number` to check if age is at least 18.

### Intermediate

1. Classify a score: A (>=90), B (>=80), C (>=70), otherwise F.
2. Nest two `if` statements: outer checks login, inner checks role.
3. Use the inline form: `label = if x > 0, "plus", otherwise, "minus".`
4. Add `pass.` to an `otherwise` branch that intentionally does nothing.
5. Combine `and` / `or` ([07 — Operators](07-operators.md)) in one condition.

### Advanced

1. Write `function grade(score),` using `if` / `otherwise if` / `otherwise` and return a letter grade.
2. Refactor deeply nested `if` into two functions with early `return` ([11 — Functions](11-functions.md)).

---

## Summary

Conditionals branch with `if`, `otherwise if`, and `otherwise`, each opening with `,` and closing the whole chain with `..`. Use `pass.` for empty branches. Comparisons and `and` / `or` / `not` build conditions. An inline form assigns one of two values in a single expression.

**Previous:** [11 — Functions](11-functions.md) · **Next:** [13 — Loops](13-loops.md)

**See also:** [07 — Operators](07-operators.md), [08 — Input](08-input.md), [11 — Functions](11-functions.md), [33 — Common Mistakes](33-common-mistakes.md)
