# 20 — Standard Library

**Status: Partial — see v0.1 table below**

---

> The [full standard library](../spec/16-standard-library.md) describes dozens of modules. This chapter lists **what works in v0.1** vs what is specification only.

---

## Builtins (v0.1 — no import needed)

| Function | Signature | Description |
|----------|-----------|-------------|
| `len` | `len(value)` | Length of string, list, or dict |
| `to_string` | `to_string(value)` | Convert to string |
| `assert` | `assert condition.` | Fail if false |
| `assert` | `assert condition, "msg".` | Fail with message |

```lp
print len("hello").
assert age >= 0, "invalid age".
```

---

## Statements (v0.1)

| Name | Purpose |
|------|---------|
| `print` | Terminal output |
| `input` | User input |
| `read` / `read_bytes` / `read_lines` | Read files |
| `write` / `write_bytes` / `append` | Write files |
| `copy` / `move` / `rename` / `delete` | File management |

---

## Planned modules (specification)

| Module | Purpose |
|--------|---------|
| `core` | Extended builtins (`panic`, `exit`, `type_of`, …) |
| `collections` | `list`, `dict`, `set` helpers |
| `math` | `sqrt`, `pow`, `floor`, … |
| `json` | Parse and stringify |
| `filesystem` | Extended file APIs |
| `network` | HTTP, WebSocket |
| `database` | Database drivers |
| `navigator` | Browser UI |
| `ai` | LLM assistants |

Do not `use` these in v0.1 programs expecting them to load.

---

## Single source of truth

When adding a builtin, update:

1. `editors/langp-manifest.json`
2. [LANGUAGE-REFERENCE.md](../guides/LANGUAGE-REFERENCE.md)
3. This chapter

---

## Next steps

- [21 — Navigator Framework](21-navigator.md)
- [Standard library (spec)](../spec/16-standard-library.md)
