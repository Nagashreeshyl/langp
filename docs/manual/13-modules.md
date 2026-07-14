# 13 — Modules

**Status: Specification — `use` imports not yet in v0.1 interpreter**

---

> Module imports are defined in the language specification. The v0.1 toolchain runs single-file programs. Multi-file `use` is **planned**.

---

## Import syntax (official)

```lp
use navigator.
use ai.
use database.
use network.
use filesystem.
```

Submodules use dot paths:

```lp
use json.
use http.server.
```

---

## Intended project layout (specification)

```
myapp/
    lang.toml          @ project manifest (planned)
    main.lp            @ entry point
    utils/
        helpers.lp
```

---

## Exports

The specification defines visibility rules in [Modules (spec)](../spec/11-modules-imports.md). Public symbols are exported by default at module top level unless marked private in future releases.

---

## v0.1 workaround

Put all code in one `.lp` file or concatenate manually. Example layout:

```
myproject/
    main.lp
    @ helpers.lp — copy/paste or wait for use support
```

---

## Next steps

- [14 — Error Handling](14-error-handling.md)
- [Modules (spec)](../spec/11-modules-imports.md)
