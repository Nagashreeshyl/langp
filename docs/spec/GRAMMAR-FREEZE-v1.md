# Lang.P Grammar Freeze v1.0

**Status:** Official — frozen as of 2026-07-15  
**Version:** 1.0.0  
**Supersedes:** Informal dual close rules (`.` for declarations vs `..` for control flow)

This document is the **single source of truth** for Lang.P surface syntax. All documentation, examples, parser rules, and IDE tooling MUST conform to it.

Related documents:

- [DESIGN-DECISIONS.md](DESIGN-DECISIONS.md) — why each rule exists
- [LANGUAGE-PHILOSOPHY.md](LANGUAGE-PHILOSOPHY.md) — design principles
- [KEYWORDS.md](KEYWORDS.md) — reserved words
- [Formal EBNF](../grammar/03-syntactic-grammar.ebnf) — machine-readable grammar

---

## 1. Language Philosophy (summary)

Lang.P code reads like instructions. Beginners come first. Syntax stays minimal: three punctuation marks (`.`, `,`, `..`) plus `@` for comments. See [LANGUAGE-PHILOSOPHY.md](LANGUAGE-PHILOSOPHY.md) for the full principles.

---

## 2. Grammar Rules

| Token | Name | Role |
|-------|------|------|
| `@` | Comment | Explanatory text; not executed |
| `.` | Statement end | Terminates **every statement** |
| `,` | Block open | Opens **every block** after a header |
| `..` | Block close | Closes **every block** — no exceptions |

**There is exactly one block rule:**

> Every block begins with `,` and ends with `..`.

This applies to functions, types, enums, conditionals, loops, try/catch, event handlers, and object bodies.

---

## 3. Statement Rules

1. Every executable line is a **statement**.
2. Every statement ends with `.` (period).
3. Statements inside a block are indented (4 spaces per level).
4. A statement MUST NOT use `.` to close a block — only `..` closes blocks.

```lp
name = "Naga".
print "Hello " with name.
```

---

## 4. Block Rules

### 4.1 Opening

After any block header, write `,` then indent:

```lp
if ready,
    start().
..
```

Headers include: `function …(…)`, `type Name`, `enum Name`, `if expr`, `otherwise if expr`, `otherwise`, `repeat …`, `for … in …`, `while expr`, `try`, `catch name`, `finally`, `on event`.

### 4.2 Closing

Close every block with `..` at the same indentation as the header:

```lp
function greet(name),
    print "Hello " with name.
..
```

**Never** close a block with a lone `.` on a dedented line.

### 4.3 Nesting

Each nested block has its own `..`:

```lp
repeat forever,
    if should_stop,
        break.
    ..
..
```

### 4.4 Compound constructs

`if` / `otherwise if` / `otherwise` and `try` / `catch` / `finally` share **one** closing `..` for the whole construct:

```lp
if score >= 90,
    print "A".
otherwise if score >= 80,
    print "B".
otherwise,
    print "C".
..

try,
    risky().
catch error,
    print error.
finally,
    cleanup().
..
```

---

## 5. Comments

Comments begin with `@`:

```lp
@ Store the user's name.
name = input text "Name: ".
```

---

## 6. Variables

Python-style assignment. Name, `=`, value, `.`:

```lp
count = 0.
name = "Naga".
active = true.
```

---

## 7. Functions

```lp
function greet(name),
    print "Hello " with name.
..

function add(a, b),
    return a + b.
..

greet("World").
```

- Keyword: `function`
- Parameters in parentheses
- Body opens with `,`, closes with `..`
- Calls use parentheses: `greet("Naga").`

---

## 8. Types

```lp
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..
..
```

```lp
enum Color,
    Red.
    Green.
    Blue.
..
```

Field and variant lines are **statements** (end with `.`). The enclosing `type` / `enum` block closes with `..`.

---

## 9. Loops

```lp
repeat 5 times,
    print "Hello".
..

repeat 5 times as i,
    print i.
..

repeat forever,
    work().
..

for item in items,
    print item.
..

while running,
    update().
..
```

`break.` and `continue.` are statements.

---

## 10. Conditions

Keywords: `if`, `otherwise if`, `otherwise` (never `else if` / `else`).

```lp
if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..
```

Inline conditional:

```lp
label = if score >= 60, "Pass", otherwise, "Fail".
```

---

## 11. Input

```lp
name = input text "Enter your name: ".
age = input number "Enter your age: ".
salary = input decimal "Salary: ".
password = input password "Password: ".
confirmed = input boolean "Continue? ".
```

GUI pickers (`input file`, `input folder`, `input date`, `input color`) are specified; stdin types are implemented in v0.1.

---

## 12. Output

```lp
print "Hello".
print "Hello " with name.
print "Sum: " with (a + b).
```

String composition uses `with`, not `+`.

---

## 13. Imports

```lp
use navigator.
use ai.
use database.
use network.
use filesystem.
```

Module loading is specification in v0.1; syntax is frozen.

---

## 14. Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+` `-` `*` `/` (numbers) |
| Comparison | `==` `!=` `<` `>` `<=` `>=` |
| Logical | `and` `or` `not` |
| Assignment | `=` `+=` `-=` `*=` `/=` |
| String compose | `with` |
| Member / index | `.` `[` `]` |

---

## 15. Collections

```lp
items = ["a", "b", "c"].
config = {"host": "localhost", "port": 8080}.
print items[0].
print config["host"].
```

---

## 16. Error Handling

```lp
try,
    risky().
catch error,
    print error.
finally,
    cleanup().
..
```

`throw expr.` to raise.

---

## 17. Async

```lp
async function fetch(url),
    response = wait for get url.
    return response.body.
..
```

Async runtime is specification in v0.1; syntax is frozen.

---

## 18. Object Creation

```lp
user = User("Naga", 25).

user = User(),
    name = "Naga".
    age = 25.
..
```

Named object bodies follow the same `,` / `..` block rules.

---

## 19. Complete Examples

### Learning version

```lp
@ Ask for a name and greet the user.
name = input text "Your name: ".

function greet(name),
    print "Hello " with name with "!".
..

greet(name).

@ Count down from 3.
repeat 3 times as i,
    print "Tick " with i.
..

print "Done!".
```

### Professional version

```lp
function classify(age),
    if age >= 18,
        return "adult".
    otherwise if age >= 13,
        return "teen".
    otherwise,
        return "child".
    ..
..

function main(),
    age = input number "Age: ".
    print classify(age).
..

main().
```

---

## 20. Conformance

Implementations MUST:

1. Reject block close with lone `.` (dedented)
2. Accept `..` as the only block close token
3. Require `.` on every statement

Test fixtures: `tests/conformance/parse/`

---

## Amendment process

After v1.0 freeze, syntax changes require a new grammar version and entry in [Chapter 22 — Compatibility](22-compatibility-versioning.md). Do not amend this document without a version bump.
