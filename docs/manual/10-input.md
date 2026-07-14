# 10 — Input

**Status: Partial — stdin types implemented; native pickers specification**

---

## Overview

Input reads data from the user. Typed forms validate and convert values.

---

## Implemented (v0.1) — terminal input

| Form | Returns | Example |
|------|---------|---------|
| `input text "prompt"` | String | `input text "Name: "` |
| `input "prompt"` | String (same as text) | `input "Name: "` |
| `input number "prompt"` | Integer | `input number "Age: "` |
| `input decimal "prompt"` | Float | `input decimal "Price: "` |
| `input boolean "prompt"` | Bool | `input boolean "Yes/no: "` |
| `input password "prompt"` | String (hidden) | `input password "Pass: "` |

### Learning version

```lp
@ Collect user profile from keyboard.
name = input text "Enter your name: ".
age = input number "Enter your age: ".
salary = input decimal "Enter salary: ".

print "Hello " with name.
print "Age: " with age.
```

### Professional version

```lp
name = input text "Enter your name: ".
age = input number "Enter your age: ".
print "Hello " with name with " (" with age with ")".
```

See [examples/input_demo.lp](../../examples/input_demo.lp).

---

## Specification — native pickers (not yet in v0.1)

These forms are defined in the language specification for GUI environments:

```lp
@ Specification — requires graphical environment (not v0.1 terminal).
file = input file "Choose file".
folder = input folder "Choose folder".
date = input date "Choose date".
color = input color "Choose color".
```

Do not use these in terminal-only programs until the runtime supports them.

---

## Compiler inference

When you write `input number`, the compiler knows the result is numeric — use it in math without manual conversion:

```lp
a = input number "A: ".
b = input number "B: ".
print (a + b).
```

For `input text`, use `with` for display or `int(...)` / conversion builtins when added.

---

## Next steps

- [11 — Output](11-output.md)
- [Input types (spec)](../spec/04-types.md)
- [Input expression (spec)](../spec/06-expressions.md)
