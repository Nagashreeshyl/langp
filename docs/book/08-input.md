# Input

## Introduction

**Input** lets your program read data from the user. Lang.P provides typed `input` forms that read from the keyboard (stdin), validate the answer, and return the correct type — so you can use numbers in math without conversion.

**Why typed input:** Beginners should not parse strings manually. `input number` already gives you an `Int`; `input boolean` gives you a `Bool`.

**When to use input:** Interactive tools, quizzes, calculators, and any program that needs a human answer before continuing. Pair with [09 — Output](09-output.md) for prompts and responses.

---

## Syntax

```lp
name = input text "Enter your name: ".
name = input "Enter your name: ".          @ same as input text

age = input number "Enter your age: ".
price = input decimal "Enter price: ".
ok = input boolean "Continue? ".
secret = input password "Enter password: ".
```

| Form | Returns | Notes |
|------|---------|-------|
| `input text "prompt"` | `String` | Line of text |
| `input "prompt"` | `String` | Omit `text` keyword |
| `input number "prompt"` | `Int` | Validated integer |
| `input decimal "prompt"` | `Float` | Validated decimal |
| `input boolean "prompt"` | `Bool` | `true` or `false` |
| `input password "prompt"` | `String` | Masked entry |

Every `input` expression sits inside a statement that ends with `.`.

**Interactive note:** Input reads from **stdin** in the terminal. Run with `lang run program.lp` in an interactive shell. In CI or piped runs, provide answers on stdin (see [examples/calculator.lp](../../examples/calculator.lp) notes). GUI pickers (`input file`, `input date`, etc.) are **not** in v0.2.0 terminal mode.

---

## Examples

### Simple — ask for a name

**Learning version:**

```lp
@ Omit text — still returns a String.
name = input "What is your name? ".
print "Hello " with name with "!".
```

**Professional version:**

```lp
name = input "What is your name? ".
print "Hello " with name with "!".
```

### Intermediate — typed profile

**Learning version:**

```lp
@ Each input type matches its data type.
name = input text "Name: ".
age = input number "Age: ".
salary = input decimal "Salary: ".
online = input boolean "Online? ".

print "--- Profile ---".
print "Name: " with name.
print "Age: " with age.
print "Salary: " with salary.
print "Online: " with online.
```

**Professional version:**

```lp
name = input text "Name: ".
age = input number "Age: ".
salary = input decimal "Salary: ".
online = input boolean "Online? ".
print name with " — " with age with " — " with salary.
```

See [examples/input_demo.lp](../../examples/input_demo.lp).

### Advanced — password and immediate use

**Learning version:**

```lp
@ Password is String; length is safe to show, not the secret.
user = input text "Username: ".
secret = input password "Password: ".

print "Welcome " with user with ".".
print "Password length: " with len(secret).
```

**Professional version:**

```lp
user = input text "Username: ".
secret = input password "Password: ".
print "Welcome " with user with " (" with len(secret) with " chars)".
```

---

## Common Mistakes

**Mistake:** Wrong type annotation with typed input

```lp
age: String = input number "Age: ".
```

**Why:** `input number` returns `Int`.

**Fix:**

```lp
age: Int = input number "Age: ".
```

---

**Mistake:** Expecting input inside non-interactive runs

```lp
@ Program waits forever if stdin is empty
name = input "Name: ".
```

**Why:** `input` blocks until a line is available.

**Fix:** Run interactively, or pipe input: `echo "Naga" | lang run program.lp`.

---

**Mistake:** Using a variable as the prompt

```lp
prompt = "Age: ".
age = input number prompt.    @ not supported — prompt must be a string literal
```

**Fix:** Use a literal string in the `input` call.

---

**Mistake:** Treating `input boolean` as yes/no strings

```lp
@ User types true/false — not "yes"/"no" unless your runtime maps them
active = input boolean "Active? ".
```

**Fix:** Follow the prompt format your environment expects; test with `lang run`.

---

## Best Practices

- Write clear prompts ending with a space or `?` so the cursor lines up nicely.
- Pick the narrowest input type: `number` for ages, `decimal` for money, `boolean` for flags.
- Omit `text` when you want the shortest readable form for strings.
- Use `password` for secrets; never `print` the raw password value.
- Validate ranges in code after input (e.g. `if age < 0`) — [12 — Conditionals](12-conditionals.md).

---

## Exercises

### Beginner

1. Ask for a name with `input "..."` and greet the user.
2. Ask for an age with `input number` and print `age + 1` next year.
3. Ask `input boolean "Ready? "` and print the result.
4. Copy [examples/input_demo.lp](../../examples/input_demo.lp) and run it interactively.
5. Add an `@` comment explaining why prompts use string literals.

### Intermediate

1. Build a two-number calculator using only `input number` ([examples/calculator.lp](../../examples/calculator.lp)).
2. Collect name, age, and salary; print a one-line summary with `with`.
3. Use `input password` and print only `len(password)`.
4. Add `age: Int = input number "Age: "` with an explicit annotation.
5. Write a program that asks for decimal price and prints price with tax at 10%.

### Advanced

1. Create a login flow: username (`input text`), password (`input password`), print success without echoing the password.
2. Document how you would test an input program non-interactively using shell piping.

---

## Summary

`input` reads from stdin with optional type keywords: `text` (or omitted), `number`, `decimal`, `boolean`, and `password`. Each form returns the matching primitive type. Use string literal prompts and run interactively for best results.

**Previous:** [07 — Operators](07-operators.md) · **Next:** [09 — Output](09-output.md)

**See also:** [06 — Data Types](06-data-types.md), [12 — Conditionals](12-conditionals.md), [35 — Complete Projects](35-complete-projects.md), [examples/input_demo.lp](../../examples/input_demo.lp)
