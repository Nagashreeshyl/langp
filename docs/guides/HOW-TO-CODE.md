# How to Code in Lang.P

A complete beginner's guide to writing Lang.P programs (`.lp` files).

For the full 25-chapter manual (operators, types, filesystem, best practices, error messages), see **[Lang.P Manual](../manual/README.md)**. For a quick list of working commands, see **[Language Reference v0.1](LANGUAGE-REFERENCE.md)**.

---

## Quick start

```bash
# Install (one line)
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh

# Run a program
lang run examples/hello.lp

# Uninstall (one line)
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.sh | sh
```

Reload Cursor/VS Code after install for syntax colors and autocomplete.

---

## The three rules (memorize these)

Lang.P reads like English sentences. Three punctuation rules govern everything:

| Symbol | Name | Meaning |
|--------|------|---------|
| **`.`** | Statement end | Every instruction ends with a period (like a sentence) |
| **`,`** | Block open | After `if`, `repeat`, `function`, etc. — next lines are indented |
| **`..`** | Block close | Ends an indented block (like `}` in other languages) |

```lp
@ This is a comment — starts with @
print "Hello".          @ ← period ends the statement
```

**Wrong:** `print "Hello"` — missing `.`  
**Right:** `print "Hello".`

---

## Comments

```lp
@ This is a single-line comment.
@ Use @ at the start of the line.
```

---

## Variables and assignment

```lp
name = "Naga".
age = 25.
pi = 3.14.
active = true.
```

Every assignment ends with `.`

---

## Printing output

Use `print` with `with` to combine text and values (never use `+` for strings):

```lp
print "Hello World!".
print "Hello " with name with "!".
print "Age: " with age.
```

---

## Functions

Define with `function`, open body with `,`, close with `..`:

```lp
function greet(name),
    print "Hello " with name with "!".
..

greet("World").
greet("Naga").
```

### Functions with multiple parameters

```lp
function add(a, b),
    print a with " + " with b with " = " with (a + b).
..

add(3, 5).
```

---

## Conditionals

### if / otherwise if / otherwise

```lp
age = 20.

if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

### Inline if (single line)

```lp
status = if score >= 60, "Pass", otherwise, "Fail".
```

---

## Loops

### repeat N times — like Python `for _ in range(N)`

```lp
repeat 5 times,
    print "Hello".
..
```

### repeat with counter — like Python `for i in range(N)`

```lp
repeat 5 times as i,
    print "i = " with i.
..
```

`i` goes from **0** to **4** (zero-indexed).  
**Important:** `i` only exists **inside** the repeat block. You cannot use it after `..`.

### while loop

```lp
count = 5.
while count > 0,
    print count.
    count = count - 1.
..
```

### repeat forever — like `while True`

```lp
repeat forever,
    print "Running...".
    break.          @ use break to exit
..
```

### for loop over a list

```lp
@ Iterate over a list literal.
items = ["apple", "banana", "cherry"].
for item in items,
    print item.
..
```

See `examples/loops.lp` for more loop examples. For the full manual, see [08 — Loops](../manual/08-loops.md).

---

## Blocks and indentation

When a line ends with `,`, the next lines are indented (4 spaces). Close with `..` on its own line:

```lp
function demo(),
    if true,
        print "Inside if".
    ..
    print "After if".
..

demo().
```

In Cursor/VS Code, pressing **Enter** after a `,` line auto-indents. Typing `..` auto-de-dents.

---

## Types

| Lang.P | Description | Example |
|--------|-------------|---------|
| `Int` | Whole numbers | `42` |
| `Float` | Decimals | `3.14` |
| `Bool` | true/false | `true` |
| `String` | Text | `"hello"` |
| `Char` | Single character | `'a'` |
| `List` | Ordered collection | `[1, 2, 3]` |
| `null` | No value | `null` |

---

## Input from user

```lp
name = input text "What is your name?".
print "Hello " with name with "!".
```

Typed variants: `input number`, `input decimal`, `input boolean`, `input password`.

---

## String composition

Use `with` to join **text**. Use `+` `-` `*` `/` for **numbers** only.

```lp
@ Strings — use with
message = "Hello " with name with "!".

@ Numbers — use + - * /
print "Sum: " with (num1 + num2).
print "Total: " with (a + b + c).
```

```lp
@ Wrong for strings — do not use + on text
@ message = "Hello " + name
```

See [Language Reference — Math](LANGUAGE-REFERENCE.md#math-and-operators) for all operators.

---

## Math (calculator)

```lp
num1 = input number "Enter a number: ".
num2 = input number "Enter another number: ".

print "Sum: " with (num1 + num2).
print "Difference: " with (num1 - num2).
print "Product: " with (num1 * num2).
print "Quotient: " with (num1 / num2).
```

Full example: `examples/calculator.lp`

---

## Error handling

```lp
try,
    result = risky_operation().
catch err,
    print "Error: " with err.
finally,
    print "Cleanup done.".
..
```

---

## Common errors and fixes

| Error | Cause | Fix |
|-------|-------|-----|
| `expected StmtEnd` | Missing `.` at end of line | Add `.` |
| `undefined variable 'i'` | Using loop variable outside its block | Only use `i` inside `repeat ... ..` |
| `undefined name 'X'` | Variable not defined yet | Assign before use |
| `semantic errors prevent execution` | Type/name errors | Run `lang check file.lp` |

---

## Commands cheat sheet

| Command | What it does |
|---------|--------------|
| `lang run file.lp` | Run a program |
| `lang file.lp` | Same as above |
| `lang check file.lp` | Check for errors without running |
| `lang --version` | Show version |
| `langc --emit ast file.lp` | Debug: show syntax tree |

**All built-in keywords and functions:** [Language Reference (v0.1)](LANGUAGE-REFERENCE.md) — print, input, len, if, repeat, file I/O, and more.

---

## Project layout

```
myproject/
    main.lp          @ your program
    utils.lp         @ optional other files (imports coming soon)
```

---

## Example programs

| File | What it teaches |
|------|-----------------|
| `examples/hello.lp` | Functions, loops, print |
| `examples/loops.lp` | All loop types |
| `examples/input_demo.lp` | User input |
| `examples/calculator.lp` | Math: + − × ÷ |

---

## IDE features (Cursor / VS Code)

After install + reload:

- **Syntax colors** — keywords, strings, comments, functions
- **Autocomplete** — type `fun` → suggests `function` snippet
- **Error squiggles** — red underlines on mistakes
- **Hover** — docs on keywords
- **Auto-indent** — after `,` indent; on `..` de-indent
- **Go to definition** — click a function name to jump to it

If colors don't appear: bottom-right language mode → select **Lang.P**, then reload window.

---

## Python → Lang.P quick reference

| Python | Lang.P |
|--------|--------|
| `# comment` | `@ comment` |
| `print("hi")` | `print "hi".` |
| `def f(x):` | `function f(x),` ... `.` |
| `if x:` | `if x,` ... `..` |
| `elif x:` | `otherwise if x,` |
| `else:` | `otherwise,` |
| `for i in range(5):` | `repeat 5 times as i,` ... `..` |
| `while x:` | `while x,` ... `..` |
| `True/False` | `true/false` |
| `"a" + b` | `"a" with b` |
| `input("name?")` | `input text "name?"` |

---

## Next steps

- **[Language Reference (v0.1)](LANGUAGE-REFERENCE.md)** — all commands & functions that work now
- Read the [full language specification](../spec/LANGP-SPEC.md)
- Browse [examples/](../../examples/)
- Run `lang check` on your code before `lang run`

Happy coding in Lang.P!
