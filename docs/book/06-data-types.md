# Data Types

## Introduction

Every value in Lang.P has a **type** — a category that tells the compiler what operations are allowed. Lang.P is statically typed: types are checked before your program runs.

**Why types:** They catch mistakes early (`"hello" + 5` is wrong) and make code self-documenting. You rarely need to write types explicitly; the compiler infers them from literals and expressions.

**When to think about types:** Whenever you assign, compare, print, or pass data to [08 — Input](08-input.md) or [11 — Functions](11-functions.md). This chapter covers the five core types in v0.2.0.

---

## Syntax

### Primitive types (v0.2.0)

| Type | Description | Literal examples |
|------|-------------|------------------|
| `Int` | Whole numbers | `42`, `-7`, `0` |
| `Float` | Decimal numbers | `3.14`, `45000.50`, `-0.5` |
| `String` | Text (UTF-8) | `"hello"`, `"Lang.P"` |
| `Bool` | True or false | `true`, `false` |
| `Null` | Absence of value | `null` |

### Type annotations (optional)

```lp
count: Int = 0.
rate: Float = 3.14.
name: String = "Naga".
online: Bool = true.
missing: Null = null.
```

Annotations are **validated semantically** — the compiler rejects mismatches:

```lp
@ Error: 18 is Int, not String
age: String = 18.
```

### Inference (no annotation needed)

```lp
age = 18.              @ inferred Int
salary = 45000.50.     @ inferred Float
name = "Naga".         @ inferred String
online = true.         @ inferred Bool
```

---

## Examples

### Simple — literals

**Learning version:**

```lp
@ Each literal picks its type automatically.
whole = 42.
decimal = 3.14.
text = "Lang.P".
flag = true.
empty = null.

print whole with " " with decimal with " " with text.
```

**Professional version:**

```lp
whole = 42.
decimal = 3.14.
text = "Lang.P".
flag = true.
empty = null.
```

### Intermediate — typed variables with input

**Learning version:**

```lp
@ input forms return the matching type.
name = input text "Name: ".
age = input number "Age: ".
salary = input decimal "Salary: ".
active = input boolean "Active? ".

print name with " is " with age with " years old.".
print "Salary: " with salary.
print "Active: " with active.
```

**Professional version:**

```lp
name = input text "Name: ".
age = input number "Age: ".
salary = input decimal "Salary: ".
active = input boolean "Active? ".
print name with " — " with age with " — " with salary.
```

See [examples/input_demo.lp](../../examples/input_demo.lp).

### Advanced — annotations for clarity

**Learning version:**

```lp
@ Explicit types document a small API.
function describe(score: Int, passed: Bool) -> String,
    if passed,
        return "Score " with to_string(score) with " — Pass".
    otherwise,
        return "Score " with to_string(score) with " — Fail".
    ..
..

message = describe(85, true).
print message.
```

**Professional version:**

```lp
function describe(score: Int, passed: Bool) -> String,
    if passed,
        return "Score " with to_string(score) with " — Pass".
    otherwise,
        return "Score " with to_string(score) with " — Fail".
    ..
..

print describe(85, true).
```

---

## Common Mistakes

**Mistake:** Quoting numbers

```lp
age = "18".    @ String, not Int — math will fail
```

**Why:** Double quotes create `String`; unquoted digits create `Int` or `Float`.

**Fix:**

```lp
age = 18.
```

---

**Mistake:** Using `1` and `0` instead of `true` / `false`

```lp
online = 1.    @ Int, not Bool
```

**Fix:**

```lp
online = true.
```

---

**Mistake:** Wrong annotation for `input`

```lp
age: String = input number "Age: ".    @ compile error
```

**Why:** `input number` returns `Int`, not `String`.

**Fix:**

```lp
age: Int = input number "Age: ".
```

---

**Mistake:** Expecting `+` to join strings

```lp
@ greeting = "Hello " + name    @ wrong — + is for numbers
```

**Fix:** Use `with` ([10 — Strings](10-strings.md)):

```lp
greeting = "Hello " with name.
```

---

## Best Practices

- Let inference handle local variables; annotate function parameters and return types when they help readers.
- Use `input number`, `input decimal`, and `input boolean` so types match without manual conversion.
- Use `null` only when a value is genuinely missing; prefer clear defaults when possible.
- Use `to_string` when you need a `String` from a number for display.
- Reserve `PascalCase` for type names ([18 — Types and OOP](18-type-oop.md)), not variables.

---

## Exercises

### Beginner

1. Create one variable of each primitive type and print each with `print`.
2. Predict the type of `x = 100.` and `y = 100.0.` — run `lang check` to confirm.
3. Fix `name: Int = "Naga".` so it compiles.
4. Set `logged_in = false.` and print it.
5. Write `null` into a variable `result` and print `to_string(result)`.

### Intermediate

1. Build a profile using all five `input` types from [08 — Input](08-input.md).
2. Add explicit type annotations to every variable in [examples/calculator.lp](../../examples/calculator.lp).
3. Write a function `is_adult(age: Int) -> Bool` that returns `true` when `age >= 18`.
4. Explain in an `@` comment why `"3" + 2` would fail but `3 + 2` works.
5. Convert inferred assignments to annotated form and back — ensure both pass `lang check`.

### Advanced

1. Write `describe_value` that accepts any primitive via separate overloaded-style functions (one per type) and returns a descriptive string.
2. List three situations where explicit annotations help more than inference, with short code samples.

---

## Summary

Lang.P's core types are `Int`, `Float`, `String`, `Bool`, and `Null`. Literals and `input` forms infer types automatically; optional annotations like `x: Int = 1.` are checked at compile time. Use the right literal form and typed `input` to avoid conversion errors.

**Previous:** [05 — Variables](05-variables.md) · **Next:** [07 — Operators](07-operators.md)

**See also:** [08 — Input](08-input.md), [10 — Strings](10-strings.md), [14 — Collections Overview](14-collections.md), [34 — Language Reference](34-language-reference.md)
