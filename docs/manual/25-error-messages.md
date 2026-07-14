# 25 — Error Messages

**Status: Implemented (v0.1)**

---

## Design goals

Lang.P errors should tell beginners:

1. **What happened**
2. **Why it happened**
3. **How to fix it**

Format:

```
error[CODE]: message
  --> file.lp:LINE:COLUMN
```

Run `lang check file.lp` to see all issues without running the program.

---

## Parse errors (E0200)

### Unexpected token / expected `StmtEnd`

**What:** A statement did not end with `.`

**Why:** Lang.P requires every statement to end with a period.

**Fix:** Add `.` at the end of the line.

```lp
@ Wrong
print "Hello"

@ Fixed
print "Hello".
```

---

### Expected `RParen` / expected `LParen`

**What:** Parentheses are unbalanced in an expression.

**Why:** `(num1 + num2` is missing `)`.

**Fix:** Close all parentheses.

```lp
print "Sum: " with (num1 + num2).
```

---

### Expected `..`

**What:** A block was closed with `.` instead of `..`.

**Why:** Grammar Freeze v1.0 — every block closes with `..`.

**Fix:**

```lp
if true,
    print "Yes".
..
```

---

## Semantic errors (E0202+)

### `undefined name 'x'`

**What:** Variable used before definition.

**Why:** No assignment or binding named `x` is visible.

**Fix:** Assign first or check spelling.

```lp
name = input text "Name: ".
print "Hello " with name.
```

---

### Loop variable outside block

**What:** `undefined name 'i'` after a loop.

**Why:** `i` from `repeat N times as i` exists only inside the block.

**Fix:** Use `i` only before `..`.

---

## Warnings

Warnings (yellow in IDE) do not stop `lang run` unless upgraded to errors. Fix them to keep code clean.

---

## Getting help

```bash
lang check myfile.lp
```

1. Read the error code in brackets
2. Go to the line/column shown
3. Check [24 — Common Mistakes](24-common-mistakes.md)

---

## Error code reference

| Code | Category |
|------|----------|
| E0200 | Parse / unexpected token |
| E0201 | Missing block close |
| E0202 | Undefined name / stmt end |
| E0203 | Type mismatch (future) |

---

## Next steps

- [Manual index](README.md)
- [Language Reference](../guides/LANGUAGE-REFERENCE.md)
