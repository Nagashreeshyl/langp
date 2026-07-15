# Functions

## Introduction

**Functions** are named, reusable blocks of code. Define once, call many times — with parameters when you need flexibility and `return` when you need a result back.

**Why functions:** They eliminate copy-paste, organize programs into readable steps, and are the main unit of reuse before [24 — Modules](24-modules.md).

**When to define functions:** Any time the same logic appears twice, or when a block deserves its own name (`calculate_tax`, `greet_user`). Functions use the same `,` / `..` block rules as [03 — Language Basics](03-language-basics.md).

---

## Syntax

```lp
function greet(name),
    print "Hello " with name with "!".
..

function add(a, b),
    return a + b.
..

greet("World").
result = add(3, 5).
```

| Part | Rule |
|------|------|
| Header | `function name(params),` — comma opens the body |
| Body | Indented statements, each ending with `.` |
| Close | `..` on its own line |
| Call | `name(arg1, arg2).` — parentheses required |
| Return | `return value.` — exits the function |

Optional type annotations on parameters and return type:

```lp
function add(a: Int, b: Int) -> Int,
    return a + b.
..
```

---

## Examples

### Simple — greet

**Learning version:**

```lp
@ Define then call.
function greet(name),
    print "Hello " with name with "!".
..

greet("Naga").
greet("World").
```

**Professional version:**

```lp
function greet(name),
    print "Hello " with name with "!".
..

greet("Naga").
```

See [examples/hello.lp](../../examples/hello.lp).

### Intermediate — return a value

**Learning version:**

```lp
@ Return sends a value to the caller.
function double(n),
    return n * 2.
..

a = double(4).
print "double(4) = " with a.
```

**Professional version:**

```lp
function double(n),
    return n * 2.
..

print double(4).
```

### Advanced — recursion

**Learning version:**

```lp
@ A function may call itself — base case stops recursion.
function factorial(n),
    if n <= 1,
        return 1.
    otherwise,
        return n * factorial(n - 1).
    ..
..

print "5! = " with factorial(5).
```

**Professional version:**

```lp
function factorial(n),
    if n <= 1,
        return 1.
    otherwise,
        return n * factorial(n - 1).
    ..
..

print factorial(5).
```

---

## Common Mistakes

**Mistake:** Missing `..` to close the function

```lp
function greet(name),
    print "Hi".
@ forgot ..
```

**Fix:** Add `..` after the last body statement.

---

**Mistake:** Calling without parentheses

```lp
greet "World".    @ wrong
```

**Fix:**

```lp
greet("World").
```

---

**Mistake:** `return` outside a function

```lp
return 5.    @ error at top level
```

**Fix:** Put `return` only inside a `function` body.

---

**Mistake:** Forgetting `.` on inner statements

```lp
function add(a, b),
    return a + b    @ wrong — needs .
..
```

**Fix:**

```lp
function add(a, b),
    return a + b.
..
```

---

## Best Practices

- Use `snake_case` function names: `calculate_total`, not `calculateTotal`.
- Keep functions focused — one clear job per function.
- Prefer `return` for computed values; use `print` inside only when the function's job is to display.
- Put the function definition above its first call in small programs.
- Document tricky functions with `@` comments ([04 — Comments](04-comments.md)).

---

## Exercises

### Beginner

1. Write `function say_hello(),` that prints `"Hello!"` and call it twice.
2. Write `function square(n),` returning `n * n`, and print `square(6)`.
3. Add `greet` from [examples/hello.lp](../../examples/hello.lp) to your own file and call it with your name.
4. Fix a function missing its closing `..`
5. Write a function with two parameters that prints both with `with`.

### Intermediate

1. Write `function max_of(a, b),` returning the larger value using `if` ([12 — Conditionals](12-conditionals.md)).
2. Write `function is_even(n),` returning whether `n % 2 == 0`.
3. Add type annotations `function add(a: Int, b: Int) -> Int` and test with `lang check`.
4. Nest an `if` inside a function and call it from top level.
5. Refactor three repeated `print` blocks into one function.

### Advanced

1. Implement `fibonacci(n)` recursively with a clear base case.
2. Write `function countdown(n),` that prints from `n` to `1` using recursion or a loop ([13 — Loops](13-loops.md)).

---

## Summary

Define functions with `function name(params),` … `..`, call with `name(args).`, and use `return value.` to send results back. Functions follow the same block punctuation as `if` and loops. Recursion works when a base case stops the calls.

**Previous:** [10 — Strings](10-strings.md) · **Next:** [12 — Conditionals](12-conditionals.md)

**See also:** [05 — Variables](05-variables.md), [13 — Loops](13-loops.md), [24 — Modules](24-modules.md), [examples/hello.lp](../../examples/hello.lp)
