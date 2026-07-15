# Language Basics

## Introduction

All Lang.P programs are built from **statements** grouped into **blocks**. Three punctuation marks define structure: `.` `,` and `..`. There are no semicolons, braces, or `#` comments.

**Why strict punctuation:** Beginners can see where a sentence ends and where a block ends without memorizing special cases.

**When to think about basics:** Always. Every feature in later chapters still uses these rules.

---

## Syntax

| Token | Name | Rule |
|-------|------|------|
| `.` | Statement end | Every executable line ends with `.` |
| `,` | Block open | After headers: `if x,`, `function f(),`, `repeat 5 times,` |
| `..` | Block close | Dedented line closing the block |
| `@` | Comment | Rest of line ignored |

**Indentation:** Use 4 spaces per block level (like Python).

```lp
if ready,
    start_engine().
    print "Started.".
..
```

One `..` closes the entire `if` / `otherwise` chain or `try` / `catch` / `finally` chain.

---

## Examples

### Simple — statements only

**Learning version:**

```lp
@ Three separate statements.
x = 10.
y = 20.
print x with y.
```

**Professional version:**

```lp
x = 10.
y = 20.
print x with y.
```

### Intermediate — nested blocks

**Learning version:**

```lp
@ Outer loop, inner condition.
repeat 3 times as i,
    if i > 0,
        print "Step " with i.
    ..
..
```

**Professional version:**

```lp
repeat 3 times as i,
    if i > 0,
        print "Step " with i.
    ..
..
```

### Advanced — function + conditional

**Learning version:**

```lp
function classify(score),
    if score >= 90,
        print "A".
    otherwise if score >= 80,
        print "B".
    otherwise,
        print "C".
    ..
..

classify(95).
classify(72).
```

**Professional version:**

```lp
function classify(score),
    if score >= 90,
        print "A".
    otherwise if score >= 80,
        print "B".
    otherwise,
        print "C".
    ..
..

classify(95).
classify(72).
```

---

## Common mistakes

**Mistake:** Closing a block with `.` instead of `..`.

```lp
function add(a, b),
    return a + b.
.    @ WRONG — this is not valid block close
```

**Fix:**

```lp
function add(a, b),
    return a + b.
..
```

---

**Mistake:** Omitting `,` after a block header.

```lp
if x > 0
    print x.
..
```

**Fix:**

```lp
if x > 0,
    print x.
..
```

---

## Best practices

- Align `..` with the block header column.
- One concept per block level — extract nested logic into functions ([11 — Functions](11-functions.md)).
- Run `lang check` — parse errors often mention missing `.` or `..`.

---

## Exercises

### Beginner

1. Write three assignments and one `print` — four statements, four periods.
2. Write an `if true,` block with one print inside; close with `..`.
3. Fix a broken program missing `,` after `repeat 2 times`.
4. Add `@` comments labeling "block open" and "block close" in your example.
5. Run `lang check` on a file with an intentional error; read the message.

### Intermediate

1. Nest `if` inside `repeat 5 times as i` and print only even `i`.
2. Write a function with two nested `if` blocks.
3. Convert a flat program into one function + one call.
4. Draw an indentation diagram for a nested block on paper.
5. Compare Lang.P blocks to [Grammar Freeze §4](../spec/GRAMMAR-FREEZE-v1.md).

### Advanced

1. Write `max(a, b)` using `if` without calling built-in `max`.
2. Implement FizzBuzz for 1–15 using `repeat` and nested `if`.
3. Explain why Lang.P uses `..` instead of indentation-only blocks.

---

## Summary

Lang.P uses `.` for statements, `,` to open blocks, and `..` to close them. Indent with 4 spaces. These rules never change across the language.

**Previous:** [02 — Your First Program](02-your-first-program.md) · **Next:** [04 — Comments](04-comments.md)

**See also:** [12 — Conditionals](12-conditionals.md), [13 — Loops](13-loops.md), [33 — Common Mistakes](33-common-mistakes.md)
