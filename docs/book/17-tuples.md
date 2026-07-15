# Tuples

## Introduction

A **Tuple** is a **fixed-size, immutable** group of values. Once created, you cannot add elements, remove elements, or reassign indices. Tuples protect data that should stay constant — coordinates, RGB colors, database rows returned from a query.

**Why tuples:** Immutability prevents accidental mutation. When a function returns `(x, y)`, callers know the pair will not change underneath them.

**When to use one:** Small, fixed collections where write-once semantics matter. If you need `append` or index assignment, use a [List](14-collections.md) instead.

**Status:** **Stable** in v0.2.0.

---

## Syntax

### Literal

Parentheses with comma-separated values:

```lp
point = (10, 20).
rgb = (255, 128, 0).
mixed = (1, "hello", true).
```

### Read-only access

| Operation | Supported? |
|-----------|------------|
| Index read `t[0]` | Yes |
| Index assign `t[0] = x` | **No** — runtime error |
| `length()` | Yes |
| `contains(x)` | Yes |
| `len(t)` | Yes |
| `for item in t` | Yes |

### Methods

| Method | Description |
|--------|-------------|
| `length()` | Number of elements |
| `contains(x)` | Whether value is present |

Tuples do not support `append`, `remove`, `sort`, or other mutating list methods.

---

## Examples

### Simple — create and index

**Learning version:**

```lp
@ Tuples are fixed groups you read, not rewrite.
point = (10, 20).

print point[0].
print point[1].
print point.length().
```

**Professional version:**

```lp
point = (10, 20).
print point[0].
print point[1].
print point.length().
```

### Intermediate — contains and iteration

**Learning version:**

```lp
@ Inspect contents without changing them.
status = (200, "OK", true).

print status.contains(200).
print status.contains(404).
print len(status).

for part in status,
    print part.
..
```

**Professional version:**

```lp
status = (200, "OK", true).
print status.contains(200).
print status.contains(404).
print len(status).

for part in status,
    print part.
..
```

### Advanced — tuples vs lists

**Learning version:**

```lp
@ Use a tuple when immutability is intentional.
origin = (0, 0).
@ origin[0] = 1.   @ would fail at runtime

@ Use a list when you need to mutate.
path = [0, 0].
path[0] = 10.
path.append(20).
print path.length().

function make_pair(a, b),
    return (a, b).
..

pair = make_pair(3, 4).
print pair[0] with ", " with pair[1].
```

**Professional version:**

```lp
origin = (0, 0).
path = [0, 0].
path[0] = 10.
path.append(20).
print path.length().

function make_pair(a, b),
    return (a, b).
..

pair = make_pair(3, 4).
print pair[0] with ", " with pair[1].
```

---

## Common Mistakes

**Mistake:** Assigning to a tuple index.

```lp
point = (10, 20).
point[0] = 99.    @ Tuple is immutable
```

**Fix:** Create a new tuple or switch to a List:

```lp
point = (99, point[1]).
@ or: coords = [10, 20]. then coords[0] = 99.
```

---

**Mistake:** Confusing tuples with parenthesized expressions.

A single value in parentheses is still a one-element tuple only when the syntax is unambiguous. Prefer explicit commas for one-tuples when needed: `(42,)`.

---

**Mistake:** Using a Tuple when the size will grow.

**Fix:** Lists support `append` and dynamic sizing; tuples do not.

---

## Best Practices

- Use tuples for **small, fixed-shape** data returned from functions or passed as immutable snapshots.
- Document tuple field meaning in comments when indices alone are unclear (`@ (latitude, longitude)`).
- Prefer Lists for user-editable sequences; prefer Tuples for internal constants.
- Use `contains` for membership checks; do not rely on index scans for large tuples (tuples are typically small).
- When unpacking is needed, assign from a List or iterate with `for` — tuple unpacking assignment follows the same rules as list unpacking where supported.

---

## Exercises

### Beginner

1. Create a tuple `(100, 200)` and print both elements.
2. Print `length()` and `len()` for the same tuple.
3. Use `contains` to test for a value inside the tuple.
4. Loop over a three-element tuple and print each part.
5. Explain in a comment why `point[0] = 5` fails.

### Intermediate

1. Write a function `midpoint(a, b)` that returns `((a[0]+b[0])/2, (a[1]+b[1])/2)` using tuple points.
2. Store a `(name, score)` tuple and print a formatted line with `with`.
3. Compare memory and intent: same data as List vs Tuple in comments only.
4. Return a tuple from a function and read fields at the call site.
5. Run `lang run` on a file that attempts tuple mutation; observe the error.

### Advanced

1. Design a small API using tuples for `(success, message)` results instead of exceptions for expected failures.
2. Refactor a List-based coordinate into a Tuple-based one and list three bugs the tuple version prevents.

---

## Summary

Tuples are **immutable, fixed-size** collections written as `(a, b, c)`. Read by index; use `length()`, `contains()`, and `len()`. Index assignment is rejected at runtime. Choose tuples when data must not change after creation.

**Previous:** [16 — Sets](16-sets.md) · **Next:** [18 — Types and OOP](18-type-oop.md)

**See also:** [14 — Collections Overview](14-collections.md), [11 — Functions](11-functions.md), [34 — Language Reference](34-language-reference.md), [examples/collections.lp](../../examples/collections.lp)
