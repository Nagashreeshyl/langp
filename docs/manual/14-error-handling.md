# 14 — Error Handling

**Status: Implemented (v0.1)**

---

## `try` / `catch` / `finally`

### Learning version

```lp
@ Handle errors gracefully.
try,
    result = risky_operation().
catch err,
    print "Error: " with err.
finally,
    print "Cleanup complete.".
..
```

### Professional version

```lp
try,
    data = read "config.txt".
catch err,
    print "Failed to read config: " with err.
..
```

Each inner block closes according to its rules (`try` branches use `,`; the outer `try` closes with `..`).

---

## `throw` (specification)

Raising errors with `throw` is defined in [Error handling (spec)](../spec/13-error-handling.md). Support in v0.1 may be limited — prefer `assert` for debug checks:

```lp
assert count > 0, "count must be positive".
```

---

## Next steps

- [16 — File System](16-filesystem.md)
- [Error handling (spec)](../spec/13-error-handling.md)
