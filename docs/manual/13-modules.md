# 13 — Modules

**Status: Beta (v0.2)**

---

> `use` imports load built-in stdlib modules today. Project multi-file evaluation is partial. See [STATUS.md](../../STATUS.md) and [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md).

---

## Import syntax

```lp
use filesystem.
use math.
use navigator.
use ai.
```

Access exports with dot notation:

```lp
print filesystem.exists("notes.txt").
print math.abs(-5).
```

---

## Standard library modules (v0.2)

| Module | Status | Notes |
|--------|--------|-------|
| `filesystem` | ✅ Stable | Full file API |
| `math` | 🟡 Beta | `abs`, `min`, `max` |
| `json` | 🟡 Beta | Stub parse/stringify |
| `navigator` | 📋 Stub | Version string only |
| `ai` | 📋 Stub | Version string only |
| `network` | 📋 Stub | Version string only |
| `database` | 📋 Stub | Version string only |

Top-level `read` / `write` statements work without import.

---

## Project layout

```
myapp/
    langp.toml       @ manifest
    langp.lock       @ pinned dependencies
    main.lp          @ entry point
    src/             @ project modules (partial support)
    tests/           @ test programs
```

Initialize with:

```bash
lang init myapp
lang install filesystem
```

---

## Example

See [examples/modules.lp](../../examples/modules.lp).

---

## Next steps

- [14 — Error Handling](14-error-handling.md)
- [Modules (spec)](../spec/11-modules-imports.md)
- [Package system (spec)](../spec/20-package-system.md)
