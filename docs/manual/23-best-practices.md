# 23 — Best Practices

**Status: Implemented (v0.1)**

---

## Naming

- Use **`snake_case`** for variables and functions: `user_name`, `calculate_total`
- Use **`PascalCase`** for types when OOP ships: `UserAccount`
- Use descriptive names: `max_retries` not `mr`

---

## Formatting

| Rule | Example |
|------|---------|
| 4 spaces per indent level | Standard in Lang.P IDEs |
| One statement per line | Each ends with `.` |
| Blank line between logical sections | Improves readability |
| `@` comments for non-obvious logic | Not for every line |

```lp
@ Validate before processing payment.
if amount > 0 and account_active,
    process_payment(amount).
..
```

---

## Performance

- v0.1 uses a tree-walking interpreter — fine for scripts and learning
- Avoid tight `repeat forever` without `break`
- Prefer `lang check` before shipping scripts

---

## Project structure (v0.1)

```
myproject/
    main.lp           @ entry point
    README.md
```

When modules arrive:

```
myproject/
    lang.toml
    main.lp
    lib/
        helpers.lp
```

---

## Run checklist

```bash
lang check main.lp    @ static checks first
lang run main.lp      @ then run
```

---

## Next steps

- [24 — Common Mistakes](24-common-mistakes.md)
- [25 — Error Messages](25-error-messages.md)
