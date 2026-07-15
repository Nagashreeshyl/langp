# Sets

## Introduction

A **Set** holds **unique values** with no guaranteed order. Duplicates added twice appear only once. Sets excel at membership tests and combining groups of values.

**Why sets:** Checking "is this ID already used?" or "which tags appear in both lists?" is natural with sets.

**When to use one:** Unique collections, deduplication, and set algebra (union, intersection, difference). Do not use a Set when you need index `0`, `1`, `2` access — use a [List](14-collections.md) instead.

**Status:** **Stable** in v0.2.0.

---

## Syntax

### Literal

Curly braces **without colons** create a Set:

```lp
tags = {1, 2, 3}.
colors = {"red", "green", "blue"}.
```

**Disambiguation:** `{a: 1}` is a Dictionary; `{1, 2, 3}` is a Set. The colon is the deciding token.

### Methods

| Method | Arguments | Returns / effect |
|--------|-----------|------------------|
| `add(x)` | value | Insert value (no duplicate) |
| `remove(x)` | value | Remove value (error if missing) |
| `contains(x)` | value | `true` if present |
| `clear()` | none | Remove all elements |
| `union(other)` | Set | Values in either set |
| `intersection(other)` | Set | Values in both sets |
| `difference(other)` | Set | Values in this set but not `other` |
| `length()` | none | Number of elements |

### Typed annotation

```lp
ids: Set<Int> = {1, 2, 3}.
```

### Iteration

```lp
for item in tags,
    print item.
..
```

Order is not guaranteed.

---

## Examples

### Simple — create and test membership

**Learning version:**

```lp
@ Sets store unique values.
tags = {1, 2, 3}.
tags.add(4).

print tags.contains(2).
print tags.contains(99).
print tags.length().
```

**Professional version:**

```lp
tags = {1, 2, 3}.
tags.add(4).
print tags.contains(2).
print tags.contains(99).
print tags.length().
```

### Intermediate — add, remove, clear

**Learning version:**

```lp
@ Modify membership over time.
seen = {"apple", "banana"}.
seen.add("cherry").
print seen.contains("banana").

seen.remove("banana").
print seen.contains("banana").

seen.clear().
print seen.length().
```

**Professional version:**

```lp
seen = {"apple", "banana"}.
seen.add("cherry").
print seen.contains("banana").

seen.remove("banana").
print seen.contains("banana").

seen.clear().
print seen.length().
```

### Advanced — set algebra

**Learning version:**

```lp
@ Combine sets for overlap and difference.
set_a = {1, 2, 3}.
set_b = {3, 4, 5}.

u = set_a.union(set_b).
i = set_a.intersection(set_b).
d = set_a.difference(set_b).

print u.length().
print i.contains(3).
print d.contains(1).
print d.contains(3).
```

**Professional version:**

```lp
set_a = {1, 2, 3}.
set_b = {3, 4, 5}.

u = set_a.union(set_b).
i = set_a.intersection(set_b).
d = set_a.difference(set_b).

print u.length().
print i.contains(3).
print d.contains(1).
print d.contains(3).
```

---

## Common Mistakes

**Mistake:** Using `{key: value}` syntax for a set.

```lp
@ WRONG — this is a Dictionary
bad = {tag: "news"}.
```

**Fix:**

```lp
good = {"news", "tech"}.
```

---

**Mistake:** Expecting ordered or indexed access.

```lp
s = {10, 20, 30}.
print s[0].    @ Sets are not indexed
```

**Fix:** Convert to a List if you need indices, or iterate with `for`.

---

**Mistake:** Calling `remove` on a value that is not in the set.

**Fix:** Guard with `contains` or accept the runtime error as a signal the value was absent.

---

## Best Practices

- Use Sets for **membership** and **uniqueness**, not as general-purpose sequences.
- Prefer `union` / `intersection` / `difference` over manual loops when combining groups.
- Name sets clearly (`active_ids`, `visited_urls`) to signal uniqueness intent.
- When order matters for output, copy into a List and sort before printing.
- Combine with [15 — Dictionaries](15-dictionaries.md) when you need unique keys *and* associated values.

---

## Exercises

### Beginner

1. Create a Set of three numbers and print `length()`.
2. `add` a duplicate value; verify `length()` did not increase.
3. Print `contains` for a value inside and outside the set.
4. `remove` one value and print the new length.
5. `clear` the set and confirm it is empty.

### Intermediate

1. Build two Sets of integers with one shared element; print their intersection.
2. Print the union and verify it contains all elements from both sets.
3. Print the difference `a.difference(b)` and explain the result in a comment.
4. Loop over a Set with `for` and print each item.
5. Annotate a variable as `Set<String>` and run `lang check`.

### Advanced

1. Simulate tag filtering: given Sets `all_tags` and `blocked_tags`, compute allowed tags via `difference`.
2. Write a program that reads a list of names (hard-coded List), inserts them into a Set to deduplicate, then prints the unique count versus the original list length.

---

## Summary

Sets store **unique values** using `{v1, v2, v3}` syntax (no colons). Use `add`, `remove`, `contains`, and `clear` for membership; `union`, `intersection`, and `difference` for combining sets. They are unordered and not indexable.

**Previous:** [15 — Dictionaries](15-dictionaries.md) · **Next:** [17 — Tuples](17-tuples.md)

**See also:** [14 — Collections Overview](14-collections.md), [15 — Dictionaries](15-dictionaries.md), [34 — Language Reference](34-language-reference.md), [examples/collections.lp](../../examples/collections.lp)
