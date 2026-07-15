# Comments

## Introduction

**Comments** are notes for humans. The computer ignores them. In Lang.P, comments start with `@` and continue to the end of the line.

**Why `@`:** It reads like "annotation" or "note" and does not conflict with string quotes or operators.

**When to comment:** Explain *why*, not *what* obvious code already shows. Learning examples use more comments; production code uses fewer ([32 — Best Practices](32-best-practices.md)).

---

## Syntax

```lp
@ This entire line is a comment.
name = "Naga".    @ inline comment after code (same line)
```

Comments cannot span multiple lines unless each line starts with `@`:

```lp
@ Line one of a multi-line note.
@ Line two of the same note.
```

---

## Examples

### Simple

**Learning version:**

```lp
@ Program: greet.lp — prints a welcome message.
print "Welcome.".
```

**Professional version:**

```lp
print "Welcome.".
```

### Intermediate — document a function

**Learning version:**

```lp
@ Return the larger of two integers.
function max(a, b),
    if a >= b,
        return a.
    otherwise,
        return b.
    ..
..

result = max(10, 25).
print result.
```

**Professional version:**

```lp
function max(a, b),
    if a >= b,
        return a.
    otherwise,
        return b.
    ..
..

result = max(10, 25).
print result.
```

### Advanced — section headers in a long file

**Learning version:**

```lp
@ === Configuration ===
max_users = 100.

@ === Main logic ===
function run(),
    print max_users.
..
..
```

**Professional version:**

```lp
max_users = 100.

function run(),
    print max_users.
..
..
```

---

## Common mistakes

**Mistake:** Using `#` or `//` like other languages.

```lp
# not a comment in Lang.P
```

**Fix:** Use `@` only.

---

**Mistake:** Commenting out code by breaking syntax.

```lp
@ print "disabled".   @ OK — whole line commented
print "Hello".       @ to disable, comment the whole line
```

---

## Best practices

- Use `@` at the top of files: purpose, author, how to run.
- Do not comment every line in production — prefer clear names ([05 — Variables](05-variables.md)).
- Keep comments updated when code changes; wrong comments harm more than none.

---

## Exercises

### Beginner

1. Add a one-line `@` description to your hello program.
2. Write a program where every line has an `@` comment (learning style).
3. Comment out a `print` by prefixing `@` on that line.
4. Explain in a comment what `.` does at end of line.
5. Run `lang check` — confirm comments never cause errors.

### Intermediate

1. Document a function with `@` describing parameters and return value.
2. Rewrite a over-commented program as professional version.
3. Add section headers (`@ === Section ===`) to a 20-line program.
4. Compare `@` to Python `#` in a short paragraph (in comments or notes).
5. Find three comments in [examples/hello.lp](../../examples/hello.lp).

### Advanced

1. Write a style guide for your team: when to comment vs rename variables.
2. Document a tricky algorithm with `@` above each logical step only.
3. Propose one improvement to this chapter via a GitHub issue or PR.

---

## Summary

Comments start with `@` and are ignored at runtime. Use them to teach and to explain non-obvious intent.

**Previous:** [03 — Language Basics](03-language-basics.md) · **Next:** [05 — Variables](05-variables.md)

**See also:** [32 — Best Practices](32-best-practices.md), [STYLE_GUIDE.md](../../STYLE_GUIDE.md)
