# Strings

## Introduction

**Strings** hold text — names, messages, file paths, and anything you read with `input text`. Lang.P strings are immutable UTF-8 text in double quotes.

**Why a dedicated string model:** Lang.P separates number math (`+`) from text building (`with`) so beginners never confuse `"5" + 5` with `5 + 5`.

**When to work with strings:** Greetings, formatting output ([09 — Output](09-output.md)), labels, and any human-readable data. Use `len` and `to_string` for length and conversion.

---

## Syntax

### Literals

```lp
name = "Naga".
message = "Hello, Lang.P!".
```

### Escapes (inside double quotes)

| Sequence | Meaning |
|----------|---------|
| `\"` | Double quote |
| `\\` | Backslash |
| `\n` | Newline |
| `\t` | Tab |

### Composition with `with`

```lp
greeting = "Hello " with name with "!".
print "You are " with age with " years old".
```

### Builtins

```lp
print len("hello").        @ 5
print to_string(42).       @ "42"
```

**There is no `+` for strings** in Lang.P v0.2.0. Use `with` exclusively.

---

## Examples

### Simple — build a greeting

**Learning version:**

```lp
@ Join literal text and a variable.
name = "World".
print "Hello " with name with "!".
```

**Professional version:**

```lp
name = "World".
print "Hello " with name with "!".
```

### Intermediate — stored message

**Learning version:**

```lp
@ with works in assignments too, not only print.
first = "Lang".
second = "P".
full = first with "." with second.

print full.
print "Length: " with len(full).
```

**Professional version:**

```lp
first = "Lang".
second = "P".
full = first with "." with second.
print full with " (" with len(full) with " chars)".
```

### Advanced — mixed types in one line

**Learning version:**

```lp
@ to_string converts numbers for text output.
item = "Notebook".
qty = 3.
price = 12.50.

line = item with " x" with to_string(qty) with " @ $" with to_string(price).
print line.
print "Total: $" with to_string(qty * price).
```

**Professional version:**

```lp
item = "Notebook".
qty = 3.
price = 12.50.
print item with " x" with to_string(qty) with " @ $" with to_string(price).
```

---

## Common Mistakes

**Mistake:** Using `+` to concatenate

```lp
@ msg = "Hi " + name    @ wrong
```

**Why:** `+` is for numeric addition ([07 — Operators](07-operators.md)).

**Fix:**

```lp
msg = "Hi " with name.
```

---

**Mistake:** Single quotes for strings

```lp
@ name = 'Naga'    @ wrong — single quotes are for characters in the full spec
```

**Fix:**

```lp
name = "Naga".
```

---

**Mistake:** Forgetting `to_string` when mixing types in assignment

```lp
@ label = "Count: " with count    @ OK in print; in complex builds, be explicit
```

**Fix:** `with` converts at runtime for `print`; use `to_string` when clarity matters:

```lp
label = "Count: " with to_string(count).
```

---

**Mistake:** Unescaped quotes inside strings

```lp
print "She said "Hi"." .    @ parse error
```

**Fix:**

```lp
print "She said \"Hi\".".
```

---

## Best Practices

- Use `with` everywhere you would use `+` for strings in other languages.
- Use `to_string` before embedding numbers in stored string variables when readability matters.
- Use `len` for password length checks — not for security validation alone.
- Break very long messages across multiple `with` segments or `\n` — not multiple unrelated formats.
- Keep string literals in double quotes; reserve prompts for `input` ([08 — Input](08-input.md)).

---

## Exercises

### Beginner

1. Print `"Lang.P"` and your name on one line using `with`.
2. Store `a = "Hello "` and `b = "World"` and combine with `with`.
3. Print `len("Lang.P")`.
4. Fix a line that uses `"Hi " + name`.
5. Print a string with an embedded `\n` for two lines.

### Intermediate

1. Build an email-style greeting: `"Dear "` with name with `",\nWelcome!"`.
2. Use `to_string` to build `"Order #"` with order_id (an `Int`).
3. Read a name with `input` and print `"Nice to meet you, "` with name.
4. Escape a quote inside `"She said \"Go\"."` and print it.
5. Compare `len("")` and `len("a")` — print both.

### Advanced

1. Format a table row: name (String), score (Int), passed (Bool) in one line with `with` and `to_string`.
2. Write a short style guide comment block: when to use `with` vs separate `print` lines.

---

## Summary

Strings use double-quoted literals. Join text with `with`, never `+`. Use `len(value)` for length and `to_string(value)` for explicit conversion. Escapes `\n`, `\t`, `\"`, and `\\` work inside literals.

**Previous:** [09 — Output](09-output.md) · **Next:** [11 — Functions](11-functions.md)

**See also:** [07 — Operators](07-operators.md), [08 — Input](08-input.md), [14 — Collections Overview](14-collections.md), [examples/hello.lp](../../examples/hello.lp)
