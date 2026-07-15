# Project Structure

## Introduction

A consistent **project structure** helps you and others find the entry point, source files, tests, and dependencies quickly. Lang.P projects created with **`lang init`** follow a simple, predictable layout designed for v0.2.0 tooling.

**Why structure matters:** `lang build`, `lang test`, and the module loader look for specific files and folders. A standard layout means commands work without extra configuration.

**When to set it up:** At project start — run `lang init myapp` before writing application code ([28 — Package Manager](28-package-manager.md)).

---

## Syntax

There is no special syntax for layout — it is **convention plus manifest**. The manifest `entry` field tells tools which file to run:

```toml
[package]
name = "myapp"
version = "0.1.0"
entry = "main.lp"
```

### Standard layout (v0.2.0)

```
myapp/
    langp.toml       @ package manifest — name, version, dependencies
    langp.lock       @ pinned dependency versions (after lang install)
    main.lp          @ entry point — start here
    src/             @ future project modules (*.lp) — partial support
    tests/           @ test programs checked by lang test
```

Optional as the project grows:

```
myapp/
    README.md
    examples/
    .gitignore
```

### What each part does

| Path | Role |
|------|------|
| `main.lp` | Default program entry; run with `lang run main.lp` or `lang build` |
| `langp.toml` | Declares package name, version, entry, `[dependencies]` |
| `langp.lock` | Exact resolved versions — commit to Git |
| `src/` | Placeholder for multi-file modules (`src/helpers.lp`) — evaluation partial |
| `tests/` | `.lp` files validated by `lang test` ([31 — Testing](31-testing.md)) |

### Module resolution (partial)

The interpreter searches for modules in:

1. Built-in stdlib (`filesystem`, `math`, …)
2. `src/<name>.lp`
3. `<name>.lp` in project root
4. `~/.cache/langp/packages/` (installed packages)

Project `.lp` files are found but **full per-file evaluation is not enabled** yet — see [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md).

---

## Examples

### Simple — default init layout

**Learning version:**

```bash
lang init myapp
```

Resulting tree:

```
myapp/
    langp.toml
    main.lp
    src/
    tests/
```

**Professional version:**

Same — `lang init` is the canonical scaffold.

### Intermediate — organized main.lp

**Learning version:**

```lp
@ main.lp — entry point for myapp
@ Keep high-level flow here; helpers go in functions or future src/ modules.

function greet(name),
    print "Hello " with name with "!".
..

greet("Lang.P").
```

**Professional version:**

```lp
function greet(name),
    print "Hello " with name with "!".
..

greet("Lang.P").
```

### Advanced — tests folder

**Learning version:**

`tests/greet_test.lp`:

```lp
@ Checked by lang test — syntax and semantics must pass.
assert true, "project tests run".
```

```bash
lang test
```

**Professional version:**

```bash
lang test && lang run main.lp
```

---

## Common Mistakes

**Mistake:** Putting the entry point only in `src/` without updating manifest

```toml
entry = "main.lp"    @ lang build still checks main.lp, not src/app.lp
```

**Fix:** Set `entry = "src/app.lp"` in `langp.toml` if the entry moves.

---

**Mistake:** Expecting `src/helpers.lp` to auto-import

```lp
@ helpers.lp exists in src/ but nothing imports it yet
```

**Fix:** Use `use helpers.` when project module eval is complete; until then, keep code in `main.lp` or use stdlib modules.

---

**Mistake:** Empty `tests/` and assuming `lang test` failed

```bash
lang test    @ prints "no tests/ directory" or "0 test file(s)" — not always an error
```

**Fix:** Add at least one `tests/*.lp` file for meaningful CI.

---

## Best Practices

- One **`main.lp`** entry with clear top-to-bottom flow.
- Put **`use`** imports at the top of each file ([25 — Imports](25-imports.md)).
- Add **`tests/`** early — even a single smoke test helps ([31 — Testing](31-testing.md)).
- Document the project in **`README.md`** with install and run commands.
- Run **`lang check main.lp`** before every commit ([32 — Best Practices](32-best-practices.md)).

---

## Exercises

### Beginner

1. Run `lang init layout-demo` and draw the folder tree on paper.
2. Change the greeting in `main.lp` and run it.
3. List the four core files/folders every init project gets.
4. Open `langp.toml` and explain each field under `[package]`.
5. Run `lang build` in your project.

### Intermediate

1. Add `lang install filesystem` and use it from `main.lp`.
2. Create `tests/smoke.lp` and run `lang test`.
3. Add a `function` in `main.lp` and call it from the top level.
4. Write a README section: Prerequisites, Run, Test.
5. Compare this layout to [28 — Package Manager](28-package-manager.md) manifest fields.

### Advanced

1. Plan how you would split `main.lp` into `src/` modules when eval lands.
2. Design a `.gitignore` for Lang.P projects (lock file, cache, local temp files).

---

## Summary

A Lang.P app typically has **`main.lp`**, **`langp.toml`**, **`langp.lock`**, **`src/`**, and **`tests/`**. **`lang init myapp`** creates this layout. The manifest **`entry`** field names the main file. Multi-file **`src/`** modules are **partial** in v0.2.0.

**Previous:** [28 — Package Manager](28-package-manager.md) · **Next:** [30 — Debugging](30-debugging.md)

**See also:** [25 — Imports](25-imports.md), [28 — Package Manager](28-package-manager.md), [31 — Testing](31-testing.md), [32 — Best Practices](32-best-practices.md), [Manual: Modules](../manual/13-modules.md)
