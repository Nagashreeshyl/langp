# Loops

## Introduction

**Loops** repeat work without copying code. Lang.P offers several forms: counted `repeat`, unbounded `repeat forever`, conditional `while`, and collection `for`. Use `break.` to exit early and `continue.` to skip to the next iteration.

**Why multiple loop types:** Counted loops fit "do this N times"; `while` fits "until a condition changes"; `for` fits "each item in a list". Pick the one that reads closest to your intent.

**When to loop:** Printing patterns, processing lists ([14 — Collections Overview](14-collections.md)), retry logic, and game main loops. Remember loop variables from `as i` are block-local ([05 — Variables](05-variables.md)).

---

## Syntax

```lp
repeat 5 times,
    print "Hello".
..

repeat 5 times as i,
    print "i = " with i.
..

repeat forever,
    break.
..

while count > 0,
    count = count - 1.
..

for item in items,
    print item.
..

break.
continue.
```

| Form | Meaning |
|------|---------|
| `repeat N times,` | Run body N times |
| `repeat N times as name,` | Counter from `0` to `N - 1` |
| `repeat forever,` | Infinite until `break` |
| `while condition,` | While condition is true |
| `for x in list,` | Each element in a list |
| `break.` | Exit the innermost loop |
| `continue.` | Skip to next iteration |

Each loop body ends with `..`.

---

## Examples

### Simple — repeat N times

**Learning version:**

```lp
@ Like Python: for _ in range(5)
repeat 5 times,
    print "Hello".
..
```

**Professional version:**

```lp
repeat 5 times,
    print "Hello".
..
```

See [examples/loops.lp](../../examples/loops.lp).

### Intermediate — counter and while

**Learning version:**

```lp
@ Zero-based counter: 0, 1, 2, 3, 4
repeat 5 times as i,
    print "Step " with i.
..

@ Countdown with while
count = 3.
while count > 0,
    print count.
    count = count - 1.
..
print "Go!".
```

**Professional version:**

```lp
repeat 5 times as i,
    print i.
..

count = 3.
while count > 0,
    print count.
    count = count - 1.
..
```

### Advanced — for, break, and continue

**Learning version:**

```lp
@ Skip 5, stop at 8.
repeat 10 times as i,
    if i == 5,
        continue.
    ..
    if i == 8,
        break.
    ..
    print i.
..

@ For-each over a list
fruits = ["apple", "banana", "cherry"].
for fruit in fruits,
    print fruit.
..
```

**Professional version:**

```lp
repeat 10 times as i,
    if i == 5,
        continue.
    ..
    if i == 8,
        break.
    ..
    print i.
..

for fruit in ["apple", "banana", "cherry"],
    print fruit.
..
```

---

## Common Mistakes

**Mistake:** Using loop variable after the block

```lp
repeat 3 times as i,
    print i.
..
print i.    @ error — out of scope
```

**Fix:** Use the variable only inside the loop, or copy values you need outside.

---

**Mistake:** Infinite loop without `break`

```lp
repeat forever,
    print "Running".
@ never exits
```

**Fix:** Add a condition with `break`, or switch to `repeat N times`.

---

**Mistake:** Forgetting `..` to close the loop

```lp
repeat 3 times,
    print "*".
@ missing ..
```

**Fix:** Add `..` after the body.

---

**Mistake:** `while` condition never changes

```lp
n = 5.
while n > 0,
    print n.
@ forgot n = n - 1 — infinite loop
..
```

**Fix:** Update variables the condition depends on inside the body.

---

## Best Practices

- Prefer `repeat N times` when you know the count upfront.
- Use `as i` when you need the index; omit `as` when you only need repetition.
- Always ensure `while` loops progress toward termination.
- Use `break` and `continue` sparingly — a clear `if` often reads better.
- For lists, prefer `for x in list` over manual indexing when possible.

---

## Exercises

### Beginner

1. Print `"*"` ten times with `repeat 10 times`.
2. Print `0` through `4` using `repeat 5 times as i`.
3. Run [examples/loops.lp](../../examples/loops.lp) and explain the output.
4. Fix a loop missing its closing `..`
5. Write a `while` loop that prints `3, 2, 1`.

### Intermediate

1. Print even numbers from `0` to `8` using `repeat` and `if i % 2 == 0`.
2. Sum numbers `1` to `10` with a loop and a running `total` variable.
3. Loop a list `["red", "green", "blue"]` with `for color in ...`.
4. Use `continue` to skip printing when `i == 0`.
5. Nest two `repeat 3 times as` loops to print a 3×3 grid pattern.

### Advanced

1. Build a menu loop with `repeat forever`, `input number` for choice, and `break` on quit.
2. Rewrite a `while` countdown as `repeat N times as i` printing `N - i` — compare readability.

---

## Summary

Loops repeat code with `repeat N times`, `repeat forever`, `while`, and `for x in list`. Counters use `as name` from `0` to `N - 1` inside the block only. Exit early with `break.`; skip an iteration with `continue.`. Close every loop with `..`.

**Previous:** [12 — Conditionals](12-conditionals.md) · **Next:** [14 — Collections Overview](14-collections.md)

**See also:** [05 — Variables](05-variables.md), [11 — Functions](11-functions.md), [12 — Conditionals](12-conditionals.md), [examples/loops.lp](../../examples/loops.lp)
