# Dictionaries

## Introduction

A **Dictionary** (also called a **dict** or **map**) stores data as **key → value** pairs. Instead of remembering that index `0` is the name and index `1` is the age, you use readable keys like `name` and `age`.

**Why dictionaries:** They model real-world records — users, products, settings — where each field has a meaningful name.

**When to use one:** Whenever you look up information by a label rather than by position. If order of keys matters for display, dictionaries preserve insertion order in v0.2.0.

**Status:** **Stable** in v0.2.0.

---

## Syntax

### Literal

Keys are identifiers or strings; values are any expression. Colons separate key from value:

```lp
student = {
    name: "Naga",
    age: 18,
    college: "DSU"
}.
```

Compact form:

```lp
student = {name: "Naga", age: 18}.
```

### Access and assignment

| Form | Example |
|------|---------|
| Dot access | `student.name` |
| Bracket access | `student["name"]` |
| Dot assign | `student.age = 19.` |
| Bracket assign | `student["age"] = 20.` |

### Methods

| Method | Arguments | Returns / effect |
|--------|-----------|------------------|
| `keys()` | none | List of key strings |
| `values()` | none | List of values |
| `items()` | none | List of `[key, value]` pairs |
| `contains(key)` | key | `true` if key exists |
| `remove(key)` | key | Removes entry (error if missing) |
| `clear()` | none | Removes all entries |
| `length()` | none | Number of entries |

### Typed annotation

```lp
scores: Dictionary<String, Int> = {alice: 95, bob: 87}.
@ Alias: Dict<String, Int> also works in type expressions.
```

### Iteration

```lp
for key, value in student.items(),
    print key with ": " with value.
..
```

---

## Examples

### Simple — create and read

**Learning version:**

```lp
@ A dictionary is a labeled record.
student = {name: "Naga", age: 18}.

print student.name.
print student["age"].
```

**Professional version:**

```lp
student = {name: "Naga", age: 18}.
print student.name.
print student["age"].
```

### Intermediate — update and inspect

**Learning version:**

```lp
@ Change fields and query the map.
student = {name: "Naga", age: 18, college: "DSU"}.

student.age = 19.
student["age"] = 20.
print student.contains("name").
print student.length().

keys = student.keys().
print len(keys).
```

**Professional version:**

```lp
student = {name: "Naga", age: 18, college: "DSU"}.
student.age = 19.
student["age"] = 20.
print student.contains("name").
print student.length().

keys = student.keys().
print len(keys).
```

### Advanced — iteration and cleanup

**Learning version:**

```lp
@ Walk every entry; remove and reset when done.
config = {host: "localhost", port: 8080, debug: true}.

for key, value in config.items(),
    print key with " = " with to_string(value).
..

config.remove("debug").
print config.contains("debug").

config.clear().
print config.length().
```

**Professional version:**

```lp
config = {host: "localhost", port: 8080, debug: true}.

for key, value in config.items(),
    print key with " = " with to_string(value).
..

config.remove("debug").
print config.contains("debug").

config.clear().
print config.length().
```

---

## Common Mistakes

**Mistake:** Forgetting the colon — Lang.P parses `{1, 2}` as a Set, not a Dictionary.

```lp
@ WRONG
bad = {name "Naga"}.
```

**Fix:**

```lp
good = {name: "Naga"}.
```

---

**Mistake:** Calling `remove` on a key that does not exist.

```lp
d = {a: 1}.
d.remove("missing").    @ runtime error
```

**Fix:** Check first with `contains`:

```lp
if d.contains("missing"),
    d.remove("missing").
..
```

---

**Mistake:** Using dot access with dynamic key names stored in a variable.

Dot access requires a literal identifier. For variable keys, use brackets:

```lp
field = "age".
print student[field].    @ correct
```

---

## Best Practices

- Prefer **dot access** for fixed, known keys (`user.name`) and **brackets** when the key comes from a variable or is not a valid identifier.
- Use `items()` when you need both key and value in a loop; avoid repeated lookups.
- Annotate with `Dictionary<String, Int>` (or your types) on shared data structures.
- Keep keys consistent — pick `snake_case` or `camelCase` and stick to it across the project.
- Do not use a Dictionary when a List of values is enough; simpler types are easier to debug.

---

## Exercises

### Beginner

1. Create a dictionary `{title: "Book", pages: 200}` and print both fields.
2. Change `pages` to `250` using dot assignment.
3. Add a new key `author` with bracket assignment.
4. Print whether the dictionary `contains("title")`.
5. Print `length()` after adding the author.

### Intermediate

1. Build a `{product: price}` dictionary with three items; loop with `for key, value in d.items()`.
2. Store the result of `keys()` in a variable and print its length with `len()`.
3. Annotate a variable as `Dictionary<String, Int>` and run `lang check`.
4. Remove one key and verify with `contains`.
5. Use `clear()` and confirm `length()` is zero.

### Advanced

1. Write a word-frequency counter: split a string into words manually (or use a fixed list), count occurrences in a `Dictionary<String, Int>`, print each word and count.
2. Build a nested record using dictionary values that are themselves lists or dictionaries (mixed-type values are allowed at runtime).

---

## Summary

Dictionaries map **keys to values** using `{key: value}` syntax. Access with dot or bracket notation; mutate with assignment. Methods `keys`, `values`, `items`, `contains`, `remove`, `clear`, and `length` cover the common operations. Use `Dictionary<K, V>` annotations for compile-time validation.

**Previous:** [14 — Collections Overview](14-collections.md) · **Next:** [16 — Sets](16-sets.md)

**See also:** [14 — Collections Overview](14-collections.md), [17 — Tuples](17-tuples.md), [34 — Language Reference](34-language-reference.md), [examples/collections.lp](../../examples/collections.lp)
