# Lang.P Keywords Reference

**Status:** Official  
**Grammar:** [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md)

All reserved keywords and compound keywords. **Do not use these as identifiers.**

Legend: **v0.1** = implemented in current interpreter; **spec** = parsed or specified, runtime may be pending.

---

## Control flow

### `if` (v0.1)

Opens a conditional block.

```lp
if age >= 18,
    print "Adult".
..
```

### `otherwise if` (v0.1)

Else-if branch. MUST be written as two words — never `else if`.

```lp
otherwise if age >= 13,
    print "Teen".
..
```

### `otherwise` (v0.1)

Final else branch.

```lp
otherwise,
    print "Child".
..
```

### `repeat` (v0.1)

Counted loop. Use with `times` and optional `as`.

```lp
repeat 5 times as i,
    print i.
..
```

### `repeat forever` (v0.1)

Infinite loop (compound keyword).

```lp
repeat forever,
    work().
..
```

### `for` (v0.1)

Iteration over a collection.

```lp
for item in items,
    print item.
..
```

### `while` (v0.1)

Pre-test loop.

```lp
while count > 0,
    count -= 1.
..
```

### `break` (v0.1)

Exit innermost loop. Statement — ends with `.`

```lp
break.
```

### `continue` (v0.1)

Skip to next iteration.

```lp
continue.
```

### `match` (spec)

Pattern matching (v0.2+).

```lp
match status,
    Active => print "yes".
..
```

---

## Functions

### `function` (v0.1)

Define a function. Body opens with `,`, closes with `..`.

```lp
function greet(name),
    print "Hello " with name.
..
```

### `return` (v0.1)

Return from function.

```lp
return result.
return a, b.
```

### `async` (spec)

Async function modifier.

```lp
async function fetch(url),
    body = wait for get url.
    return body.
..
```

---

## Types and OOP

### `type` (spec)

Define a class-like type.

```lp
type User,
    name.
    age.
..
```

### `enum` (spec)

Algebraic enum.

```lp
enum Color,
    Red.
    Green.
..
```

### `interface` (spec)

Behavior contract.

```lp
interface Drawable,
    function draw().
..
```

### `self` / `super` / `this` (spec)

Object orientation references.

---

## Modules

### `use` (spec)

Import a module.

```lp
use network.
use filesystem.
```

---

## I/O

### `print` (v0.1)

Write to stdout.

```lp
print "Hello " with name.
```

### `input` (v0.1)

Read user input. Followed by type keyword and prompt.

```lp
name = input text "Name: ".
age = input number "Age: ".
```

Input type keywords (after `input`): `text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, `color`.

### `read` / `write` / `append` (v0.1)

File I/O statements. See [Language Reference](../guides/LANGUAGE-REFERENCE.md).

---

## Error handling

### `try` (v0.1)

Begin try block.

```lp
try,
    risky().
catch error,
    handle(error).
..
```

### `catch` (v0.1)

Error handler clause inside try.

### `finally` (v0.1)

Always-run clause inside try.

### `throw` (spec)

Raise an error.

```lp
throw Error("failed").
```

---

## Async

### `wait` / `wait for` (spec)

Await async operation (compound: `wait for`).

```lp
data = wait for fetch(url).
```

### `await` (spec)

Reserved; prefer `wait for` in Lang.P source.

---

## Events

### `on` (spec)

Event handler.

```lp
on button.clicked,
    print "Clicked".
..
```

---

## Logic and literals

### `and` / `or` / `not` (v0.1)

Logical operators.

```lp
if active and not deleted,
    process().
..
```

### `true` / `false` / `null` (v0.1)

Boolean and null literals.

```lp
found = false.
value = null.
```

---

## Operators as keywords

### `with` (v0.1)

String composition (left-associative).

```lp
msg = "Hello " with name with "!".
```

### `in` (v0.1)

Membership and `for` loops.

```lp
for x in items,
    print x.
..
```

### `as` (v0.1)

Loop counter binding.

```lp
repeat 10 times as i,
    print i.
..
```

---

## Contextual / secondary keywords

| Word | Context | Status |
|------|---------|--------|
| `times` | After `repeat N` | v0.1 |
| `forever` | After `repeat` → `repeat forever` | v0.1 |
| `to` | `write x to path`, `copy a to b` | v0.1 |
| `inline` | `print inline` (streaming) | spec |
| `pass` | No-op statement | spec |
| `let` | Immutable binding (future) | spec |
| `static` | Static member (future) | spec |

---

## Reserved but discouraged

| Keyword | Note |
|---------|------|
| `else` | Reserved token; use `otherwise` / `otherwise if` instead |

---

## Statements (not keywords but reserved builtins)

| Name | Example | v0.1 |
|------|---------|------|
| `len` | `len(items)` | yes |
| `assert` | `assert x > 0.` | yes |
| `to_string` | `to_string(n)` | yes |
| `copy` / `move` / `rename` / `delete` | file ops | yes |

Full list: [LANGUAGE-REFERENCE.md](../guides/LANGUAGE-REFERENCE.md)

---

## Related

- [GRAMMAR-FREEZE-v1.md](GRAMMAR-FREEZE-v1.md)
- [02 — Lexical structure](02-lexical-structure.md)
- [editors/langp-manifest.json](../../editors/langp-manifest.json)
