# Collections Overview

## Introduction

Real programs rarely work with a single number or string. They manage **groups of data**: shopping lists, user profiles, unique tags, and fixed coordinates. Lang.P v0.2.0 provides four built-in collection types, each tuned for a different job.

| Type | Literal | Mutable? | Ordered? | Use when |
|------|---------|----------|----------|----------|
| **List** | `[1, 2, 3]` | Yes | Yes | Sequences you change (append, sort, index) |
| **Dictionary** | `{name: "Naga", age: 18}` | Yes | Insertion order | Look up values by key |
| **Set** | `{1, 2, 3}` | Yes | No | Unique values, set algebra |
| **Tuple** | `(10, 20)` | No | Yes | Fixed-size records that must not change |

**Why four types:** One structure cannot fit every problem. Lists model sequences; dictionaries model records; sets model uniqueness; tuples model fixed snapshots.

**When to choose:** Start with a List if order matters and duplicates are allowed. Use a Dictionary when you need named fields. Use a Set for membership and union/intersection. Use a Tuple when immutability is a feature, not a limitation.

**Status:** All four collection types are **Stable** in v0.2.0. Generic annotations (`List<Int>`, `Dictionary<String, Int>`) are validated at compile time.

---

## Syntax

### List

```lp
items = [1, 2, 3].
items: List<Int> = [1, 2, 3].
items[0] = 99.
items.append(4).
items.length().
```

### Dictionary

Colon (`:`) inside `{ }` makes a **Dictionary**. Keys are labels; use dot or bracket access.

```lp
user = {name: "Naga", age: 18}.
user.name = 19.
user["age"] = 20.
user.length().
```

### Set

Values only — no colons — inside `{ }` make a **Set**.

```lp
tags = {1, 2, 3}.
tags.add(4).
tags.union(other).
```

### Tuple

Parentheses create an **immutable** fixed-size group.

```lp
point = (10, 20).
print point[0].
print point.length().
@ point[0] = 5.   @ ERROR — tuples cannot be modified
```

### Typed annotations

```lp
scores: List<Int> = [95, 87, 92].
ages: Dictionary<String, Int> = {bob: 30, alice: 25}.
ids: Set<Int> = {1, 2, 3}.
```

The semantic analyzer rejects literals that violate the annotation (for example, a string inside `List<Int>`).

### Built-in `len()`

All four collections support the global `len()` function as an alternative to `.length()`:

```lp
print len([1, 2, 3]).
```

---

## Examples

### Simple — create and read

**Learning version:**

```lp
@ Four ways to hold data.
fruits = ["apple", "banana"].
person = {name: "Naga", age: 18}.
unique = {1, 2, 3}.
coord = (10, 20).

print fruits[0].
print person.name.
print unique.contains(2).
print coord[1].
```

**Professional version:**

```lp
fruits = ["apple", "banana"].
person = {name: "Naga", age: 18}.
unique = {1, 2, 3}.
coord = (10, 20).

print fruits[0].
print person.name.
print unique.contains(2).
print coord[1].
```

### Intermediate — mutate lists and dicts

**Learning version:**

```lp
@ Lists and dicts can grow and change.
nums = [3, 1, 2].
nums.append(4).
nums.sort().
print nums.contains(4).

student = {name: "Alex", grade: 85}.
student.grade = 90.
print student.contains("name").
print student.length().
```

**Professional version:**

```lp
nums = [3, 1, 2].
nums.append(4).
nums.sort().
print nums.contains(4).

student = {name: "Alex", grade: 85}.
student.grade = 90.
print student.contains("name").
print student.length().
```

### Advanced — typed collections and iteration

**Learning version:**

```lp
@ Annotations catch mistakes before run time.
scores: List<Int> = [95, 87, 92].
labels: List<String> = ["a", "b", "c"].
ages: Dictionary<String, Int> = {bob: 30, alice: 25}.

for score in scores,
    print score.
..

for key, value in ages.items(),
    print key with ": " with value.
..

set_a = {1, 2, 3}.
set_b = {3, 4, 5}.
print set_a.union(set_b).length().
```

**Professional version:**

```lp
scores: List<Int> = [95, 87, 92].
labels: List<String> = ["a", "b", "c"].
ages: Dictionary<String, Int> = {bob: 30, alice: 25}.

for score in scores,
    print score.
..

for key, value in ages.items(),
    print key with ": " with value.
..

set_a = {1, 2, 3}.
set_b = {3, 4, 5}.
print set_a.union(set_b).length().
```

---

## Common Mistakes

**Mistake:** Using `{1, 2, 3}` when you meant a dictionary (or the reverse).

```lp
@ WRONG — this is a Set, not a map.
bad = {name "Naga"}.
```

**Fix:** Dictionaries require `key: value` pairs with a colon:

```lp
good = {name: "Naga"}.
```

---

**Mistake:** Trying to modify a tuple.

```lp
point = (10, 20).
point[0] = 99.    @ runtime error: Tuple is immutable
```

**Fix:** Use a List if you need index assignment, or create a new tuple.

---

**Mistake:** Confusing `len()` with `.length()`.

Both work on collections. Pick one style per project and stay consistent. Methods like `.append()` only exist on the collection itself, not on `len()`'s return value.

---

## Best Practices

- Choose the smallest collection that fits: do not use a List when a Set's uniqueness is what you need.
- Add type annotations on public-facing data (`List<Int>`, `Dictionary<String, Int>`) so `lang check` catches mismatches early.
- Iterate with `for x in list` for sequences; use `for key, value in dict.items()` for dictionary entries.
- Keep related collection logic together — see [15 — Dictionaries](15-dictionaries.md), [16 — Sets](16-sets.md), and [17 — Tuples](17-tuples.md) for deep dives.
- Run `lang run examples/collections.lp` to verify your environment.

---

## Exercises

### Beginner

1. Create a List of three favorite foods and print the second item.
2. Create a Dictionary with `name` and `city` keys; print using dot access.
3. Create a Set with duplicate values written twice (`{1, 1, 2}`) — observe only unique values remain.
4. Create a Tuple `(x, y)` and print both indices.
5. Print `len()` of each collection you created.

### Intermediate

1. Append, sort, and reverse a List of integers; verify order with a `for` loop.
2. Annotate a variable as `List<String>` and run `lang check` on a file with one wrong element type.
3. Build two Sets and print their union, intersection, and difference.
4. Loop over a Dictionary with `for key, value in d.items()`.
5. Explain in a comment why `{a: 1}` is a Dict but `{1, 2}` is a Set.

### Advanced

1. Write a small program that stores student scores in a `Dictionary<String, Int>`, then prints every name and score in sorted name order (hint: extract keys, sort the list of keys).
2. Compare when you would choose a Tuple over a List for a 2D coordinate — write both versions and list trade-offs in comments.

---

## Summary

Lang.P offers four collections: **List** (mutable sequence), **Dictionary** (key-value map), **Set** (unique values with algebra), and **Tuple** (immutable fixed group). Bracket syntax `[ ]` is for lists; `{ }` with colons is a dict, without colons is a set; `( )` is a tuple. Generic annotations add compile-time safety.

**Previous:** [13 — Loops](13-loops.md) · **Next:** [15 — Dictionaries](15-dictionaries.md)

**See also:** [16 — Sets](16-sets.md), [17 — Tuples](17-tuples.md), [34 — Language Reference](34-language-reference.md), [examples/collections.lp](../../examples/collections.lp)
