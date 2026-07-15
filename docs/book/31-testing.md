# Testing

## Introduction

**Testing** verifies that your program — or the Lang.P toolchain itself — behaves as expected. Application developers use **`lang test`** and **`assert`** in test files. Compiler contributors run **`cargo test`** in the Rust workspace.

**Why testing:** Catch regressions before users do. A green `lang test` in CI means every file in `tests/` at least parses and passes semantic analysis.

**When to test:** After adding features, before commits, and in continuous integration for team projects ([29 — Project Structure](29-project-structure.md)).

---

## Syntax

### lang test

From a project root with `langp.toml`:

```bash
lang test
```

Behavior:

1. Looks for **`tests/`** directory
2. Runs **`lang check`** on each **`*.lp`** file
3. Reports `ok` or `failed` per file

If `tests/` is missing, prints a message and exits successfully with zero tests.

### assert in programs

```lp
assert condition.
assert condition, "message when false".
```

Stops execution with an error if `condition` is falsy. Use in test files and development builds:

```lp
@ tests/math.lp
result = 2 + 2.
assert result == 4, "addition should work".
```

### tests/ folder convention

```
myapp/
    main.lp
    tests/
        smoke.lp
        filesystem.lp
        helpers.lp
```

Each file should be self-contained or only rely on builtins/stdlib imports.

### cargo test (compiler developers)

Inside the Lang.P repository:

```bash
cargo test
cargo test -p langp-interpreter
cargo test -p langp-semantic
```

Integration tests live in `interpreter/tests/`, `semantic/tests/`, etc. — see [TECH-STACK.md](../../docs/TECH-STACK.md).

| Audience | Command |
|----------|---------|
| App developer | `lang test` |
| Lang.P contributor | `cargo test` |

---

## Examples

### Simple — smoke test

**Learning version:**

`tests/smoke.lp`:

```lp
@ Minimal test — must pass lang check.
print "smoke test file".
assert true.
```

```bash
lang test
```

**Professional version:**

```lp
assert true, "smoke".
```

### Intermediate — test filesystem behavior

**Learning version:**

`tests/file_roundtrip.lp`:

```lp
use filesystem.

path = "tests/tmp-roundtrip.txt".
write "test data" to path.
text = read path.
assert text == "test data", "read should match write".
delete path.
```

**Professional version:**

```lp
write "x" to "tests/tmp.txt".
assert read "tests/tmp.txt" == "x".
delete "tests/tmp.txt".
```

Clean up files so repeated runs pass.

### Advanced — assert-driven development

**Learning version:**

```lp
function double(n),
    return n * 2.
..

assert double(3) == 6, "double(3) should be 6".
assert double(0) == 0.
print "All asserts passed.".
```

**Professional version:**

```lp
function double(n),
    return n * 2.
..

assert double(3) == 6.
```

Note: `lang test` runs **check**, not **run** — asserts execute only when you `lang run` the file. For v0.2.0, `lang test` validates syntax and semantics.

---

## Common Mistakes

**Mistake:** Expecting `lang test` to execute asserts at runtime

**Fix:** Today `lang test` calls **`lang check`**. Run `lang run tests/foo.lp` to execute asserts, or use check-only tests.

---

**Mistake:** Tests that leave files behind

```lp
write "x" to "tmp.txt".
@ no delete — next run may behave differently
```

**Fix:** Delete temp files at the end of each test file.

---

**Mistake:** Interactive tests in CI

```lp
name = input text "Name: ".    @ hangs in unattended CI
```

**Fix:** Avoid `input` in automated tests; use fixed values.

---

## Best Practices

- Keep **`tests/smoke.lp`** that always passes check — baseline for CI.
- Name tests by behavior: `filesystem_roundtrip.lp`, not `test1.lp`.
- Use **`assert`** with clear messages for future runtime test runners.
- Run **`lang test`** before **`lang build`** in scripts.
- Contributors: add Rust tests when fixing interpreter bugs ([30 — Debugging](30-debugging.md)).

---

## Exercises

### Beginner

1. Create `tests/smoke.lp` with `assert true` and run `lang test`.
2. Introduce a syntax error in a test file — observe `lang test` failure.
3. Explain the difference between `lang test` and `lang run`.
4. Write an assert that checks `1 + 1 == 2`.
5. Run `lang test` on a project without `tests/` — read the message.

### Intermediate

1. Add a test that imports `math` and asserts `math.abs(-5) == 5`.
2. Write a test file with intentional undefined name — fix until `lang test` passes.
3. Add `lang test` to a shell script that also runs `lang build`.
4. Read `langpm/src/lib.rs` `cmd_test` and summarize behavior.
5. Create two test files; verify both are discovered.

### Advanced

1. Run `cargo test -p langp-interpreter oop` and read one integration test.
2. Propose how Lang.P could run asserts in `lang test` in a future version.

---

## Summary

Use **`lang test`** to **`lang check`** every **`tests/*.lp`** file in a project. Write **`assert`** for conditions you want to guarantee. Compiler development uses **`cargo test`**. Combine testing with **`lang check`** habits from [32 — Best Practices](32-best-practices.md).

**Previous:** [30 — Debugging](30-debugging.md) · **Next:** [32 — Best Practices](32-best-practices.md)

**See also:** [27 — Error Handling](27-error-handling.md), [28 — Package Manager](28-package-manager.md), [29 — Project Structure](29-project-structure.md), [30 — Debugging](30-debugging.md), [interpreter/tests/](../../interpreter/tests/)
