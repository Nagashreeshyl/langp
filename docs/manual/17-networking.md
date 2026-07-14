# 17 — Networking

**Status: Specification — HTTP client/server not in v0.1**

---

> HTTP verbs and server APIs are defined in the specification. The v0.1 interpreter returns **not implemented** for `get`, `post`, etc.

---

## HTTP client (specification)

```lp
use network.

@ GET request
data = wait for get "https://api.example.com/users".

@ POST with body
response = wait for post "https://api.example.com/login" with body.
```

Planned functions: `get`, `post`, `put`, `patch`, `delete`.

---

## HTTP server (specification)

See [Introduction example](../spec/01-introduction.md) and [examples/server.lp](../../examples/server.lp) — **aspirational** until runtime ships.

---

## v0.1

Use file-based data exchange or external tools. Do not rely on `get`/`post` in production v0.1 code.

---

## Next steps

- [18 — JSON](18-json.md)
- [I/O and network (spec)](../spec/15-io-network.md)
