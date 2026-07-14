# 11 — Output

**Status: Implemented (v0.1)**

---

## `print`

Prints to the terminal. Every `print` statement ends with `.`.

### Learning version

```lp
@ Simple message.
print "Hello, Lang.P!".

@ Combine text and values.
name = "Naga".
print "Hello " with name with "!".
```

### Professional version

```lp
print "Processing...".
print "Result: " with result.
```

---

## Formatting with `with`

Each `with` appends one value (converted to text):

```lp
print "Name: " with name with ", Age: " with age with ".".
```

For arithmetic in output, evaluate first with parentheses:

```lp
print "Sum: " with (a + b).
print "Average: " with (total / count).
```

---

## `print inline`

The specification includes an inline variant for partial-line output:

```lp
print inline "Loading".
print inline ".".
print inline ".".
@ Later: println equivalent when supported.
```

---

## Next steps

- [14 — Error Handling](14-error-handling.md)
- [Statements — print (spec)](../spec/07-statements.md)
