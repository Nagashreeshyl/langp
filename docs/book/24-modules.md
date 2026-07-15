# Modules

> **Beta** — Built-in standard library modules work today. Project multi-file modules are **partial**. Stub modules return a version string only.

## Introduction

A **module** is a named bundle of functions and values you import into your program. Instead of writing everything in one file, you `use filesystem.` and call `filesystem.exists("notes.txt")` — the filesystem module handles the details.

**Why modules:** Reuse tested code (file I/O, math), organize large programs, and share utilities across projects via the package manager ([28 — Package Manager](28-package-manager.md)).

**When to use them:** Whenever you need stdlib features beyond builtins — file listing, absolute value, or (eventually) JSON and networking.

---

## Syntax

There is no `module` keyword for defining your own modules in v0.2.0. You **import** existing modules:

```lp
use filesystem.
use math.
```

Access exports with **dot notation**:

```lp
print filesystem.exists("data.txt").
print math.abs(-42).
```

### Built-in standard library modules

| Module | Status | What you get today |
|--------|--------|-------------------|
| `filesystem` | ✅ Stable | `read`, `write`, `append`, `copy`, `move`, `delete`, `exists`, `list`, `create_folder`, `remove_folder`, … |
| `math` | 🟡 Beta | `abs`, `min`, `max` (integers) |
| `json` | 🟡 Beta | Stub `parse` / `stringify` — not a real JSON parser |
| `navigator` | 📋 Stub | `version` string only (`"0.0.0-stub"`) |
| `ai` | 📋 Stub | `version` string only |
| `network` | 📋 Stub | `version` string only |
| `database` | 📋 Stub | `version` string only |

Stub modules exist so `use ai.` parses and semantic analysis succeeds; they do not provide real AI, HTTP, or database APIs yet.

### Top-level I/O without import

File **statements** like `read "path"` and `write value to "path".` work **without** `use filesystem.`. The module adds function-style API (`filesystem.list`, etc.) — see [26 — Filesystem](26-filesystem.md).

---

## Examples

### Simple — filesystem and math

**Learning version:**

```lp
@ Import two stdlib modules.
use filesystem.
use math.

print filesystem.exists("/tmp").
print math.abs(-5).
```

**Professional version:**

```lp
use filesystem.
use math.

print filesystem.exists("/tmp").
print math.abs(-5).
```

See [examples/modules.lp](../../examples/modules.lp).

### Intermediate — stub module probe

**Learning version:**

```lp
@ Stub modules — only version export works.
use ai.
use network.

print "AI module: " with ai.version.
print "Network module: " with network.version.
@ Do not call HTTP helpers — not implemented.
```

**Professional version:**

```lp
use ai.
print ai.version.
```

### Advanced — filesystem module API

**Learning version:**

```lp
@ Module functions mirror many top-level file statements.
use filesystem.

filesystem.create_folder("demo_dir").
filesystem.write("hello", "demo_dir/msg.txt").
names = filesystem.list("demo_dir").
print names.
filesystem.remove_folder("demo_dir").
```

**Professional version:**

```lp
use filesystem.

filesystem.create_folder("demo_dir").
filesystem.write("hello", "demo_dir/msg.txt").
print filesystem.list("demo_dir").
filesystem.remove_folder("demo_dir").
```

Run [examples/filesystem_demo.lp](../../examples/filesystem_demo.lp) for a full walkthrough.

---

## Common Mistakes

**Mistake:** Expecting `network` or `database` to perform real I/O

```lp
use network.
@ get "https://example.com"    @ NOT implemented
```

**Fix:** Use file-based workflows or wait for a future release. Check [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md).

---

**Mistake:** Forgetting the trailing dot on `use`

```lp
use filesystem    @ wrong — needs .
```

**Fix:**

```lp
use filesystem.
```

---

**Mistake:** Using nested imports

```lp
use http.server.    @ not supported
```

**Fix:** Import single-segment module names only in v0.2.0.

---

## Best Practices

- Import only modules you use — keeps programs readable ([25 — Imports](25-imports.md)).
- Prefer `filesystem` module functions when you need return values (e.g. `list` → list value).
- Treat `json`, `navigator`, `ai`, `network`, and `database` as placeholders until STATUS.md marks them stable.
- Pin dependencies in `langp.toml` when sharing projects ([28 — Package Manager](28-package-manager.md)).
- Run `lang check` after adding imports.

---

## Exercises

### Beginner

1. Import `math` and print `math.abs(10)`.
2. Import `filesystem` and print whether `"."` exists.
3. List all seven built-in module names from this chapter.
4. Explain the difference between a **stable** and **stub** module.
5. Run [examples/modules.lp](../../examples/modules.lp).

### Intermediate

1. Write a program that creates a folder, writes a file, lists the folder, then deletes everything.
2. Compare `read "file.txt"` (no import) vs. `filesystem.read("file.txt")`.
3. Import `json` and call `stringify` — observe stub behavior.
4. Read [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md) modules section and summarize in three bullets.
5. Add `use math.` to a calculator program and use `math.max(a, b)`.

### Advanced

1. Sketch how project modules in `src/` will work when full evaluation lands (read `interpreter/src/modules.rs`).
2. Design a program structure using only stdlib modules that works entirely offline.

---

## Summary

Lang.P ships **built-in modules** loaded with `use name.`. `filesystem` and `math` are usable today; `json` is minimal; `navigator`, `ai`, `network`, and `database` are stubs. Top-level file statements work without import. Project `.lp` modules are partial.

**Previous:** [23 — Static Members](23-static-members.md) · **Next:** [25 — Imports](25-imports.md)

**See also:** [25 — Imports](25-imports.md), [26 — Filesystem](26-filesystem.md), [28 — Package Manager](28-package-manager.md), [examples/modules.lp](../../examples/modules.lp), [Manual: Modules](../manual/13-modules.md)
