# Lang.P Language Reference (v0.1 — implemented)

This document lists **every command, keyword, and function that works today** in Lang.P.  
It matches the interpreter and IDE autocomplete (`editors/langp-manifest.json`).

For planned (not yet built) features, see the [full specification](../spec/LANGP-SPEC.md).

---

## Punctuation rules

| Symbol | Meaning |
|--------|---------|
| `.` | End of statement |
| `,` | Start an indented block (after `if`, `function`, `repeat`, …) |
| `..` | End an indented block |

---

## CLI commands

| Command | Description |
|---------|-------------|
| `lang run file.lp` | Run a program |
| `lang file.lp` | Same as `lang run` |
| `lang check file.lp` | Check for errors without running |
| `lang --version` | Show version |
| `langc --emit ast file.lp` | Debug: print syntax tree |

---

## Output

### `print`

Print to the terminal. Use `with` to append text and values.

```lp
print "Hello World!".
print "Hello " with name with "!".
print "Sum: " with (a + b).
```

| Form | Description |
|------|-------------|
| `print "text".` | Print a string |
| `print "text" with value.` | Print string + value (value converted to text) |
| `print inline …` | Inline print variant (no newline semantics differ) |

---

## User input

### `input`

Read from the keyboard (stdin).

```lp
name = input text "Enter your name: ".
age = input number "Enter your age: ".
price = input decimal "Enter price: ".
ok = input boolean "Continue? ".
secret = input password "Password: ".
```

| Type | Returns | Example |
|------|---------|---------|
| `text` | String | `input text "Name: "` |
| `number` | Integer | `input number "Age: "` |
| `decimal` | Float | `input decimal "Price: "` |
| `boolean` | true/false | `input boolean "Yes/no: "` |
| `password` | String (hidden) | `input password "Pass: "` |

You can omit `text`: `input "prompt".` is the same as `input text "prompt".`

---

## Math and operators

Use **`+` `-` `*` `/`** for numbers. Use **`with`** for joining text.

```lp
num1 = input number "First number: ".
num2 = input number "Second number: ".

print "Sum: " with (num1 + num2).
print "Difference: " with (num1 - num2).
print "Product: " with (num1 * num2).
print "Quotient: " with (num1 / num2).
```

| Operator | Numbers | Strings |
|----------|---------|---------|
| `+` | Addition | Use `with` instead |
| `-` | Subtraction | — |
| `*` | Multiplication | — |
| `/` | Division | — |
| `%` | Remainder (integers) | — |
| `==` `!=` `<` `>` `<=` `>=` | Comparisons | — |
| `and` `or` `not` | Logic | — |

Parentheses work: `(num1 + num2)`.

---

## Variables and assignment

```lp
x = 10.
name = "Naga".
x = x + 1.          @ reassignment
```

---

## Functions

```lp
function add(a, b),
    print (a + b).
.

add(3, 5).
```

| Keyword | Usage |
|---------|--------|
| `function name(params),` | Define a function; body ends with `.` |
| `return value.` | Return from a function |

---

## Control flow keywords

### Conditionals

```lp
if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

| Keyword | Purpose |
|---------|---------|
| `if condition,` | Start if block |
| `otherwise if condition,` | Else-if branch |
| `otherwise,` | Else branch |

### Loops

```lp
repeat 5 times,
    print "Hello".
..

repeat 5 times as i,
    print i.
..

while count > 0,
    count = count - 1.
..

repeat forever,
    break.
..
```

| Keyword | Purpose |
|---------|---------|
| `repeat N times,` | Loop N times |
| `repeat forever,` | Infinite loop |
| `times` | Used with `repeat N times` |
| `as name` | Loop counter variable |
| `for item in list,` | For-each loop |
| `while condition,` | While loop |
| `break.` | Exit loop |
| `continue.` | Next iteration |

### Error handling

```lp
try,
    risky().
catch err,
    print err.
finally,
    print "Done.".
..
```

| Keyword | Purpose |
|---------|---------|
| `try,` | Start try block |
| `catch name,` | Handle error |
| `finally,` | Always runs |

### Other

| Keyword | Purpose |
|---------|---------|
| `pass.` | Do nothing (placeholder) |
| `with` | Join strings in expressions |
| `in` | Used in `for x in items` |
| `true` / `false` / `null` | Literals |

---

## Built-in functions

Call with parentheses. Available everywhere without import.

| Function | Signature | Description |
|----------|-----------|-------------|
| `len` | `len(value)` | Length of string, list, or dict |
| `to_string` | `to_string(value)` | Convert any value to string |
| `assert` | `assert condition.` | Stop program if condition is false |
| `assert` | `assert condition, "message".` | Stop with custom message |

```lp
print len("hello").              @ 5
print to_string(42).
assert age >= 0, "Age invalid".
```

---

## File read expressions

Read files into memory (expression, not statement).

| Expression | Returns |
|------------|---------|
| `read "path/to/file.txt"` | File contents as string |
| `read_bytes "path/to/file.bin"` | Raw bytes |
| `read_lines "path/to/file.txt"` | Lines as a list |

```lp
text = read "data.txt".
lines = read_lines "log.txt".
```

---

## File write statements

| Statement | Description |
|-----------|-------------|
| `write value to "path".` | Write text to file |
| `write_bytes value to "path".` | Write bytes |
| `append value to "path".` | Append to file |

```lp
write "Hello file" to "out.txt".
append "\nMore text" to "out.txt".
```

---

## File management statements

| Statement | Description |
|-----------|-------------|
| `copy "src" to "dest".` | Copy file |
| `move "src" to "dest".` | Move file |
| `rename "src" to "dest".` | Rename file |
| `delete "path".` | Delete file |

---

## Collections (basic)

| Feature | Example |
|---------|---------|
| List literal | `[1, 2, 3]` |
| Dict literal | `{"a": 1, "b": 2}` |
| Index | `list[0]`, `dict["key"]` |
| String concat | `"Hello " with name` (not `+`) |

---

## Comments

```lp
@ This is a comment until end of line.
```

---

## Not implemented yet

These appear in the **full spec** but **do not work** in v0.1:

- HTTP: `get`, `post`, `put`, `patch`, `delete` (network)
- `range()`, `str()`, `int()` as functions
- `use` imports / modules
- `type`, `enum`, classes / full OOP
- `async`, `await`, `wait for`
- Lambdas `(x) => …`
- GUI pickers: `input file`, `input folder`, `input date`, `input color`

When these are added, they will appear in this document and in IDE autocomplete.

---

## See also

- [How to Code in Lang.P](HOW-TO-CODE.md) — beginner tutorial
- [examples/](../../examples/) — sample programs
- [Full specification](../spec/LANGP-SPEC.md) — complete language design
