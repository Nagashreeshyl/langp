# Output

## Introduction

**Output** sends text from your program to the terminal. In Lang.P the primary command is `print` — simple, readable, and designed for beginners who want to see results immediately.

**Why `print`:** Every first program needs feedback. `print` ends with `.` like every other statement and uses `with` to mix text and values without format codes.

**When to use output:** Showing results, debugging, building CLI tools, and responding after [08 — Input](08-input.md). For partial-line progress indicators, use `print inline`.

---

## Syntax

```lp
print "Hello, Lang.P!".
print "Hello " with name with "!".
print "Sum: " with (a + b).
print inline "Loading".
print inline ".".
```

| Form | Behavior |
|------|----------|
| `print "text".` | Print text, then newline |
| `print "text" with value.` | Append values (converted to text) |
| `print inline …` | Print without trailing newline |

Each `with` appends one expression. Use parentheses when appending arithmetic: `with (a + b)`.

---

## Examples

### Simple — hello world

**Learning version:**

```lp
@ One literal string to the terminal.
print "Hello, Lang.P!".
```

**Professional version:**

```lp
print "Hello, Lang.P!".
```

### Intermediate — text and variables

**Learning version:**

```lp
@ with chains text and values left to right.
name = "Naga".
score = 95.

print "Player: " with name.
print "Score: " with score with " points".
```

**Professional version:**

```lp
name = "Naga".
score = 95.
print "Player: " with name with " — " with score.
```

See [examples/hello.lp](../../examples/hello.lp).

### Advanced — inline progress

**Learning version:**

```lp
@ inline keeps output on one line.
print inline "Loading".
repeat 3 times,
    print inline ".".
..
print "".    @ final newline after dots
print "Done.".
```

**Professional version:**

```lp
print inline "Loading".
repeat 3 times,
    print inline ".".
..
print "".
print "Done.".
```

---

## Common Mistakes

**Mistake:** Forgetting `.` after `print`

```lp
print "Hello"    @ wrong
```

**Fix:**

```lp
print "Hello".
```

---

**Mistake:** Using `+` inside `print`

```lp
@ print "Sum: " + total    @ wrong — use with
```

**Fix:**

```lp
print "Sum: " with total.
```

---

**Mistake:** Unparenthesized math in `with`

```lp
print "Result: " with 2 + 3 * 4.    @ confusing parse
```

**Fix:**

```lp
print "Result: " with (2 + 3 * 4).
```

---

**Mistake:** Expecting `print inline` to add a newline

```lp
print inline "A".
print inline "B".
@ Output: AB on same line — intentional
```

**Fix:** Use regular `print` when you want each message on its own line.

---

## Best Practices

- Use `with` for every mix of text and variables — consistent with [10 — Strings](10-strings.md).
- Put computed values in parentheses: `print "Avg: " with (total / count).`
- Use `print inline` only for deliberate same-line output (spinners, dots).
- Keep messages user-friendly; avoid dumping raw internal values without labels.
- After a block of `print inline` fragments, print an empty string or use `print` to end the line cleanly.

---

## Exercises

### Beginner

1. Print your name in one `print` statement.
2. Store `city = "Chennai".` and print `"I live in "` with `city`.
3. Print three separate lines with three `print` statements.
4. Fix a `print` missing its trailing `.`
5. Run [examples/hello.lp](../../examples/hello.lp) and identify every `print`.

### Intermediate

1. Print a receipt: item name, quantity, and line total using multiple `with` segments.
2. Read two numbers via `input number` and print sum, difference, product, and quotient on separate lines.
3. Build a loading line with `print inline` and three dots.
4. Print a boolean and an integer using `with` (observe automatic conversion).
5. Refactor three plain strings into one `print` with `with` between literal parts.

### Advanced

1. Simulate a progress bar: ten `print inline "#"` calls, then a newline and "100%".
2. Compare output of `print inline` vs `print` in a five-line demo program with comments explaining when to use each.

---

## Summary

`print` writes to the terminal and ends with `.`. Chain text and values with `with`; wrap math in parentheses. Use `print inline` when you need multiple fragments on one line without a newline after each.

**Previous:** [08 — Input](08-input.md) · **Next:** [10 — Strings](10-strings.md)

**See also:** [07 — Operators](07-operators.md), [10 — Strings](10-strings.md), [30 — Debugging](30-debugging.md), [examples/hello.lp](../../examples/hello.lp)
