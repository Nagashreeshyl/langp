# 15 — Async Programming

**Status: Specification — not in v0.1 interpreter**

---

> Async features are specified for network and concurrent workloads. They are **not** available in the current runtime.

---

## `async function` (specification)

```lp
async function fetch_data(url),
    @ body
..
```

---

## `wait for` (specification)

```lp
@ Wait for an HTTP request to complete.
response = wait for get "https://api.example.com/data".
print response.
```

---

## Current alternative

Use synchronous file I/O and sequential logic in v0.1. See [16 — File System](16-filesystem.md).

---

## Next steps

- [17 — Networking](17-networking.md)
- [Concurrency (spec)](../spec/14-concurrency-async.md)
