# Your First Program

## Introduction

A **program** is a list of instructions stored in a file. Lang.P programs use the `.lp` extension. The computer reads the file top to bottom and executes each **statement** in order.

**Why start with hello world:** Every language tutorial begins here because it confirms your editor, runner, and syntax all work before harder topics.

**When to write a new file:** Whenever you start a new idea. One file can hold many statements; larger projects split into multiple files (see [29 — Project Structure](29-project-structure.md)).

---

## Syntax

Every Lang.P program follows three rules from day one:

1. Each statement ends with `.`
2. Blocks start with `,` after a header
3. Blocks end with `..`

Your first program uses one statement:

```lp
print "Hello, Lang.P!".
```

| Piece | Meaning |
|-------|---------|
| `print` | Built-in command — show text on screen |
| `"Hello, Lang.P!"` | A string (text in quotes) |
| `.` | End of this statement |

---

## Examples

### Simple — hello world

**Learning version:**

```lp
@ Say hello to the world.
print "Hello, Lang.P!".
```

**Professional version:**

```lp
print "Hello, Lang.P!".
```

Save as `hello.lp`, then:

```bash
lang run hello.lp
lang check hello.lp
```

### Intermediate — greet by name

**Learning version:**

```lp
@ Ask for a name and greet the user.
name = input text "What is your name? ".
print "Hello " with name with "!".
```

**Professional version:**

```lp
name = input text "What is your name? ".
print "Hello " with name with "!".
```

Run interactively (type a name when prompted):

```bash
lang run hello_name.lp
```

### Advanced — a tiny function

**Learning version:**

```lp
@ Define a reusable greeting function.
function greet(name),
    print "Hello " with name with "!".
..

@ Call the function twice.
greet("World").
greet("Lang.P").
```

**Professional version:**

```lp
function greet(name),
    print "Hello " with name with "!".
..

greet("World").
greet("Lang.P").
```

---

## Common mistakes

**Mistake:** Forgetting the period at the end.

```lp
print "Hello"
```

**Why:** Lang.P requires `.` on every statement ([Grammar Freeze](../spec/GRAMMAR-FREEZE-v1.md)).

**Fix:**

```lp
print "Hello".
```

---

**Mistake:** Using `+` to join text.

```lp
print "Hello " + name.
```

**Why:** String concatenation uses `with`, not `+` (see [10 — Strings](10-strings.md)).

**Fix:**

```lp
print "Hello " with name.
```

---

## Best practices

- Name files in `snake_case`: `hello_name.lp`, not `HelloName.lp`.
- Run `lang check` before sharing code.
- Start with learning comments; remove them as you gain confidence.
- Compare your code to [examples/hello.lp](../../examples/hello.lp).

---

## Exercises

### Beginner

1. Print your full name on one line.
2. Print two separate lines: "Lang.P" and "is fun".
3. Add an `@` comment to every line explaining it.
4. Run `lang check` on your file and fix any errors.
5. Change the string message to include an emoji (if your terminal supports it).

### Intermediate

1. Ask for name and age, then print both on one line using two `with` segments.
2. Write a function `say_twice(word)` that prints the word two times.
3. Copy [examples/hello.lp](../../examples/hello.lp) and explain each block.
4. Make a program that prints a simple ASCII art pattern (three lines).
5. Use `langc --emit ast hello.lp` and observe the output structure.

### Advanced

1. Write `greet(title, name)` that prints "Hello Dr. Smith" style output.
2. Combine a `repeat 3 times` loop with `print` (preview of [13 — Loops](13-loops.md)).
3. Document your workflow: edit → check → run in a short README.

---

## Summary

You created `.lp` files, used `print`, ran programs with `lang run`, and validated them with `lang check`. Every statement ends with `.`; blocks use `,` and `..`.

**Previous:** [01 — Installation](01-installation.md) · **Next:** [03 — Language Basics](03-language-basics.md)

**See also:** [09 — Output](09-output.md), [08 — Input](08-input.md), [11 — Functions](11-functions.md)
