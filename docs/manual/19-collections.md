# 19 — Collections

**Status: Implemented**

Lang.P supports four collection types: **List**, **Dictionary**, **Set**, and **Tuple**. Generic type annotations (`List<Int>`, `Dictionary<String, Int>`) are validated at compile time.

---

## Lists

Ordered, mutable, indexed from `0`:

```lp
numbers: List<Int> = [1, 2, 3].
names = ["Naga", "Alex", "John"].
mixed = [1, "Hello", true].

print numbers[0].
numbers[1] = 20.
numbers.append(4).
numbers.insert(2, 100).
numbers.remove(3).
print numbers.pop().
numbers.clear().
numbers = [3, 1, 2].
numbers.sort().
numbers.reverse().
print numbers.contains(2).
print numbers.length().

for item in numbers,
    print item.
..
```

| Method | Description |
|--------|-------------|
| `append(x)` | Add element at end |
| `insert(i, x)` | Insert at index |
| `remove(x)` | Remove first matching value |
| `pop()` | Remove and return last element |
| `clear()` | Remove all elements |
| `sort()` | Sort in place |
| `reverse()` | Reverse in place |
| `contains(x)` | Whether value is present |
| `length()` | Number of elements |

---

## Dictionaries

Key-value maps. Identifier keys are labels (not variable references):

```lp
student = {
    name : "Naga",
    age : 18,
    college : "DSU"
}.

print student.name.
print student["name"].
student.age = 19.
student["age"] = 20.
print student.keys().
print student.values().
print student.contains("name").
student.remove("college").
student.clear().

student = { name : "Naga", age : 18 }.
for item in student.items(),
    print item.
..
```

| Method | Description |
|--------|-------------|
| `keys()` | List of keys |
| `values()` | List of values |
| `items()` | List of `[key, value]` pairs |
| `remove(key)` | Remove entry by key |
| `contains(key)` | Whether key exists |
| `clear()` | Remove all entries |
| `length()` | Number of entries |

---

## Sets

Unordered collections of unique values. `{1, 2, 3}` is a **Set**; `{key: value}` is a **Dict** (colon disambiguates):

```lp
set_a = {1, 2, 3}.
set_b = {3, 4, 5}.
set_a.add(6).
set_a.remove(3).
print set_a.contains(2).
print set_a.union(set_b).
print set_a.intersection(set_b).
print set_a.difference(set_b).
print set_a.length().
```

| Method | Description |
|--------|-------------|
| `add(x)` | Insert value |
| `remove(x)` | Remove value |
| `contains(x)` | Whether value is present |
| `clear()` | Remove all elements |
| `union(other)` | Values in either set |
| `intersection(other)` | Values in both sets |
| `difference(other)` | Values in this set but not other |
| `length()` | Number of elements |

---

## Tuples

Fixed-size, **immutable** ordered groups:

```lp
point = (10, 20).
print point[0].
print point.length().
print point.contains(10).
```

Index assignment on tuples is not allowed.

---

## Generic types

Annotate variables for static validation:

```lp
scores: List<Int> = [95, 87, 92].
labels: List<String> = ["a", "b"].
ages: Dictionary<String, Int> = { bob : 30, alice : 25 }.
```

Mixed-type lists that violate the annotation are rejected by the type checker.

---

## Indexing and iteration

```lp
items = [10, 20, 30].
middle = items[1].

for x in items,
    print x.
..
```

`for … in …` works over List, Set, Tuple, Dictionary (keys), and String (characters).

---

## Next steps

- [20 — Standard Library](20-standard-library.md)
- [Language reference — Collections](../guides/LANGUAGE-REFERENCE.md#collections)
- [Example: collections.lp](../../examples/collections.lp)
