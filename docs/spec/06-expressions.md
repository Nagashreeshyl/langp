# Chapter 6 — Expressions

## 6.1 Overview

An expression produces a value. Expressions appear on the right-hand side of assignments, as function arguments, and as conditions.

## 6.2 The `with` Operator

`with` is Lang.P's **universal composition operator**. It replaces `+` for string concatenation and provides intelligent composition across compatible types.

### 6.2.1 String Composition

```lp
greeting = "Hello " with name.
message = first_name with " " with last_name.
url = protocol with domain with path.
version = major with "." with minor with "." with patch.
```

`with` chains left-to-right and MUST NOT require explicit conversion for:

- `String` with `String` → `String`
- `String` with `Int` → `String` (Int converted to decimal representation)
- `String` with `Float64` → `String`
- `String` with `Bool` → `String` (`"true"` / `"false"`)
- `String` with `Char` → `String`

### 6.2.2 Print with Composition

```lp
print "Hello " with name.
print "Age: " with age.
print "Score: " with score with "/100".
```

### 6.2.3 HTTP and I/O Composition

`with` is also used for passing data in API calls:

```lp
response = post url with data.
write content to path.
```

These are syntactic forms defined in their respective chapters, not general `with` overloads.

### 6.2.4 Custom `with` Overloading

Types MAY implement the `Composable` interface to define custom `with` behavior:

```lp
interface Composable,
    function compose_with(other: Any) -> Self.
..
```

### 6.2.5 Restrictions

`+` MUST NOT be used for string concatenation. The compiler MUST emit:

```
error[E0401]: use 'with' for string composition, not '+'
  --> main.lp:3:17
   |
 3 |     msg = "Hi" + name.
   |                 ^ help: replace with 'with'
```

`+` remains valid for numeric addition only.

## 6.3 Arithmetic Operators

| Operator | Operation | Types |
|----------|-----------|-------|
| `+` | Addition | Numeric |
| `-` | Subtraction | Numeric |
| `*` | Multiplication | Numeric |
| `/` | Division | Numeric |
| `%` | Modulo | Integer |
| `**` | Exponentiation | Numeric |

```lp
sum = a + b.
area = pi * r ** 2.
```

Integer division `/` on integers produces `Float64` unless both operands are explicitly integer-divided with `//`:

```lp
half = 5 // 2.    @ 2 (Int)
precise = 5 / 2.  @ 2.5 (Float64)
```

## 6.4 Comparison Operators

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

Structurally typed equality: two values of the same type are equal if all fields are equal. Reference equality for objects uses `is` / identity (see §6.10).

## 6.5 Logical Operators

| Operator | Meaning |
|----------|---------|
| `and` | Logical AND (short-circuit) |
| `or` | Logical OR (short-circuit) |
| `not` | Logical NOT |

Keywords `and`, `or`, `not` are preferred over `&&`, `||`, `!` but both are supported.

```lp
if age >= 18 and has_license,
    allow_driving().
..

if not is_empty(list),
    process(list).
..
```

## 6.6 Bitwise Operators

| Operator | Meaning |
|----------|---------|
| `&` | Bitwise AND |
| `\|` | Bitwise OR |
| `^` | Bitwise XOR |
| `~` | Bitwise NOT |
| `<<` | Left shift |
| `>>` | Right shift |

Available for integer types only.

## 6.7 Operator Precedence

From highest to lowest:

| Precedence | Operators | Associativity |
|------------|-----------|---------------|
| 1 | Member access `.` | Left |
| 2 | Index `[]`, call `()` | Left |
| 3 | `not`, `~`, unary `-` | Right |
| 4 | `**` | Right |
| 5 | `*`, `/`, `%`, `//` | Left |
| 6 | `+`, `-` | Left |
| 7 | `<<`, `>>` | Left |
| 8 | `&` | Left |
| 9 | `^` | Left |
| 10 | `\|` | Left |
| 11 | Comparisons `==`, `!=`, `<`, `>`, `<=`, `>=` | None |
| 12 | `and` | Left |
| 13 | `or` | Left |
| 14 | `with` | Left |
| 15 | Assignment `=`, `+=`, etc. | Right |

Parentheses override precedence:

```lp
result = (a + b) * c.
```

## 6.8 Range Expressions

```lp
@ Inclusive range
for i in 1..10,
    print i.
..

@ Exclusive end
for i in 1..<10,
    print i.
..
```

Range types: `Range<T>` where `T` is `Int` or `Char`.

## 6.9 Conditional Expressions

Ternary-style expressions use `if`/`otherwise` inline:

```lp
status = if score >= 60, "pass", otherwise, "fail".
```

Alternatively, a dedicated expression form:

```lp
status = if score >= 60 then "pass" else "fail".
```

Both forms are equivalent. The block form (comma-based) is preferred for multi-line branches.

## 6.10 Identity and Type Tests

```lp
@ Type test (smart cast on success)
if value is String,
    print value.length.
..

@ Negated type test
if value is not Int,
    print "not a number".
..

@ Identity comparison
if a is b,
    print "same object".
..
```

`is` for types performs runtime type checking. `is` for values performs reference identity comparison.

## 6.11 Lambda Expressions

Anonymous functions:

```lp
double = (x) => x * 2.
add = (a, b) => a + b.

numbers.map((n) => n * 2).
```

Multi-line lambda:

```lp
process = (data) =>,
    cleaned = clean(data).
    return transform(cleaned).
..
```

## 6.12 Collection Expressions

```lp
@ List
items = [1, 2, 3].
empty: List<Int> = [].

@ Dictionary
config = {"host": "localhost", "port": 8080}.

@ Set
tags = {"lang", "compiler"}.

@ Tuple
point = (10, 20).
named = (x: 10, y: 20).
```

Spread operator:

```lp
combined = [...list1, ...list2].
merged = {**defaults, **overrides}.
```

## 6.13 Null Coalescing

```lp
name = user.name ?? "Anonymous".
port = config.port ?? 8080.
```

`??` returns the left operand if non-null, otherwise the right.

## 6.14 Expression Statements

Any expression followed by `.` is a valid statement if its value is discarded:

```lp
fetch_data().    @ Call for side effect
counter += 1.
```

## 6.15 Input Expression

Lang.P provides a built-in **input expression** for reading user input. Unlike Python's `input()` function call, `input` is a keyword expression — no parentheses are required. The prompt reads like a natural instruction.

### 6.15.1 Syntax

```
input_expression ::= "input" input_type? string_literal
input_type       ::= "text" | "number" | "decimal" | "boolean"
                   | "password" | "file" | "folder" | "date" | "color"
```

```lp
name = input "Enter your name : ".
name = input text "Enter your name : ".
age = input number "Enter your age : ".
salary = input decimal "Enter your salary : ".
online = input boolean "Are you online? ".
password = input password "Enter your password : ".
resume = input file "Choose your resume".
folder = input folder "Select a folder".
birth_date = input date "Select your birth date".
theme_color = input color "Choose a theme color".
```

Rules:

- `input` MUST be followed by an optional input type keyword and a string literal prompt.
- Parentheses MUST NOT be used around the prompt.
- The prompt string MUST NOT be omitted.
- `input` is an **expression** — it produces a value and MAY appear anywhere an expression is valid (assignment, function arguments, conditions, etc.).

### 6.15.2 Input Types and Return Types

| Input type keyword | Return type | Behavior |
|--------------------|-------------|----------|
| *(none — default)* | Inferred or `String` | Text line from stdin; type inferred from context when possible |
| `text` | `String` | Single line of text from stdin |
| `number` | `Int` | Integer input with validation |
| `decimal` | `Float64` | Decimal input with validation |
| `boolean` | `Bool` | Yes/no confirmation (`true` / `false`) |
| `password` | `String` | Masked text input (characters hidden) |
| `file` | `String` | Native file picker; returns absolute file path |
| `folder` | `String` | Native folder picker; returns absolute directory path |
| `date` | `Date` | Native date picker |
| `color` | `Color` | Native color picker |

See [Chapter 4 §4.14](04-types.md#414-input-expression-types) for type definitions.

### 6.15.3 Semantics

**Text input (`text` or default):**

1. The runtime writes the prompt to standard output (without a trailing newline unless the prompt ends with one).
2. The runtime reads a line from standard input.
3. Trailing newline characters are stripped.
4. The resulting `String` is returned.

**Numeric input (`number`, `decimal`):**

1. The prompt is displayed as for text input.
2. The runtime reads and validates input against the target type.
3. On invalid input, the runtime MUST re-prompt with a beginner-friendly message (see §6.15.6).
4. On end-of-input (EOF / Ctrl+D), the runtime MUST throw `InputError`.

**Boolean input (`boolean`):**

1. The prompt is displayed.
2. The runtime accepts affirmative responses (`y`, `yes`, `true`, `1`) and negative responses (`n`, `no`, `false`, `0`), case-insensitive.
3. Invalid responses trigger re-prompting.

**Password input (`password`):**

1. The prompt is displayed.
2. Characters are read from stdin without echo (masked with `*` or platform equivalent).
3. Returns the entered text as `String`.

**Picker input (`file`, `folder`, `date`, `color`):**

1. The prompt MAY be shown in a dialog title or status area.
2. The runtime opens the platform-native picker (file dialog, folder dialog, date picker, color picker).
3. On confirmation, returns the selected value.
4. On cancellation, throws `InputCancelledError` (see §6.15.6).

**Headless / non-interactive environments:**

When stdin is not a TTY and no GUI is available:

- Text-based input (`text`, `number`, `decimal`, `boolean`, `password`) reads from stdin as usual.
- Picker-based input (`file`, `folder`, `date`, `color`) MUST throw `InputError` with message indicating that an interactive display is required.

### 6.15.4 Type Inference

When no input type keyword is given, the compiler infers the expected type from context:

```lp
@ Inferred as String (no contextual type)
name = input "Enter your name : ".

@ Inferred as Int (annotated binding)
age: Int = input "Enter your age : ".

@ Inferred as Int (used in numeric context)
age = input "Enter your age : ".
print "Next year: " with (age + 1).
```

When inference is ambiguous, the compiler defaults the expression type to `String` and MAY emit warning `W0101`:

```
warning[W0101]: input type could be more specific
  --> main.lp:3:7
   |
 3 | age = input "Age : ".
   |       ^^^^^^^^^^^^^^ the value "age" appears to be used as a number
   |
  = help: consider using: age = input number "Age : ".
```

Implementations MUST emit `W0101` when:

1. No input type keyword is present, **and**
2. The assigned variable is later used in a context that strongly suggests a non-`String` type (`Int`, `Float64`, `Bool`, `Date`, or `Color`), **and**
3. No explicit type annotation resolves the ambiguity.

The IDE SHOULD offer a quick-fix to insert the suggested input type keyword (see [Chapter 21 §21.3.2](21-tooling.md#2132-input-type-quick-fix)).

Explicit typed input suppresses `W0101`:

```lp
age = input number "Age : ".    @ No warning
```

### 6.15.5 Usage in Assignment and Expressions

The most common form assigns the result directly to a variable:

```lp
name = input "Enter your name : ".
print "Hello " with name.
```

`input` MAY appear in any expression context:

```lp
if input boolean "Continue? ",
    process().
..

greet(input text "Your name : ").

items.add(input number "Add item : ").
```

See [Chapter 7 §7.2](07-statements.md#72-assignment-statement) for assignment statement rules.

### 6.15.6 Error Handling

Input-related errors:

| Error | When |
|-------|------|
| `InputError` | Invalid input after maximum retries; EOF on required input; picker unavailable in headless mode |
| `InputCancelledError` | User cancelled a native picker (`file`, `folder`, `date`, `color`) |
| `InputValidationError` | Parsed value out of allowed range (when bounds are specified — v0.2) |

Both extend `RuntimeError`. Handle with `try`/`catch`:

```lp
try,
    resume = input file "Choose your resume".
    print "Selected: " with resume.
catch error: InputCancelledError,
    print "No file selected.".
catch error: InputError,
    print "Input failed: " with error.message.
..
```

**Invalid number input (Conformance):**

Given `input number "Enter your age : "` and the user types `abc`:

1. The runtime MUST display a clear message: `"Please enter a whole number."` (or equivalent).
2. The runtime MUST re-prompt at least once.
3. If all retries fail, the runtime MUST throw `InputError`.

**Cancelled file picker (Conformance):**

Given `input file "Choose a file"` and the user clicks Cancel:

1. The runtime MUST throw `InputCancelledError`.
2. The error message MUST indicate cancellation, not a missing file.

**Invalid syntax (Compile-time):**

```
error[E0201]: expected string literal after input
  --> main.lp:2:14
   |
 2 | age = input number prompt_var.
   |              ^^^^^^^^^^^^^^^ expected string literal prompt
   |
  = help: use a string literal: input number "Enter your age : "

error[E0201]: input does not take parentheses
  --> main.lp:2:14
   |
 2 | age = input("Age : ").
   |              ^ help: remove parentheses: input "Age : "
```

### 6.15.7 Relationship to Standard Library

The built-in `input` expression is the **primary** mechanism for interactive terminal and picker input in Lang.P. The `terminal` module (see [Chapter 16 §16.7](16-standard-library.md#167-terminal-terminal)) provides advanced formatting, colors, and tables — not basic line input.

The `read_line` and `read_line_masked` functions in [Chapter 15 §15.2](15-io-network.md#152-standard-io) remain available for library authors and scripting but SHOULD NOT be taught to beginners in place of `input`.

### 6.15.8 Conformance Examples

**Conformance:** Default text input returns a `String`:

```lp
@ Given stdin line "Naga\n"
name = input "Enter your name : ".
@ name == "Naga" (String)
```

**Conformance:** Typed number input returns `Int`:

```lp
@ Given stdin line "25\n"
age = input number "Enter your age : ".
@ age == 25 (Int)
```

**Conformance:** Password input masks characters and returns `String`:

```lp
@ Given masked input "secret\n"
password = input password "Password : ".
@ password == "secret" (String); characters were not echoed
```

**Conformance:** File picker returns absolute path as `String`:

```lp
@ Given user selects /home/user/resume.pdf
resume = input file "Choose your resume".
@ resume == "/home/user/resume.pdf" (String)
```

## 6.16 Constant Expressions

Expressions that can be evaluated at compile time:

- Literals
- Constant arithmetic
- Const variable references
- Enum variant references

Used for array sizes, compile-time configuration, and optimizer hints.
