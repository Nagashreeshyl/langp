# Variables

## Introduction

**Variables** are named containers for values. You assign once (or many times), then read the name wherever you need that value.

**Why variables:** Programs would be impossible to read if every number and string were repeated literally. Names like `user_name` and `total_score` tell humans what data means.

**When to use them:** Store anything you need more than once — input from the user, loop counters, configuration, function results. Pair clear names with [06 — Data Types](06-data-types.md) and [04 — Comments](04-comments.md).

---

## Syntax

Assignment uses `=` and **must** end with `.`:

```lp
name = "Naga".
count = 0.
count = count + 1.    @ reassignment
```

Optional **type annotations** come before `=` and are checked by the compiler:

```lp
age: Int = 18.
price: Float = 9.99.
label: String = "Lang.P".
active: Bool = true.
```

**Naming rules:**

| Style | Use | Example |
|-------|-----|---------|
| `snake_case` | Variables and functions | `user_name`, `total_count` |
| `PascalCase` | Types (when you define them) | `User`, `HttpClient` |
| `UPPER_SNAKE` | Constants by convention | `MAX_RETRIES` |

Names start with a letter or `_`, then letters, digits, or `_`.

**Scope:**

- Top-level variables are visible from their assignment line downward in the same file.
- Variables inside a function body are visible only inside that function.
- Loop counter variables (`repeat 5 times as i`) exist **only inside** the loop block.

---

## Examples

### Simple — first assignments

**Learning version:**

```lp
@ Store basic profile data.
name = "Naga".
age = 18.
print "Hello " with name with ", age " with age.
```

**Professional version:**

```lp
name = "Naga".
age = 18.
print "Hello " with name with ", age " with age.
```

### Intermediate — reassignment and typed annotations

**Learning version:**

```lp
@ Score starts at zero and increases.
score: Int = 0.
score = score + 10.
score = score + 5.
print "Final score: " with score.
```

**Professional version:**

```lp
score: Int = 0.
score = score + 10.
score = score + 5.
print "Final score: " with score.
```

### Advanced — loop variable scope

**Learning version:**

```lp
@ i is only valid inside the repeat block.
repeat 3 times as i,
    print "Inside loop, i = " with i.
..

@ Using i here would be an error — i is out of scope.
print "Loop finished.".
```

**Professional version:**

```lp
repeat 3 times as i,
    print "Inside loop, i = " with i.
..

print "Loop finished.".
```

See [examples/loops.lp](../../examples/loops.lp) for runnable loop scope examples.

---

## Common Mistakes

**Mistake:** Forgetting the trailing `.`

```lp
name = "Naga"    @ wrong — statement not ended
```

**Why:** Lang.P treats `.` as the end of every statement ([03 — Language Basics](03-language-basics.md)).

**Fix:**

```lp
name = "Naga".
```

---

**Mistake:** Using a loop variable after the block ends

```lp
repeat 5 times as i,
    print i.
..
print i.    @ error — i does not exist here
```

**Why:** `as i` creates a block-local binding, like a counter that disappears when the loop closes.

**Fix:** Copy the value you need into a variable declared before the loop, or restructure the logic to stay inside the block.

---

**Mistake:** Mismatched type annotation and value

```lp
age: String = 18.    @ compile error — 18 is Int, not String
```

**Why:** Annotations are validated semantically at compile time.

**Fix:** Use the correct type or remove the annotation and let inference decide:

```lp
age: Int = 18.
```

---

**Mistake:** `camelCase` in a Lang.P codebase

```lp
userName = "Naga".    @ works, but breaks project style
```

**Fix:** Use `snake_case`: `user_name = "Naga".` See [STYLE_GUIDE.md](../../STYLE_GUIDE.md).

---

## Best Practices

- Prefer descriptive `snake_case` names: `monthly_salary` over `ms`.
- Add type annotations on public APIs or when the type is not obvious from the right-hand side.
- Keep variable declarations near first use; avoid long gaps between assignment and use.
- Use `UPPER_SNAKE` for values that should never change (convention until `const` enforcement ships).
- Do not reuse one variable for unrelated meanings (e.g. `temp` for both a name and a count).

---

## Exercises

### Beginner

1. Create variables `first_name` and `last_name`, then print a full name with `with`.
2. Assign `x = 5.`, then reassign `x = x + 3.`, and print `x`.
3. Rename a variable from `n` to `student_count` in a three-line program.
4. Run `lang check` on a file with a missing `.` — read the error and fix it.
5. Write two variables using `snake_case` and explain why in an `@` comment.

### Intermediate

1. Declare `attempts: Int = 0.`, increment it three times, print the final value.
2. Write a program that uses `repeat 4 times as step` and prints `step` each time; confirm `step` is invalid after `..`.
3. Add optional type annotations to every variable in [examples/hello.lp](../../examples/hello.lp).
4. Convert a program that uses `camelCase` names to `snake_case`.
5. Store `input number` result in `age: Int` and print whether it is greater than 0.

### Advanced

1. Write a function `swap_values` pattern using reassignment only (no collections) for two `Int` variables.
2. Document in comments when you would add explicit types vs rely on inference, with three real examples from your own code.

---

## Summary

Variables hold values under readable names. Assign with `=`, end every statement with `.`, reassign freely, and use `snake_case` for variables and functions. Type annotations like `x: Int = 1.` are optional but validated. Loop variables from `as i` exist only inside their loop block.

**Previous:** [04 — Comments](04-comments.md) · **Next:** [06 — Data Types](06-data-types.md)

**See also:** [11 — Functions](11-functions.md), [13 — Loops](13-loops.md), [32 — Best Practices](32-best-practices.md), [STYLE_GUIDE.md](../../STYLE_GUIDE.md)
