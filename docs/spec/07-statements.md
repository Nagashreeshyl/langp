# Chapter 7 — Statements

## 7.1 Statement Grammar

Every statement ends with `.` (period).

```
statement ::= assignment_statement
            | expression_statement
            | print_statement
            | return_statement
            | break_statement
            | continue_statement
            | if_statement
            | loop_statement
            | try_statement
            | use_statement
            | type_declaration
            | function_declaration
            | event_handler
            | block_statement
```

## 7.2 Assignment Statement

```lp
variable = expression.
x, y = 10, 20.
self.name = value.
items[0] = 42.
```

The built-in `input` expression is commonly used on the right-hand side of assignment:

```lp
name = input "Enter your name : ".
age = input number "Enter your age : ".
```

`input` reads like an instruction and returns a value directly — no function call syntax or parentheses. See [Chapter 6 §6.15](06-expressions.md#615-input-expression) for full syntax, type inference, and error handling.

## 7.3 Expression Statement

```lp
greet("Naga").
process(data).
```

## 7.4 Print Statement

`print` is a built-in statement (also available as a stdlib function):

```lp
print "Hello".
print username.
print "Age: " with age.
print value1, value2, value3.    @ Multiple values, space-separated
```

`print` writes to standard output followed by a newline. Use `print inline` to suppress the newline:

```lp
print inline "Loading".
print inline ".".
@ Output: Loading.
```

## 7.5 Return Statement

```lp
return.
return value.
return x, y.    @ Multiple return values (tuple)
```

`return` exits the innermost function. `return` at the top level is equivalent to `exit(0)`.

## 7.6 Break and Continue

```lp
repeat 10 times,
    if done,
        break.
    ..
    if skip,
        continue.
    ..
..
```

`break` exits the innermost loop. `continue` skips to the next iteration.

## 7.7 Block Statement

A block is a sequence of statements enclosed by `,` and `..`:

```lp
,
    step1().
    step2().
    step3().
..
```

Standalone blocks are rarely used but valid for scoping:

```lp
function example(),
    ,
        temp = compute().
        result = transform(temp).
    ..
    return result.
..
```

## 7.8 Empty Blocks

Empty blocks MUST contain at least a comment:

```lp
if debug,
    @ intentionally empty
..
```

A completely empty block is a compile warning.

## 7.9 Statement Blocks in Control Flow

Control flow constructs use the comma/dotdot block syntax:

```lp
if condition,
    statement1.
    statement2.
..
```

See [Chapter 9](09-control-flow.md) for all control flow statements.

## 7.10 Declaration Statements

Top-level and block-level declarations:

```lp
function helper(),
    @ ...
..

type Point,
    x: Float64.
    y: Float64.
..

enum Direction,
    North.
    South.
    East.
    West.
..
```

## 7.11 Import Statement

```lp
use module_name.
use parent.child_module.
```

See [Chapter 11](11-modules-imports.md).

## 7.12 Event Handler Statement

```lp
on event.source,
    handle(event).
..
```

See [Chapter 12](12-events.md).

## 7.13 Statement Ordering and Dead Code

The compiler MUST warn on unreachable statements:

```lp
function example(),
    return 42.
    print "never reached".    @ Warning: unreachable code
..
```

## 7.14 Labeled Statements

Labels allow breaking outer loops (v0.2):

```lp
@ Future syntax
outer: repeat forever,
    inner: repeat 10 times,
        break outer.
    ..
..
```

In v0.1, use helper functions or flags to break outer loops.

## 7.15 Assertion Statement

```lp
assert condition.
assert x > 0, "x must be positive".
```

In debug builds, failed assertions panic. In release builds, assertions are elided unless `--enable-assertions` is passed to `langc`.

## 7.16 Pass Statement

No-op for syntactic completeness:

```lp
if condition,
    pass.
..
```

`pass` is equivalent to an empty comment block but reads more naturally for beginners.
