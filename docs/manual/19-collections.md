# 19 — Collections

**Status: Implemented (v0.1)**

---

## Lists

Ordered, indexed from `0`:

### Learning version

```lp
@ Create and use a list.
fruits = ["apple", "banana", "cherry"].
print fruits[0].
print len(fruits).
```

### Professional version

```lp
fruits = ["apple", "banana", "cherry"].
for fruit in fruits,
    print fruit.
..
```

---

## Dictionaries

Key-value maps:

```lp
@ User profile as dict.
user = {"name": "Naga", "age": 18, "active": true}.
print user["name"].
user["age"] = 19.
```

---

## Tuples

Parenthesized groups `(a, b)` parse as tuples; v0.1 represents them similarly to lists.

---

## Sets

Set types and operations are **specification** — see [Standard library](../spec/16-standard-library.md).

---

## Indexing

```lp
items = [10, 20, 30].
middle = items[1].
```

---

## Next steps

- [20 — Standard Library](20-standard-library.md)
- [Collections in spec](../spec/16-standard-library.md)
