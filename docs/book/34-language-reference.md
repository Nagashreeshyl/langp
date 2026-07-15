# Language Reference

## Introduction

This chapter is the **complete lookup reference** for Lang.P v0.2.0 — every keyword, operator, builtin, CLI command, collection method, filesystem function, and common error code that works today.

**Why a reference chapter exists:** Tutorial chapters teach concepts in order. While coding, you need a single index.

**When to use it:** Bookmark this page. Use the table of contents below to jump to what you need.

For narrative learning, start at [02 — Your First Program](02-your-first-program.md). For stability labels, see [STATUS.md](../../STATUS.md).

---

## Syntax

### Punctuation

| Symbol | Meaning |
|--------|---------|
| `.` | End of statement |
| `,` | Open block |
| `..` | Close block |
| `@` | Comment |

### Literals

| Literal | Type |
|---------|------|
| `42`, `-7` | Int |
| `3.14`, `-0.5` | Float |
| `"text"` | String |
| `true`, `false` | Bool |
| `null` | Null |

---

## Keywords (implemented)

### Core

| Keyword | Usage |
|---------|-------|
| `function` | Define a function |
| `return` | Return from function |
| `if` | Conditional branch |
| `otherwise if` | Else-if branch |
| `otherwise` | Final else branch |
| `repeat` | Loop (`repeat N times`, `repeat forever`) |
| `times` | With `repeat N times` |
| `as` | Loop counter name |
| `while` | Condition loop |
| `for` | `for item in collection,` |
| `in` | Membership / iteration |
| `break` | Exit loop |
| `continue` | Next iteration |
| `try` | Start error handling |
| `catch` | Handle error |
| `finally` | Always run after try/catch |
| `pass` | No-op placeholder |
| `with` | Join strings in print/expressions |
| `type` | Define object type *(Beta)* |
| `extends` | Inheritance *(Beta)* |
| `use` | Import module *(Beta)* |
| `self` | Current instance in methods *(Beta)* |

### I/O statements

| Keyword | Usage |
|---------|-------|
| `print` | Output with newline |
| `print inline` | Output without newline |
| `input` | Read user input |
| `read` | Read file (expression) |
| `read_bytes` | Read raw bytes |
| `read_lines` | Read lines as list |
| `write` | Write to file |
| `write_bytes` | Write bytes |
| `append` | Append to file |
| `copy` | Copy file |
| `move` | Move file |
| `rename` | Rename file |
| `delete` | Delete file |

---

## Operators

| Operator | Applies to | Meaning |
|----------|------------|---------|
| `+` `-` `*` `/` | Numbers | Arithmetic |
| `%` | Integers | Remainder |
| `==` `!=` | All comparable | Equality |
| `<` `>` `<=` `>=` | Numbers, strings | Ordering |
| `and` `or` `not` | Bool | Logic |
| `()` | Expressions | Grouping |

**Strings:** use `with`, not `+`.

---

## Built-in functions

| Function | Description |
|----------|-------------|
| `len(x)` | Length of string, list, or dict |
| `to_string(x)` | Convert value to string |
| `assert condition.` | Stop if false |
| `assert condition, "msg".` | Stop with message |

---

## Collection methods

### List `[a, b, c]`

| Method | Description |
|--------|-------------|
| `append(x)` | Add at end |
| `insert(i, x)` | Insert at index |
| `remove(x)` | Remove first match |
| `pop()` / `pop(i)` | Remove and return |
| `clear()` | Remove all |
| `sort()` | Sort in place |
| `reverse()` | Reverse in place |
| `contains(x)` | Membership test |
| `length()` | Element count |

### Dictionary `{key: value}`

| Method | Description |
|--------|-------------|
| `keys()` | List of keys |
| `values()` | List of values |
| `items()` | List of `[key, value]` |
| `remove(key)` | Delete entry |
| `contains(key)` | Key exists? |
| `clear()` | Remove all |
| `length()` | Entry count |

### Set `{1, 2, 3}`

| Method | Description |
|--------|-------------|
| `add(x)` | Insert value |
| `remove(x)` | Delete value |
| `contains(x)` | Membership |
| `clear()` | Remove all |
| `union(other)` | Set union |
| `intersection(other)` | Set intersection |
| `difference(other)` | Set difference |
| `length()` | Element count |

### Tuple `(a, b)`

| Method | Description |
|--------|-------------|
| `length()` | Element count |
| `contains(x)` | Membership |

Index read: `t[0]`. No index assignment.

---

## Filesystem module (`use filesystem.`)

| Function | Description |
|----------|-------------|
| `filesystem.read(path)` | Read file as string |
| `filesystem.read_bytes(path)` | Read bytes |
| `filesystem.write(content, path)` | Write file |
| `filesystem.append(content, path)` | Append |
| `filesystem.copy(src, dst)` | Copy |
| `filesystem.move(src, dst)` | Move |
| `filesystem.delete(path)` | Delete |
| `filesystem.exists(path)` | Bool |
| `filesystem.list(path)` | List directory names |
| `filesystem.create_folder(path)` | Create directory |
| `filesystem.remove_folder(path)` | Remove directory tree |

Top-level `read`/`write`/`copy`/`delete` work without import.

---

## Math module (`use math.`) — Beta

| Function | Description |
|----------|-------------|
| `math.abs(n)` | Absolute value |
| `math.min(a, b)` | Minimum |
| `math.max(a, b)` | Maximum |

---

## JSON module (`use json.`) — Beta stub

| Function | Description |
|----------|-------------|
| `json.stringify(value)` | Basic string conversion |
| `json.parse(text)` | Limited stub |

---

## CLI commands

### `lang`

| Command | Description |
|---------|-------------|
| `lang run file.lp` | Execute program |
| `lang file.lp` | Same as run |
| `lang check file.lp` | Static check only |
| `lang --version` | Version string |
| `lang init [name]` | New project |
| `lang install [pkg]` | Install dependencies |
| `lang remove <pkg>` | Remove dependency |
| `lang update` | Refresh lock file |
| `lang search <query>` | Search registry |
| `lang build` | Check project entry |
| `lang test` | Check all `tests/*.lp` |
| `lang fmt` | Validate entry (formatter WIP) |
| `lang doctor` | Toolchain health |
| `lang publish` | Not connected yet |
| `lang login` | Not connected yet |

### `langc`

| Command | Description |
|---------|-------------|
| `langc file.lp` | Parse and run |
| `langc --check file.lp` | Semantic check |
| `langc --emit ast file.lp` | Print AST JSON |
| `langc --emit tokens file.lp` | Print tokens |
| `langc --version` | Version string |

---

## Common error codes

| Code | Meaning | Fix |
|------|---------|-----|
| E0200 | Parse error | Check `.` `,` `..` and parentheses |
| E0202 | Statement must end with `.` | Add period |
| E0203 | Expected `..` | Close block with `..` |
| undefined name | Variable not declared | Define before use |
| semantic errors prevent execution | Check failed | Fix all errors before run |

See [30 — Debugging](30-debugging.md) and [33 — Common Mistakes](33-common-mistakes.md).

---

## Examples

### Simple — quick lookup usage

```lp
scores = [90, 85, 92].
print scores.length().
print len("hello").
assert scores.length() > 0.
```

### Intermediate — filesystem + module

```lp
use filesystem.

write "log entry" to "app.log".
print filesystem.exists("app.log").
delete "app.log".
```

### Advanced — types + collections

```lp
type Student,
    name.
    grades.

    function init(name),
        self.name = name.
        self.grades = [].
    ..

    function average(),
        total = 0.
        for g in self.grades,
            total = total + g.
        ..
        return total / self.grades.length().
    ..
..

s = Student("Naga").
s.grades.append(95).
s.grades.append(87).
print s.average().
```

---

## Common mistakes

Refer to [33 — Common Mistakes](33-common-mistakes.md) for detailed wrong/right pairs.

---

## Best practices

- Run `lang check` before every commit.
- Use this reference alongside [STATUS.md](../../STATUS.md) — do not assume spec-only features work.
- Prefer tutorial chapters for first-time learning of a topic.

---

## Exercises

### Beginner

1. Look up how to append to a list; write a three-line demo.
2. Find the CLI command for checking a file without running it.
3. List all input types from [08 — Input](08-input.md).
4. Write a program using `assert` and trigger it intentionally.
5. Name three keywords that are Beta in v0.2.

### Intermediate

1. Build a cheat sheet card from this chapter (one page).
2. Use every filesystem statement keyword once in a single program.
3. Call `math.abs`, `math.min`, and `math.max` in one file.
4. Parse error E0202 — reproduce and fix it.
5. Compare this reference to [docs/guides/LANGUAGE-REFERENCE.md](../guides/LANGUAGE-REFERENCE.md).

### Advanced

1. Propose one missing entry for this reference based on source code.
2. Write a `tests/reference.lp` that exercises one method from each collection type.

---

## Summary

This reference documents Lang.P v0.2.0 as implemented: punctuation, keywords, operators, builtins, collections, filesystem, modules, CLI, and errors. Planned-only features are in [37 — Future Roadmap](37-future-roadmap.md).

**Previous:** [33 — Common Mistakes](33-common-mistakes.md) · **Next:** [35 — Complete Projects](35-complete-projects.md)

**See also:** [STATUS.md](../../STATUS.md), [Grammar Freeze v1.0](../spec/GRAMMAR-FREEZE-v1.md), [examples/](../../examples/)
