# 18 — JSON

**Status: Specification — `use json` module planned**

---

> JSON encode/decode via `use json.` is specified in [Standard library](../spec/16-standard-library.md). Not available in v0.1.

---

## Intended syntax (specification)

```lp
use json.

@ Parse JSON text
data = json.parse(text).

@ Serialize to JSON
text = json.stringify(data).
```

---

## v0.1 workaround

Store structured data in dictionaries and print with `with`, or read/write JSON files as plain text until the module ships.

---

## Next steps

- [19 — Collections](19-collections.md)
- [Standard library (spec)](../spec/16-standard-library.md)
