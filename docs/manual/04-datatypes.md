# 04 — Data Types

**Status: Implemented (v0.1)**

---

## Overview

| Type | Description | Example |
|------|-------------|---------|
| **Int** | Whole numbers | `42`, `-7` |
| **Float** | Decimal numbers | `3.14`, `-0.5` |
| **Bool** | `true` or `false` | `true` |
| **String** | Text in double quotes | `"hello"` |
| **Char** | Single character | `'a'` |
| **List** | Ordered collection | `[1, 2, 3]` |
| **Dict** | Key-value map | `{"name": "Naga"}` |
| **Null** | Absence of value | `null` |

---

## Integers and floats

### Learning version

```lp
@ Whole and decimal numbers.
count = 10.
pi = 3.14.
print count with " items, pi = " with pi.
```

### Professional version

```lp
count = 10.
pi = 3.14.
```

---

## Booleans

```lp
active = true.
adult = age >= 18.
```

---

## Lists

```lp
@ A list of numbers.
numbers = [1, 2, 3, 4, 5].
first = numbers[0].
```

---

## Dictionaries

```lp
@ Key-value data.
user = {"name": "Naga", "age": 18}.
print user["name"].
```

---

## Null

```lp
@ No value assigned yet.
result = null.
```

---

## Type names in the specification

The full spec defines additional types (`String`, `Object`, `Set`, …) for static typing and standard library APIs. Runtime v0.1 uses dynamic values; explicit type annotations on variables are **specification** features.

---

## Next steps

- [05 — Strings](05-strings.md)
- [Types (spec)](../spec/04-types.md)
