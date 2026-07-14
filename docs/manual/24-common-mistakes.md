# 24 — Common Mistakes

**Status: Implemented (v0.1)**

---

## Missing period `.`

**Wrong**

```lp
print "Hello"
```

**Right**

```lp
print "Hello".
```

**Error:** `expected StmtEnd` / `E0202`

---

## Missing `..` on blocks

**Wrong**

```lp
if true,
    print "Yes".
.
```

**Right**

```lp
if true,
    print "Yes".
..
```

**Every block** — including `function`, `type`, and `enum` — closes with **`..`**, never a lone `.` on a dedented line.

```lp
@ Wrong — function closed with .
function f(),
    print "ok".
.

@ Right
function f(),
    print "ok".
..
```

---

## Wrong indentation

Blocks after `,` must indent 4 spaces:

**Wrong**

```lp
if ready,
print "Go".
..
```

**Right**

```lp
if ready,
    print "Go".
..
```

---

## Using `end` or `end.` (Python/Lua habit)

Lang.P does **not** use `end`, `end.`, or `}` to close blocks.

**Wrong**

```lp
function greet(name),
    print "Hello " with name.
end.
```

**Right**

```lp
function greet(name),
    print "Hello " with name.
..
```

---

## Using `+` instead of `with` for strings

**Wrong**

```lp
@ message = "Hello " + name
```

**Right**

```lp
message = "Hello " with name.
```

Use `+` only for numbers.

---

## Using undefined variables

**Wrong**

```lp
repeat 5 times as i,
    print i.
..
print i.
@ i does not exist here
```

**Right**

```lp
repeat 5 times as i,
    print i.
..
```

---

## Using unimplemented features

**Wrong (v0.1)**

```lp
use network.
data = get "https://example.com".
```

**Right (v0.1)**

Use file I/O or wait for networking chapter to leave specification status.

---

## Next steps

- [25 — Error Messages](25-error-messages.md)
- [How to Code — common errors](../guides/HOW-TO-CODE.md#common-errors-and-fixes)
