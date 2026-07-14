# 09 — Conditions

**Status: Implemented (v0.1)**

---

## `if` / `otherwise if` / `otherwise`

Lang.P uses **`otherwise`**, not `else`:

### Learning version

```lp
@ Classify by age.
age = input number "Enter age: ".

if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

### Professional version

```lp
if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

Every branch body ends with `.`. The whole `if` closes with `..`.

---

## Nested conditions

```lp
if logged_in,
    if is_admin,
        print "Admin panel".
    otherwise,
        print "User home".
    ..
otherwise,
    print "Please log in".
..
```

---

## Inline conditional (expression)

Two forms exist in the specification:

```lp
@ Comma form (preferred in docs)
status = if score >= 60, "Pass", otherwise, "Fail".

@ Then/else form (also supported by parser)
@ label = if score >= 60 then "Pass" else "Fail".
```

Use the comma form for consistency with statement-style `if`.

---

## Comparison with Python

| Python | Lang.P |
|--------|--------|
| `if x:` | `if x,` |
| `elif x:` | `otherwise if x,` |
| `else:` | `otherwise,` |
| (indent only) | `..` to close |

---

## Next steps

- [10 — Input](10-input.md)
- [Control flow (spec)](../spec/09-control-flow.md)
