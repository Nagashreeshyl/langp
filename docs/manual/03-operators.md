# 03 — Operators

**Status: Implemented (v0.1)**

---

## Arithmetic

| Operator | Meaning | Example |
|----------|---------|---------|
| `+` | Add | `a + b` |
| `-` | Subtract | `a - b` |
| `*` | Multiply | `a * b` |
| `/` | Divide | `a / b` |
| `%` | Remainder (integers) | `a % b` |
| `**` | Power | `a ** b` |
| `//` | Integer division | `a // b` |

### Learning version

```lp
@ Simple calculator.
a = input number "First number: ".
b = input number "Second number: ".

print "Sum: " with (a + b).
print "Product: " with (a * b).
```

### Professional version

```lp
a = input number "First number: ".
b = input number "Second number: ".
print "Sum: " with (a + b).
print "Product: " with (a * b).
```

Use parentheses to control order: `(a + b) * c`.

**Note:** `+` adds numbers. It does **not** join strings — use `with` (see [05 — Strings](05-strings.md)).

---

## Comparison

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

```lp
if score >= 60,
    print "Pass".
otherwise,
    print "Fail".
..
```

---

## Logical

| Operator | Meaning |
|----------|---------|
| `and` | Both true |
| `or` | Either true |
| `not` | Negation |

```lp
if age >= 18 and has_id,
    print "Allowed".
..
```

---

## Assignment

| Form | Meaning |
|------|---------|
| `=` | Assign |
| `+=` | Add and assign |
| `-=` | Subtract and assign |
| `*=` | Multiply and assign |
| `/=` | Divide and assign |

```lp
count = 10.
count = count + 1.
@ Or when compound assign is supported:
@ count += 1.
```

---

## Composition (`with`)

| Operator | Purpose |
|----------|---------|
| `with` | Join strings and values for display |

```lp
print "Hello " with name with "!" .
```

`with` is not a general binary operator — it chains in expressions and `print` statements.

---

## Unary

| Operator | Meaning |
|----------|---------|
| `-` | Negate number |
| `not` | Logical not |

---

## Operator precedence (summary)

Highest to lowest:

1. Parentheses `( … )`
2. Unary `-`, `not`
3. `*`, `/`, `%`, `**`
4. `+`, `-` (numeric)
5. Comparisons (`==`, `<`, …)
6. `and`
7. `or`
8. `with` (string composition, separate chain)

---

## Next steps

- [04 — Data Types](04-datatypes.md)
- [Expressions (spec)](../spec/06-expressions.md)
