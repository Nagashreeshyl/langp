# Grammar Freeze v1.0 — Consistency Audit Report

**Date:** 2026-07-15  
**Status:** Complete  
**Official grammar:** [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md)

---

## Summary

Lang.P Grammar Freeze v1.0 unifies block syntax:

| Rule | Token |
|------|-------|
| Statement ends | `.` |
| Block opens | `,` |
| Block closes | `..` (all blocks, no exceptions) |

The parser, conformance tests, EBNF, manual, spec, guides, examples, and IDE snippets were updated to match.

---

## Parser changes

| File | Change |
|------|--------|
| `parser/src/parser.rs` | Removed `BlockClose::StatementEnd`; all blocks require `..`; added `expect_block_close()`; removed `.` fallback on `if`/`try` |

---

## New specification documents

| File | Purpose |
|------|---------|
| `docs/spec/GRAMMAR-FREEZE-v1.md` | Single source of truth for frozen syntax |
| `docs/spec/DESIGN-DECISIONS.md` | Rationale for `.`, `,`, `..`, `with`, `@`, etc. |
| `docs/spec/LANGUAGE-PHILOSOPHY.md` | Design principles |
| `docs/spec/KEYWORDS.md` | Reserved keywords with examples |

---

## Conformance tests

| File | Change |
|------|--------|
| `tests/conformance/parse/valid/*.lp` | Block closers `.` → `..` |
| `tests/conformance/parse/invalid/dot_block_close.lp` | **New** — rejects lone `.` block close |
| `parser/tests/conformance.rs` | Added `invalid_dot_block_close_rejected` test |

---

## Examples (`.lp`)

| File | Fixed |
|------|-------|
| `examples/hello.lp` | Function closes with `..` |
| `examples/agent.lp` | All blocks `..` |
| `examples/browser.lp` | All blocks `..` |
| `examples/server.lp` | All blocks `..` |

---

## Documentation updated (bulk)

Automated pass converted standalone `.` block closers to `..` in ` ```lp ` code blocks (excluding intentional **Wrong** examples):

### Manual (`docs/manual/`)

- `02-philosophy.md` — unified block rule
- `07-functions.md` — function close text + examples
- `12-classes.md` — type close text + examples
- `24-common-mistakes.md` — removed dual-rule section
- `25-error-messages.md` — block error applies to all blocks
- `README.md` — punctuation table

### Spec (`docs/spec/`)

- `01-introduction.md` through `22-compatibility-versioning.md` — examples
- `LANGP-SPEC.md` — ~200 block closers fixed
- `09-control-flow.md` — nested blocks
- `README.md` — links to freeze docs

### Guides

- `docs/guides/HOW-TO-CODE.md`
- `docs/guides/LANGUAGE-REFERENCE.md`

### Grammar

- `docs/grammar/03-syntactic-grammar.ebnf` — `function_decl`, `type_decl`, `enum_decl` use `BLOCK_CLOSE`
- `docs/grammar/LANGP-EBNF.md` — matching prose grammar

### Other

- `README.md`
- `examples/README.md`
- `tests/conformance/parse/README.md`
- `tests/README.md`
- `ai/README.md`, `navigator/README.md`
- `.cursor/rules/langp.mdc`

---

## IDE / tooling

| File | Change |
|------|--------|
| `editors/vscode-langp/snippets/langp.json` | `function` and `type` snippets end with `..` |
| `editors/langp-manifest.json` | Function snippet uses `..` |

---

## Inconsistencies fixed

1. **Dual block-close rules** — docs said functions/types use `.`, control flow uses `..` → **one rule: `..`**
2. **Parser accepted `.` as block close** — fallback removed; `..` required
3. **EBNF mismatch** — `function_decl` ended with `STMT_END` → `BLOCK_CLOSE`
4. **HOW-TO-CODE** — "close with `.`" for functions → `..`
5. **hello.lp canonical example** — function used `.` → `..`
6. **IDE snippets** — generated invalid function syntax

---

## Intentional exceptions

| Location | Why |
|----------|-----|
| `docs/manual/24-common-mistakes.md` | **Wrong** examples still show lone `.` to teach the error |
| `tests/conformance/parse/invalid/dot_block_close.lp` | Must fail parse |

---

## Verification

```bash
cargo test -p langp-parser   # 6 tests pass
```

Audit script checks:

- ✓ No valid `.lp` files contain lone `.` block closers (except invalid fixtures)
- ✓ No documentation claims dual close rules (except wrong-example sections)
- ✓ All valid conformance fixtures parse
- ✓ `dot_block_close.lp` rejected

---

## Not in scope (future pass)

- `docs/spec/LANGP-SPEC.md` inline prose may still reference "declaration close" in chapter text outside code blocks — search if needed
- VS Code extension copies under `.vscode/extensions/` (local install artifacts) not updated
- Interpreter/runtime behavior unchanged (parse-only freeze)

---

## Amendment policy

After v1.0, syntax changes require a new grammar version per [Chapter 22](22-compatibility-versioning.md). Update this audit when amending.
