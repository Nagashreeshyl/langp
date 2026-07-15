# Error Handling

## Introduction

Programs fail — files go missing, users enter bad data, assertions fail. **Error handling** lets you **catch** failures, run recovery code, and always run **cleanup** in a `finally` block. Lang.P v0.2.0 implements **`try`**, **`catch`**, and **`finally`** in the interpreter.

**Why error handling:** Prevent one failed operation from crashing the whole program without explanation.

**When to use it:** Around file I/O ([26 — Filesystem](26-filesystem.md)), risky calculations, and anywhere you want a friendly message instead of a raw runtime error.

**Not implemented:** **`throw`** — you cannot raise custom errors yet. Use **`assert`** for debug checks instead.

---

## Syntax

### try / catch / finally

```lp
try,
    @ statements that may fail
catch err,
    @ err holds the error message (string)
finally,
    @ always runs (optional)
..
```

Rules:

- The **`try`**, **`catch`**, and **`finally`** branches open with `,` (comma).
- The **entire construct** closes with one **`..`** (double dot) — same as `if` chains ([12 — Conditionals](12-conditionals.md)).
- `catch err` binds the error message to `err` (a string).
- `finally` is optional.

### catch only (no finally)

```lp
try,
    data = read "config.txt".
catch err,
    print "Failed: " with err.
..
```

### assert builtin

```lp
assert condition.
assert condition, "helpful message".
```

If `condition` is falsy, the program stops with a user error. Optional second argument is the message.

| Feature | v0.2.0 |
|---------|--------|
| `try` / `catch` / `finally` | ✅ Implemented |
| `throw` | ❌ Not implemented |
| `assert` | ✅ Implemented |

---

## Examples

### Simple — catch a missing file

**Learning version:**

```lp
@ read on a missing file triggers catch.
try,
    text = read "missing.txt".
    print text.
catch err,
    print "Error reading file: " with err.
..
```

**Professional version:**

```lp
try,
    print read "missing.txt".
catch err,
    print "Error: " with err.
..
```

### Intermediate — finally for cleanup

**Learning version:**

```lp
@ finally runs whether try succeeds or catch runs.
try,
    write "working" to "temp.log".
    print read "temp.log".
catch err,
    print "Problem: " with err.
finally,
    print "Cleanup message.".
..
delete "temp.log".
```

**Professional version:**

```lp
try,
    write "working" to "temp.log".
catch err,
    print err.
finally,
    print "Done.".
..
```

### Advanced — assert for invariants

**Learning version:**

```lp
@ assert stops the program when condition is false.
count = 0.

function increment(ref count),
    count = count + 1.
..

increment(count).
assert count > 0, "count must be positive after increment".
print "Count: " with count.
```

**Professional version:**

```lp
count = 5.
assert count > 0, "count must be positive".
print count.
```

Use `assert` during development; use `try`/`catch` for expected runtime failures.

---

## Common Mistakes

**Mistake:** Using `throw` to raise errors

```lp
throw "Something went wrong".    @ not implemented
```

**Fix:** Let operations fail naturally and catch them, or use `assert` for logic bugs.

---

**Mistake:** Closing blocks with `.` instead of `..`

```lp
try,
    print "x".
catch err,
    print err.
.    @ wrong — need ..
```

**Fix:** One `..` closes the whole try/catch/finally chain ([33 — Common Mistakes](33-common-mistakes.md)).

---

**Mistake:** Expecting `catch` to receive a rich error object

```lp
catch err,
    print err.code.    @ err is a string, not an object
```

**Fix:** Treat `err` as text: `print "Error: " with err.`

---

## Best Practices

- Catch errors where you can **recover** or **explain** — not around every line.
- Use descriptive messages in `assert`'s second argument.
- Put resource cleanup in `finally` when you add more I/O features.
- Combine with `lang check` to catch undefined names before runtime ([30 — Debugging](30-debugging.md)).
- Read error output format in [Manual: Error Messages](../manual/25-error-messages.md).

---

## Exercises

### Beginner

1. Wrap `read "nope.txt"` in try/catch and print the error.
2. Add a `finally` block that prints `"Finished."`.
3. Write `assert true.` and run it.
4. Write `assert false, "test failure".` and observe the error.
5. Fix a try block closed with `.` instead of `..`.

### Intermediate

1. Read a file if it exists; otherwise write a default and read again (use [26 — Filesystem](26-filesystem.md)).
2. Nest an `if` inside `catch` to print different messages based on `err` text.
3. Use `assert` to validate that a list is non-empty before processing ([14 — Collections](14-collections.md)).
4. Compare behavior: program with vs. without try around a failing statement.
5. Read [tests/conformance/parse/valid/try_catch.lp](../../tests/conformance/parse/valid/try_catch.lp).

### Advanced

1. Design error handling for a config loader: try read, catch print default path, finally log timestamp.
2. Explain why `throw` is spec-only and what workaround patterns exist in v0.2.0.

---

## Summary

Use **`try`**, **`catch`**, and optional **`finally`** to handle runtime failures. The catch variable holds an **error message string**. **`assert`** checks conditions during development. **`throw`** is **not** implemented — catch errors from builtins and I/O instead.

**Previous:** [26 — Filesystem](26-filesystem.md) · **Next:** [28 — Package Manager](28-package-manager.md)

**See also:** [12 — Conditionals](12-conditionals.md), [26 — Filesystem](26-filesystem.md), [30 — Debugging](30-debugging.md), [33 — Common Mistakes](33-common-mistakes.md), [Manual: Error Handling](../manual/14-error-handling.md)
