# Debugging

## Introduction

**Debugging** is the process of finding why a program fails or behaves unexpectedly. Lang.P gives you **`lang check`** for static errors before run time, **`langc --emit`** for inspecting tokens and AST, and readable **error codes** that point to the exact line and column.

**Why debugging skills matter:** Even careful programmers hit parse errors, undefined names, and runtime I/O failures. Knowing which tool to use saves hours.

**When to debug:** Whenever `lang run` fails, output is wrong, or the IDE shows squiggles — start with `lang check`, then narrow down.

---

## Syntax

### lang check

Analyze a file without executing it:

```bash
lang check myfile.lp
```

Use this first — it reports parse and semantic errors with codes and locations.

### langc --emit (compiler developers)

Low-level inspection via the compiler CLI:

```bash
langc myfile.lp --emit tokens
langc myfile.lp --emit ast
```

| Flag | Output |
|------|--------|
| `--emit tokens` | Token stream from the lexer |
| `--emit ast` | AST as JSON from parser |

Everyday users should prefer **`lang`**; use **`langc --emit`** when filing parser bugs or learning how Lang.P sees your code.

### Error message format

```
error[CODE]: message
  --> file.lp:LINE:COLUMN
```

Common codes (see [Manual: Error Messages](../manual/25-error-messages.md)):

| Code | Category | Typical cause |
|------|----------|---------------|
| **E0200** | Parse / unexpected token | Missing `.`, bad punctuation |
| **E0201** | Missing block close | Used `.` instead of `..` |
| **E0202** | Undefined name / stmt end | Typo, variable out of scope |

### Reading an E0200 (parse)

**What:** Statement did not end with `.`

```lp
print "Hello"    @ E0200 — expected StmtEnd
```

**Fix:**

```lp
print "Hello".
```

### Reading an E0202 (semantic)

**What:** Variable used before definition

```lp
print message.    @ E0202 — undefined name 'message'
```

**Fix:**

```lp
message = "Hi".
print message.
```

---

## Examples

### Simple — check before run

**Learning version:**

```bash
@ Always check first.
lang check examples/hello.lp
lang run examples/hello.lp
```

**Professional version:**

```bash
lang check main.lp && lang run main.lp
```

### Intermediate — fix a parse error

**Learning version:**

```lp
@ Broken — missing period.
if true,
    print "Yes"
..
```

Run:

```bash
lang check broken.lp
```

Output pattern:

```
error[E0200]: ...
  --> broken.lp:3:...
```

**Fix:**

```lp
if true,
    print "Yes".
..
```

**Professional version:** Same fix — add `.` after `"Yes"`.

### Advanced — inspect AST

**Learning version:**

```bash
@ See how the parser structures a try/catch block.
langc tests/conformance/parse/valid/try_catch.lp --emit ast
```

**Professional version:**

```bash
langc myfile.lp --emit tokens | head
```

Useful when reporting issues on GitHub with minimal reproduction.

---

## Common Mistakes

**Mistake:** Running repeatedly without reading the error line number

**Fix:** Go to `file.lp:LINE:COLUMN` shown in the output — fix that line first.

---

**Mistake:** Using `lang run` to find syntax errors

**Fix:** `lang check` is faster and shows **all** issues in one pass.

---

**Mistake:** Ignoring warnings

**Fix:** Warnings may become errors in future versions; clean them when shown.

---

## Best Practices

- **`lang check`** before **`lang run`** — every time ([32 — Best Practices](32-best-practices.md)).
- Read the **error code** (E0200 vs E0202) to choose parse vs. semantic fix.
- Compare broken code to [33 — Common Mistakes](33-common-mistakes.md).
- For runtime errors, wrap suspect code in **`try`/`catch`** and print `err` ([27 — Error Handling](27-error-handling.md)).
- Reduce the program to the smallest file that still fails before asking for help.

---

## Exercises

### Beginner

1. Run `lang check` on a valid file — confirm zero errors.
2. Remove a `.` from a print statement; run check and note the code.
3. Use an undefined variable; identify E0202 in output.
4. Fix a block closed with `.` instead of `..`.
5. Read [Manual: Error Messages](../manual/25-error-messages.md) sections on E0200 and E0202.

### Intermediate

1. Create a file with **two** errors; fix both using check output order.
2. Run `langc --emit tokens` on [examples/hello.lp](../../examples/hello.lp).
3. Document your personal debug checklist (check → read line → fix → re-check).
4. Trigger a runtime file error; catch and print it with try/catch.
5. Compare IDE diagnostics (if using VS Code extension) with CLI `lang check`.

### Advanced

1. Emit AST for a program with `use filesystem.` and locate the import node.
2. Write a troubleshooting guide for teammates linking to error codes and chapters 33 and 27.

---

## Summary

Start with **`lang check`** — errors show as **`error[CODE]`** with file location. **E0200** is usually parse/punctuation; **E0202** is often undefined names or missing `.`. Use **`langc --emit tokens`** or **`--emit ast`** for deep inspection. Full patterns live in [Manual: Error Messages](../manual/25-error-messages.md).

**Previous:** [29 — Project Structure](29-project-structure.md) · **Next:** [31 — Testing](31-testing.md)

**See also:** [27 — Error Handling](27-error-handling.md), [33 — Common Mistakes](33-common-mistakes.md), [34 — Language Reference](34-language-reference.md), [Manual: Error Messages](../manual/25-error-messages.md), [langc README](../../langc/README.md)
