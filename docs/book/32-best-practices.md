# Best Practices

## Introduction

**Best practices** are habits that keep Lang.P programs readable, correct, and easy to maintain. They encode lessons from the grammar ([Grammar Freeze v1.0](../spec/GRAMMAR-FREEZE-v1.md)), the interpreter, and common beginner errors ([33 — Common Mistakes](33-common-mistakes.md)).

**Why follow them:** Lang.P has strict punctuation (`.` and `..`), English logical operators, and string rules that differ from Python or JavaScript. Consistent style prevents silent bugs.

**When to apply them:** From your second program onward — especially before sharing code or opening a pull request.

---

## Syntax

Best practices are conventions, not grammar rules. The table below summarizes the most important ones:

| Practice | Rule | Example |
|----------|------|---------|
| Statement end | Every statement ends with `.` | `print "Hi".` |
| Block end | Blocks close with `..` | `if x, ... ..` |
| String join | Use `with`, not `+` | `"Hi " with name` |
| Logic | Use `and`, `or`, `not` | `if a and b,` |
| Naming | `snake_case` for variables and functions | `user_name`, `calc_total` |
| Types (OOP) | `PascalCase` for type names | `UserAccount` |
| Check before run | Run `lang check` first | `lang check main.lp` |
| Imports | `use module.` at file top | `use filesystem.` |

---

## Examples

### Simple — naming and punctuation

**Learning version:**

```lp
@ snake_case names; every line ends with .
user_name = "Ada".
max_score = 100.

print "Player: " with user_name.
print "Max: " with max_score.
```

**Professional version:**

```lp
user_name = "Ada".
print "Player: " with user_name.
```

### Intermediate — strings and blocks

**Learning version:**

```lp
@ with for text; .. closes the whole if chain.
item = "apple".
price = 3.

if price > 0,
    line = "Item: " with item with " — $" with price.
    print line.
otherwise,
    print "Invalid price.".
..
```

**Professional version:**

```lp
if price > 0,
    print "Item: " with item with " — $" with price.
otherwise,
    print "Invalid price.".
..
```

See [10 — Strings](10-strings.md) and [12 — Conditionals](12-conditionals.md).

### Advanced — check-before-run workflow

**Learning version:**

```bash
@ Terminal workflow for every edit session.
lang check main.lp
lang run main.lp
lang test
```

**Professional version:**

```bash
lang check main.lp && lang run main.lp
```

In the IDE, save and let the extension run check — same principle as CLI ([30 — Debugging](30-debugging.md)).

---

## Common Mistakes

Even experienced developers slip — see [33 — Common Mistakes](33-common-mistakes.md) for the full list. Top three to avoid:

1. Missing **`.`** at statement end
2. Closing blocks with **`.`** instead of **`..`**
3. Using **`+`** instead of **`with`** for strings

---

## Best Practices

- **Run `lang check` before `lang run`** — catches E0200/E0202 early ([30 — Debugging](30-debugging.md)).
- **Use `with` for all string building** — reserve `+` for numbers ([07 — Operators](07-operators.md)).
- **Close every block with `..`** — functions, types, if, loops, try ([03 — Language Basics](03-language-basics.md)).
- **Use `snake_case`** for variables and functions; **`PascalCase`** for types ([18 — Types and OOP](18-type-oop.md)).
- **Comment with `@`** only when logic is non-obvious ([04 — Comments](04-comments.md)).
- **Group imports** at the top ([25 — Imports](25-imports.md)).
- **Handle file errors** with try/catch ([27 — Error Handling](27-error-handling.md)).
- **Keep `main.lp` readable** — extract `function` blocks ([11 — Functions](11-functions.md)).
- **Add tests/** early ([31 — Testing](31-testing.md)).
- **Read [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md)** before using beta features.

---

## Exercises

### Beginner

1. Rename a variable from `UserName` to `user_name` in a sample program.
2. Replace `"a" + "b"` with `"a" with "b"`.
3. Add missing `.` to three statements; verify with `lang check`.
4. Write an `if` block using `and` instead of nested `if`s.
5. Add one `@` comment explaining *why*, not *what*.

### Intermediate

1. Refactor a 30-line script into two functions with `snake_case` names.
2. Create a pre-run script: check → run → test.
3. Fix a program that closes a `function` with `.` instead of `..`.
4. Use parentheses in `print "Sum: " with (a + b)`.
5. Review [33 — Common Mistakes](33-common-mistakes.md) and fix one example of each category.

### Advanced

1. Write a one-page style guide for a team Lang.P project.
2. Audit [examples/](../../examples/) for `with` vs `+` — report any inconsistencies.

---

## Summary

End statements with **`.`**, blocks with **`..`**, join strings with **`with`**, name variables in **`snake_case`**, and **`lang check`** before **`lang run`**. These habits align with Lang.P v0.2.0 grammar and prevent the errors catalogued in [33 — Common Mistakes](33-common-mistakes.md).

**Previous:** [31 — Testing](31-testing.md) · **Next:** [33 — Common Mistakes](33-common-mistakes.md)

**See also:** [03 — Language Basics](03-language-basics.md), [07 — Operators](07-operators.md), [10 — Strings](10-strings.md), [30 — Debugging](30-debugging.md), [33 — Common Mistakes](33-common-mistakes.md), [Manual: Best Practices](../manual/23-best-practices.md)
