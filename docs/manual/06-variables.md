# 06 — Variables

**Status: Implemented (v0.1)**

---

## Assignment

Variables use Python-style assignment and **must** end with `.`:

### Learning version

```lp
@ Store user data.
name = "Naga".
age = 18.
salary = 45000.50.
online = true.
```

### Professional version

```lp
name = "Naga".
age = 18.
salary = 45000.50.
online = true.
```

---

## Reassignment

```lp
count = 0.
count = count + 1.
```

---

## Naming conventions

| Style | Use | Example |
|-------|-----|---------|
| `snake_case` | Variables, functions | `user_name`, `total_count` |
| `PascalCase` | Types (specification) | `User`, `HttpClient` |
| `UPPER_SNAKE` | Constants (convention) | `MAX_RETRIES` |

Names must start with a letter or `_`, then letters, digits, or `_`.

---

## Constants

Lang.P v0.1 does not enforce immutable bindings. By convention, use `UPPER_SNAKE` for values that should not change:

```lp
@ Maximum login attempts (convention — not enforced by compiler yet).
MAX_ATTEMPTS = 3.
```

The specification defines `let` and `const` for future static checking.

---

## Scope

- Variables assigned at the top level of a file are visible below their assignment in that file.
- Loop variables (`repeat 5 times as i`) exist **only inside** the loop block.
- Function parameters are visible inside the function body.

```lp
repeat 3 times as i,
    print i.
..
@ i is not visible here — using i would be an error.
```

---

## Next steps

- [07 — Functions](07-functions.md)
- [Variables (spec)](../spec/05-variables-assignment.md)
