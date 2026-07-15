# Imports

> **Beta** — `use` imports register module names for semantic analysis and load stdlib modules at runtime. Wildcard and nested imports are not supported.

## Introduction

An **import** brings a module's name into scope so you can write `filesystem.exists(...)` instead of repeating long paths. In Lang.P, imports use the **`use`** keyword followed by a module path ending in a dot.

**Why imports:** Clear dependencies at the top of a file, early error detection (`lang check` knows module names), and access to the standard library ([24 — Modules](24-modules.md)).

**When to use them:** At the start of any file that calls module functions — before other statements.

---

## Syntax

### Basic import

```lp
use filesystem.
use math.
use json.
```

Each import:

1. Must end with `.` (period)
2. Registers the **first segment** as a name in scope (e.g. `filesystem`)
3. Loads the module when the program runs

### Dot access after import

```lp
use filesystem.

exists = filesystem.exists("config.txt").
items = filesystem.list(".").
print exists.
```

The pattern is always **`ModuleName.export_name(...)`** — same dot notation as object fields ([19 — Objects](19-objects.md)), but modules hold native functions.

### Multiple imports

```lp
use filesystem.
use math.
use ai.

print math.min(3, 7).
print ai.version.
```

Imports are typically grouped at the top of the file, after comments.

### What is not allowed

| Syntax | Status |
|--------|--------|
| `use json.*.` | ❌ Wildcard forbidden by design |
| `use http.server.` | ❌ Nested paths not supported |
| `use filesystem` (no dot) | ❌ Parse error |

---

## Examples

### Simple — import and call

**Learning version:**

```lp
@ Register filesystem, then call exists.
use filesystem.

if filesystem.exists("notes.txt"),
    print "Found notes.".
otherwise,
    print "No notes file.".
..
```

**Professional version:**

```lp
use filesystem.

if filesystem.exists("notes.txt"),
    print "Found notes.".
otherwise,
    print "No notes file.".
..
```

### Intermediate — semantic registration

**Learning version:**

```lp
@ The analyzer knows 'ai' is a module after use ai.
use ai.

print "Module version: " with ai.version.
@ Without 'use ai.' above, lang check reports undefined name 'ai'.
```

**Professional version:**

```lp
use ai.
print ai.version.
```

The semantic analyzer test `use_import_makes_module_name_available` in `semantic/tests/use_imports.rs` verifies this behavior.

### Advanced — combining imports with top-level I/O

**Learning version:**

```lp
use filesystem.

@ Top-level write — no module prefix needed.
write "Hello" to "greeting.txt".

@ Module API for existence check and cleanup.
print filesystem.exists("greeting.txt").
delete "greeting.txt".
```

**Professional version:**

```lp
use filesystem.

write "Hello" to "greeting.txt".
print filesystem.exists("greeting.txt").
delete "greeting.txt".
```

See [26 — Filesystem](26-filesystem.md) for all file statements.

---

## Common Mistakes

**Mistake:** Using a module without importing it

```lp
print filesystem.exists("x").    @ undefined name 'filesystem'
```

**Fix:** Add `use filesystem.` at the top.

---

**Mistake:** Wildcard import

```lp
use math.*.    @ forbidden
```

**Fix:** Import the module and qualify calls: `math.abs(-1)`.

---

**Mistake:** Importing after executable statements

```lp
print "Starting".
use math.    @ works at runtime but poor style; some tools may warn later
```

**Fix:** Put all `use` lines at the top of the file.

---

## Best Practices

- One `use ModuleName.` per line for readability.
- Order imports: stdlib first, then project packages (when supported).
- Run `lang check` — undefined module names surface as semantic errors ([30 — Debugging](30-debugging.md)).
- Do not import stub modules unless you need them for future API shape or version checks.
- Document required imports in project README when sharing code.

---

## Exercises

### Beginner

1. Add `use math.` to a new file and print `math.abs(-100)`.
2. Fix a file that calls `filesystem.list(".")` without an import.
3. Write three valid `use` statements for stub modules.
4. Explain why `use json.*.` is rejected.
5. Run `lang check` on [examples/modules.lp](../../examples/modules.lp).

### Intermediate

1. Import both `filesystem` and `math` in one program that checks a file exists and prints the max of two numbers.
2. Write a comment block listing each import and why it is needed.
3. Trigger an `undefined name` error on purpose — fix it with `use`.
4. Compare dot access on modules vs. on object instances.
5. Read `semantic/tests/use_imports.rs` and explain what it tests.

### Advanced

1. Describe how circular imports are detected (`interpreter/src/modules.rs`).
2. Propose a project layout with multiple files and imports once project module eval is complete ([29 — Project Structure](29-project-structure.md)).

---

## Summary

Import modules with **`use name.`** — the trailing dot is required. Access exports via **`ModuleName.member`**. The semantic analyzer registers module names so `lang check` catches typos. Wildcards and nested imports are not supported in v0.2.0.

**Previous:** [24 — Modules](24-modules.md) · **Next:** [26 — Filesystem](26-filesystem.md)

**See also:** [24 — Modules](24-modules.md), [26 — Filesystem](26-filesystem.md), [28 — Package Manager](28-package-manager.md), [30 — Debugging](30-debugging.md), [Modules (spec)](../spec/11-modules-imports.md)
