# Operators

## Introduction

**Operators** are symbols and keywords that combine values — add numbers, compare scores, chain logic, and build text. Lang.P keeps the familiar math and comparison symbols while using English words for logic (`and`, `or`, `not`) and a dedicated keyword for strings (`with`).

**Why operators matter:** They are the vocabulary of computation. Using the right operator avoids runtime errors and keeps programs readable aloud.

**When to use them:** In every expression — assignments, `if` conditions ([12 — Conditionals](12-conditionals.md)), loop tests ([13 — Loops](13-loops.md)), and `print` output ([09 — Output](09-output.md)).

---

## Syntax

### Arithmetic (numbers)

| Operator | Meaning | Example |
|----------|---------|---------|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Remainder (integers) | `a % b` |

### Comparison

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

### Logical

| Operator | Meaning |
|----------|---------|
| `and` | Both conditions true |
| `or` | At least one true |
| `not` | Negation |

### Grouping

Parentheses control evaluation order:

```lp
result = (a + b) * c.
```

### String composition

Use **`with`** to join strings and values — **not** `+`:

```lp
message = "Hello " with name with "!".
print "Sum: " with (a + b).
```

---

## Examples

### Simple — calculator

**Learning version:**

```lp
@ Basic arithmetic on two numbers.
a = 10.
b = 3.

print "Sum: " with (a + b).
print "Difference: " with (a - b).
print "Product: " with (a * b).
print "Quotient: " with (a / b).
print "Remainder: " with (a % b).
```

**Professional version:**

```lp
a = 10.
b = 3.
print "Sum: " with (a + b).
print "Product: " with (a * b).
```

See [examples/calculator.lp](../../examples/calculator.lp).

### Intermediate — comparisons and logic

**Learning version:**

```lp
@ Gate access with multiple conditions.
age = input number "Age: ".
has_ticket = input boolean "Have a ticket? ".

if age >= 18 and has_ticket,
    print "Enter.".
otherwise,
    print "Denied.".
..
```

**Professional version:**

```lp
age = input number "Age: ".
has_ticket = input boolean "Have a ticket? ".

if age >= 18 and has_ticket,
    print "Enter.".
otherwise,
    print "Denied.".
..
```

### Advanced — mixed expressions in output

**Learning version:**

```lp
@ Compute inside parentheses, then display with with.
score = 87.
max_score = 100.
passed = score >= 60.

print "Score: " with score with "/" with max_score.
print "Passed: " with passed.
print "Percent: " with (score * 100 / max_score) with "%".
```

**Professional version:**

```lp
score = 87.
max_score = 100.
passed = score >= 60.
print "Score: " with score with "/" with max_score with " — " with passed.
```

---

## Common Mistakes

**Mistake:** Using `+` to join strings

```lp
@ msg = "Hi " + name    @ wrong
```

**Why:** `+` is defined for numeric addition in Lang.P v0.2.0.

**Fix:**

```lp
msg = "Hi " with name.
```

---

**Mistake:** Using `=` instead of `==` in conditions

```lp
if x = 5,    @ wrong — = is assignment
    print x.
..
```

**Fix:**

```lp
if x == 5,
    print x.
..
```

---

**Mistake:** Forgetting parentheses in `print`

```lp
print "Sum: " with a + b.    @ may parse unexpectedly
```

**Fix:**

```lp
print "Sum: " with (a + b).
```

---

**Mistake:** Division by zero

```lp
result = 10 / 0.    @ runtime error
```

**Fix:** Check the divisor first with an `if` ([12 — Conditionals](12-conditionals.md)).

---

## Best Practices

- Wrap arithmetic inside `print ... with (...)` in parentheses.
- Use `and` / `or` instead of chaining many separate `if` statements when conditions combine.
- Prefer `!=` and `==` for equality tests; do not rely on truthy integers.
- Keep complex conditions readable — extract sub-expressions into named variables.
- Remember: `with` chains left to right; insert literal strings between values as needed.

---

## Exercises

### Beginner

1. Compute `5 + 3`, `5 - 3`, `5 * 3`, `5 / 3`, and `5 % 3`; print each result.
2. Print whether `10 > 7` and whether `10 == 7`.
3. Fix a program that uses `"Hello " + "World"`.
4. Use `not false` in a print statement.
5. Run [examples/calculator.lp](../../examples/calculator.lp) with two numbers you choose.

### Intermediate

1. Write an `if` that prints "Even" when `n % 2 == 0` and "Odd" otherwise.
2. Combine `age >= 13 and age <= 19` for a "Teen" message.
3. Print a receipt line: `"Item: " with item with " — $" with price` using parentheses for tax.
4. Rewrite `(a + b) * 2` as two steps with a temporary variable.
5. Build a boolean `can_vote` from `age >= 18 or has_guardian_consent`.

### Advanced

1. Implement a min/max pair printer without functions — only comparisons and `if`.
2. Document operator precedence in your own words with five example expressions and expected results.

---

## Summary

Use `+ - * / %` on numbers, `== != < > <= >=` for comparisons, and `and or not` for logic. Parentheses control order. Join text with `with`, never `+`. Wrap math inside `print` with parentheses for clarity.

**Previous:** [06 — Data Types](06-data-types.md) · **Next:** [08 — Input](08-input.md)

**See also:** [10 — Strings](10-strings.md), [12 — Conditionals](12-conditionals.md), [09 — Output](09-output.md), [examples/calculator.lp](../../examples/calculator.lp)
