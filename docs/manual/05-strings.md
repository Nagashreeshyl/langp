# 05 — Strings

**Status: Implemented (v0.1)**

---

## String literals

Strings use double quotes:

```lp
name = "Naga".
message = "Hello, Lang.P!".
```

Single quotes are for **characters**: `'a'`.

---

## Escaping

Use backslash inside double-quoted strings:

| Sequence | Meaning |
|----------|---------|
| `\"` | Double quote |
| `\\` | Backslash |
| `\n` | Newline |
| `\t` | Tab |

```lp
print "She said \"Hello\".".
print "Line one\nLine two".
```

---

## Multi-line strings

Lang.P does not use triple-quoted strings. Use one of:

```lp
@ Multiple print statements.
print "Line one.".
print "Line two.".

@ Or one string with \n.
print "Line one\nLine two".
```

---

## Interpolation with `with`

Lang.P does not use f-strings or `%` formatting. Join text with **`with`**:

### Learning version

```lp
@ Build a greeting from parts.
name = "Naga".
age = 18.

print "Hello " with name with ", you are " with age with " years old".
```

### Professional version

```lp
print "Hello " with name with ", age " with age.
```

---

## String operations

| Operation | Syntax |
|-----------|--------|
| Length | `len(text)` |
| To string | `to_string(value)` |

```lp
print len("hello").    @ 5
print to_string(42).
```

---

## Common mistake: `+` for strings

```lp
@ Wrong — + is for numbers
@ message = "Hello " + name

@ Correct
message = "Hello " with name.
```

---

## Next steps

- [06 — Variables](06-variables.md)
- [Expressions — with (spec)](../spec/06-expressions.md)
