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
..

add(3, 5).
```

| Keyword | Usage |
|---------|--------|
| `function name(params),` | Define a function; statements inside end with `.`, block closes with `..` |
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

## Collections

### List

```lp
numbers = [1, 2, 3].
names = ["Naga", "Alex", "John"].
mixed = [1, "Hello", true].

print numbers[0].
numbers[1] = 20.

numbers.append(4).
numbers.insert(2, 100).
numbers.remove(3).
numbers.pop().
numbers.clear().
numbers.sort().
numbers.reverse().
print numbers.contains(5).
print numbers.length().

for item in numbers,
    print item.
..
```

Typed annotation: `scores: List<Int> = [95, 87, 92].`

| Method | Description |
|--------|-------------|
| `append(x)` | Add element at end |
| `insert(i, x)` | Insert at index |
| `remove(x)` | Remove first matching value |
| `pop()` / `pop(i)` | Remove and return last / indexed element |
| `clear()` | Remove all elements |
| `sort()` | Sort in place |
| `reverse()` | Reverse in place |
| `contains(x)` | Returns `true`/`false` |
| `length()` | Number of elements |

Also available: global `len(list)`.

### Dictionary

```lp
student = {
    name : "Naga",
    age : 18,
    college : "DSU"
}.

print student.name.
print student["name"].
student.age = 19.
student["age"] = 20.

print student.keys().
print student.values().
print student.items().
student.remove("age").
student.contains("name").
student.clear().
```

| Method | Description |
|--------|-------------|
| `keys()` | List of keys |
| `values()` | List of values |
| `items()` | List of `[key, value]` pairs |
| `remove(key)` | Remove entry |
| `contains(key)` | Key exists? |
| `clear()` | Remove all entries |
| `length()` | Number of entries |

### Set

```lp
numbers = {1, 2, 3}.
other = {3, 4, 5}.

numbers.add(4).
numbers.remove(2).
print numbers.contains(1).
numbers.clear().

print numbers.union(other).
print numbers.intersection(other).
print numbers.difference(other).
```

`{1, 2, 3}` is a **Set**; `{key: value}` is a **Dict** (colon disambiguates).

### Tuple (immutable)

```lp
point = (10, 20).
print point[0].
print point.length().
```

Tuple index assignment is not allowed.

---

## Collections (basic — legacy summary)

| Feature | Example |
|---------|---------|
| List literal | `[1, 2, 3]` |
| Dict literal | `{name: "Naga", age: 18}` |
| Set literal | `{1, 2, 3}` |
| Tuple literal | `(10, 20)` |
| Index | `list[0]`, `dict["key"]`, `tuple[0]` |
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
