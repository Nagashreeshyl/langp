# Lang.P Language Specification (Complete)

**Version:** 0.1.0-draft
**Status:** Phase 1 — Specification
**Generated from:** Individual chapter files in `docs/spec/`

---

# Chapter 1 — Introduction

## 1.1 Purpose

Lang.P (spoken name: **Lang**) is a general-purpose, statically typed programming language with type inference. It is designed for readability first: source code should read like natural instructions while remaining expressive enough for production systems.

This specification defines the syntax, semantics, standard library interfaces, and tooling contracts for Lang.P implementations.

## 1.2 Design Goals

| Goal | Description |
|------|-------------|
| **Readability** | A person with no programming experience should understand most code after one reading. |
| **Simplicity** | One obvious way to accomplish common tasks. |
| **Power** | Full support for OOP, generics, async, events, and systems programming (future). |
| **Toolability** | First-class IDE, LSP, debugger, and formatter support. |
| **Library growth** | Language features grow through libraries, not keyword proliferation. |
| **AI-native** | Built-in AI framework and MCP integration. |

## 1.3 Non-Goals

Lang.P is **not**:

- A domain-specific language (DSL) — it is a complete general-purpose language.
- A macro-heavy metaprogramming language — reflection exists but is constrained.
- A language that sacrifices safety for brevity — static analysis is mandatory.

## 1.4 Target Domains

Lang.P MUST support building:

- CLI tools and automation scripts
- HTTP APIs and web services
- Desktop applications and browsers (via Navigator)
- AI agents and LLM-powered applications
- Games and multimedia (via Graphics/Audio/Video stdlib)
- Mobile apps (future, Phase 15+)
- Operating systems components (future, Phase 20+)

## 1.5 Naming Conventions

| Artifact | Name |
|----------|------|
| Language | Lang.P |
| Spoken name | Lang |
| Source file extension | `.lp` |
| Compiler CLI | `langc` |
| Package manager CLI | `lang` |
| IDE | Lang Studio |
| Language Server | Lang LSP |

## 1.6 Relationship to Other Languages

Lang.P draws inspiration from:

- **Python** — indentation-based blocks, dynamic-feeling syntax with static types
- **Go** — simplicity, one obvious way, tooling-first culture
- **Kotlin** — null safety, extension functions, coroutines-style async
- **Swift** — readable keyword choices, protocol-oriented design
- **TypeScript** — structural typing, gradual adoption of types via inference

Lang.P deliberately avoids:

- C-style braces `{}` for blocks
- Semicolon-terminated statements
- `+` for string concatenation (uses `with` instead)
- `class` keyword (uses `type` instead)
- `//` or `#` comments (uses `@` instead)

## 1.7 Specification Organization

Chapters 2–15 define the core language. Chapters 16–18 define standard library frameworks. Chapters 19–22 define runtime, packaging, tooling, and versioning.

## 1.8 Example Program

The following program demonstrates core Lang.P syntax:

```lp
@ main.lp — A simple HTTP greeting server.

use network.

function handle_request(request),
    name = request.query.get("name", default = "World").
    body = "Hello " with name with "!".
    return response(200, body = body, content_type = "text/plain").
.

server = Server(port = 8080).

on server.request,
    reply = handle_request(server.request).
    server.respond(reply).
.

print "Server running on port 8080".
server.start().
```

## 1.9 Document Conventions

- **Grammar productions** appear in monospace with `|` for alternatives.
- **Keywords** are written in lowercase and are reserved.
- **Placeholders** in examples use angle brackets: `<name>`.
- Line numbers in error messages are 1-indexed.

## 1.10 Versioning

This specification is version **0.1.0**. See [Chapter 22](22-compatibility-versioning.md) for semver policy.

---

# Chapter 2 — Lexical Structure

## 2.1 Source Files

A Lang.P source file MUST:

- Use the `.lp` file extension.
- Be encoded in UTF-8 without a byte-order mark (BOM).
- Use Unix line endings (`\n`) in canonical form; tools MUST accept `\r\n`.

## 2.2 Comments

Comments begin with `@` and extend to the end of the line.

```lp
@ This is a comment.
print "Hello".  @ Inline comments are allowed after statements.
```

Rules:

- Comments MUST NOT nest.
- Comments are treated as whitespace by the parser.
- Multi-line comments require a `@` on each line:

```lp
@ Line one of the comment.
@ Line two of the comment.
```

There is no block comment syntax. This is intentional — `@` reads as "note" or "annotation" and is visually distinct from code.

## 2.3 Whitespace

Whitespace consists of:

- Space (`U+0020`)
- Tab (`U+0009`) — discouraged; formatters SHOULD convert tabs to spaces
- Newline (`U+000A`, `U+000D U+000A`)

Whitespace is significant **only** for indentation within blocks (see §2.7).

## 2.4 Statement Terminator

Every statement MUST end with a period (`.`).

```lp
age = 18.
print "Hello".
```

The period is the statement terminator, analogous to a semicolon in C-family languages but chosen for readability — it marks the end of an instruction, like a sentence.

### 2.4.1 Trailing Period in Blocks

The closing block marker `..` does NOT require a preceding period on the same line, but the last statement inside a block MUST still end with `.`:

```lp
if true,
    print "Yes".   @ Required period
..
```

## 2.5 Block Delimiters

Blocks begin with a comma (`,`) and end with double-period (`..`).

```lp
if age >= 18,
    print "Adult".
..
```

Rules:

- The comma MUST appear at the end of the line that opens the block (or after the block header expression).
- The `..` MUST appear at the beginning of the indentation level of the block opener (dedented).
- Blocks MUST contain at least one statement or a blank-line placeholder comment.

### 2.5.1 Indentation

- Indentation MUST use spaces only.
- Each indentation level is **4 spaces**.
- The compiler, formatter, and IDE MUST enforce consistent indentation.
- Mixing indentation levels within a block is a **compile error**.

The IDE automatically inserts indentation when `,` is typed and dedents when `..` is typed.

## 2.6 Identifiers

```
identifier ::= (letter | "_") (letter | digit | "_")*
letter     ::= "a"..."z" | "A"..."Z"
digit      ::= "0"..."9"
```

Rules:

- Identifiers are case-sensitive: `name` and `Name` are distinct.
- Identifiers MUST NOT begin with a digit.
- Identifiers MUST NOT equal any reserved keyword (§2.8).
- Unicode identifiers are NOT supported in v0.1; this MAY be added in a future version.
- By convention, `SCREAMING_SNAKE_CASE` is used for module-level constants.

## 2.7 Literals

### 2.7.1 Integer Literals

```
integer_literal ::= decimal_integer | hex_integer | binary_integer | octal_integer
decimal_integer ::= digit+
hex_integer     ::= "0x" hex_digit+
binary_integer  ::= "0b" ("0" | "1")+
octal_integer   ::= "0o" digit+
```

Examples: `42`, `0xFF`, `0b1010`, `0o755`

Integers are arbitrary-precision at compile time; runtime integers are platform-sized (`Int`) or explicitly `Int64`.

### 2.7.2 Float Literals

```
float_literal ::= digit+ "." digit+ (exponent)? | digit+ exponent
exponent        ::= ("e" | "E") ("+" | "-")? digit+
```

Examples: `3.14`, `95.6`, `1.0e-10`, `2E+5`

Default float type is `Float64`.

### 2.7.3 Boolean Literals

```
boolean_literal ::= "true" | "false"
```

### 2.7.4 String Literals

```
string_literal ::= '"' (escape_sequence | non_quote_char)* '"'
                 | "'" (escape_sequence | non_apostrophe_char)* "'"
```

Both double-quoted and single-quoted strings are supported. They are equivalent.

Escape sequences:

| Sequence | Meaning |
|----------|---------|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quote |
| `\'` | Apostrophe |
| `\u{XXXX}` | Unicode code point (hex) |

Examples:

```lp
name = "Naga".
greeting = 'Hello'.
path = "C:\\Users\\Naga".
emoji = "\u{1F600}".
```

Raw strings (no escape processing) use triple quotes:

```
raw_string ::= '"""' any_char* '"""'
```

```lp
regex = """\d+\.\d+""".
```

### 2.7.5 Character Literals

```
character_literal ::= "'" (escape_sequence | non_apostrophe_char) "'"
```

Character literals represent a single Unicode scalar value of type `Char`.

```lp
letter = 'A'.
newline = '\n'.
```

### 2.7.6 Null Literal

```
null_literal ::= "null"
```

`null` represents the absence of a value for nullable types.

## 2.8 Reserved Keywords

The following tokens are reserved and MUST NOT be used as identifiers:

```
and         as          async       await       break
catch       continue    else        enum        false
finally     for         forever     function    if
in          input       interface   let         match
not         null        on          or          otherwise
repeat      return      self        static      super
this        true        try         type        use
wait        while       with
```

Additionally, these contextual keywords are reserved in their syntactic positions:

```
otherwise if    @ Two-token keyword for else-if
repeat forever  @ Two-token keyword for infinite loop
wait for        @ Two-token keyword for async await
```

### 2.8.1 Input Type Keywords

The following tokens are **contextual keywords** — reserved only immediately after the `input` keyword (see [Chapter 6 §6.15](06-expressions.md#615-input-expression)). Outside that position, they MAY be used as identifiers:

```
boolean     color       date        decimal     file
folder      number      password    text
```

Example disambiguation:

```lp
file = input file "Choose a file".    @ `file` (left) is identifier; `file` (after input) is keyword
text = input text "Enter text : ".   @ `text` keyword selects text input mode
```

## 2.9 Operators and Delimiters

| Token | Name |
|-------|------|
| `.` | Statement terminator / member access |
| `,` | Block opener / separator |
| `..` | Block closer |
| `=` | Assignment / default parameter |
| `==` `!=` `<` `>` `<=` `>=` | Comparison |
| `+` `-` `*` `/` `%` | Arithmetic |
| `**` | Exponentiation |
| `&` `\|` `^` `~` | Bitwise |
| `<<` `>>` | Bit shifts |
| `&&` `\|\|` | Logical |
| `!` | Logical not |
| `?` | Nullable / optional |
| `:` | Type annotation / map entry |
| `(` `)` | Grouping / call |
| `[` `]` | Index / list |
| `{` `}` | Dictionary / set (collection literals only) |
| `->` | Function return type annotation |
| `=>` | Lambda / match arm |
| `@` | Comment |
| `..<` | Range (exclusive end) |
| `...` | Spread / variadic |

Note: `+` exists for arithmetic but MUST NOT be used for string concatenation — use `with` (see Chapter 6).

## 2.10 Tokenization Rules

1. The lexer MUST use maximal munch — the longest valid token is chosen.
2. `..` is a single token (block closer), not two periods.
3. `...` is a single token (spread), distinct from `..`.
4. `..<` is a single token (exclusive range).
5. `otherwise if` is tokenized as a single keyword when `otherwise` is followed by whitespace and `if`.
6. `repeat forever` and `wait for` follow the same two-token keyword rule.
7. A period (`.`) at the end of a statement is the terminator, not member access. The lexer uses context: if `.` is followed by whitespace or newline, it is a terminator; if followed by an identifier, it is member access.

### 2.10.1 Disambiguation Example

```lp
user.name = "Naga".     @ `.name` is member access; final `.` is terminator
print user.name.        @ member access then terminator
```

## 2.11 Line Continuation

Statements MUST NOT span multiple lines unless inside a block, parentheses, brackets, or a string literal. There is no line-continuation character.

```lp
@ Valid — inside parentheses
result = calculate(
    value1,
    value2
).

@ Invalid — statement split across lines
print "Hello"
    with name.
```

---

# Chapter 3 — Program Structure

## 3.1 Compilation Units

A Lang.P **compilation unit** is a single `.lp` source file. A **program** is one or more compilation units linked together with the package system (see Chapter 20).

## 3.2 Entry Point

A program MUST designate an entry point. Conventions:

| Entry file | Purpose |
|------------|---------|
| `main.lp` | Default executable entry point |
| `lib.lp` | Library module entry (no `main`) |

An executable program MUST contain a `main.lp` file with top-level statements or a `main` function:

```lp
@ Option A — top-level statements
print "Starting application".
run_app().

@ Option B — main function
function main(),
    print "Starting application".
    run_app().
.

@ langc invokes main() automatically if present
```

Rules:

- If both top-level statements and `main()` exist, top-level statements execute first, then `main()`.
- Library packages MUST NOT define a `main()` function.

## 3.3 Top-Level Declarations

The following MAY appear at the top level of a compilation unit:

- `use` import statements
- `function` definitions
- `type` definitions
- `enum` definitions
- Variable bindings
- Top-level statements (executable code)

```lp
use network.
use json.

API_URL = "https://api.example.com".

type Config,
    host.
    port.
.

function load_config() -> Config,
    @ ...
.

config = load_config().
```

## 3.4 Execution Order

Top-level declarations are **hoisted** for name resolution but executed in source order:

1. All `use` imports are resolved first.
2. Type and function declarations are registered.
3. Top-level variable initializers run in source order.
4. Top-level statements run in source order.
5. `main()` is invoked if present.

## 3.5 Module Boundaries

Each `.lp` file is a **module**. The module name defaults to the file path relative to the package root, with path separators replaced by dots:

```
src/network/http.lp  →  network.http
main.lp              →  main
```

Modules MAY explicitly declare a name:

```lp
module network.http.
```

Explicit module names MUST match the file path convention or a compile warning is emitted.

## 3.6 Visibility

Lang.P has three visibility levels:

| Modifier | Scope | Syntax |
|----------|-------|--------|
| `public` | Exported from module (default for `type` members in public API) | `public name` |
| `internal` | Visible within the package | `internal name` |
| `private` | Visible within the file | `private name` |

If no modifier is specified:

- Top-level functions and types default to `public`.
- Type members default to `public`.
- Module-level variables default to `internal`.

```lp
private helper_cache = {}.

public function fetch(url),
    @ ...
.

type User,
    public name.
    internal id.
    private password_hash.
.
```

## 3.7 Namespaces

Imported modules are accessed via dot notation:

```lp
use json.

data = json.parse('{"name": "Naga"}').
print data.name.
```

Wildcard imports are NOT supported — every import MUST be explicit:

```lp
@ Valid
use json.

@ Invalid — wildcard imports are forbidden
use json.*.
```

This ensures readability: the origin of every name is traceable.

## 3.8 Conditional Compilation

Lang.P supports compile-time conditions via `when` blocks (v0.2 planned). In v0.1, use runtime checks or separate build targets.

## 3.9 Embedded Resources

The `embed` directive (v0.2 planned) will allow embedding files at compile time:

```lp
@ Future syntax
embed logo = "assets/logo.png".
```

In v0.1, use the `filesystem` standard library to read resources at runtime.

## 3.10 Program Lifecycle

```
┌─────────────┐
│   Compile   │  langc: lex → parse → analyze → codegen
└──────┬──────┘
       ▼
┌─────────────┐
│    Link     │  Resolve dependencies via lang package manager
└──────┬──────┘
       ▼
┌─────────────┐
│  Initialize │  Runtime init: GC, stdlib, module loaders
└──────┬──────┘
       ▼
┌─────────────┐
│   Execute   │  Top-level code → main() → event loop (if applicable)
└──────┬──────┘
       ▼
┌─────────────┐
│  Shutdown   │  finally blocks, resource cleanup, GC sweep
└─────────────┘
```

Applications using Navigator or async event loops enter a runtime event loop after initialization. The loop terminates when the application calls `exit()` or all non-daemon tasks complete.

---

# Chapter 4 — Types

## 4.1 Overview

Lang.P is **statically typed** with **complete type inference**. Programmers rarely write explicit type annotations — the compiler infers types at compile time and reports errors before execution.

Type annotations are optional and used for:

- Public API documentation
- Disambiguation when inference is insufficient
- Generic type parameters

```lp
@ Inferred
age = 18.                    @ Int
name = "Naga".               @ String
scores = [95, 87, 92].       @ List<Int>

@ Explicit (optional)
count: Int = 0.
rate: Float64 = 3.14.
items: List<String> = [].
```

## 4.2 Primitive Types

| Type | Description | Literal examples |
|------|-------------|------------------|
| `Int` | Signed integer (platform word size) | `42`, `-7` |
| `Int8` `Int16` `Int32` `Int64` | Fixed-width signed integers | `42i64` |
| `UInt` `UInt8` ... `UInt64` | Unsigned integers | `42u32` |
| `Float32` `Float64` | IEEE 754 floating point | `3.14`, `1.0e-5` |
| `Bool` | Boolean | `true`, `false` |
| `Char` | Unicode scalar value | `'A'`, `'\n'` |
| `String` | UTF-8 string (immutable) | `"hello"` |
| `Null` | Null type (only value: `null`) | `null` |

### 4.2.1 Numeric Type Inference

- Integer literals default to `Int`.
- Float literals default to `Float64`.
- Mixed numeric operations promote to the wider type following standard rules.

## 4.3 Composite Types

### 4.3.1 Array

Fixed-size, homogeneous collection. Type: `Array<T, N>` where `N` is a compile-time constant.

```lp
@ Inferred as Array<Int, 3>
nums = [1, 2, 3].

@ Explicit size
buffer: Array<Byte, 256> = Array(256).
```

### 4.3.2 List

Dynamic-size, homogeneous collection. Type: `List<T>`.

```lp
users = [].
users = ["Alice", "Bob"].
users.add("Charlie").
```

### 4.3.3 Tuple

Fixed-size, heterogeneous collection. Type: `(T1, T2, ...)`.

```lp
point = (10, 20).
x = point.0.
y = point.1.

@ Named tuple fields
person = (name: "Naga", age: 25).
print person.name.
```

### 4.3.4 Dictionary

Key-value map. Type: `Dictionary<K, V>`.

```lp
config = {"host": "localhost", "port": 8080}.
config["host"] = "127.0.0.1".
```

Keys MUST be hashable types (`String`, `Int`, `Char`, and types implementing `Hashable`).

### 4.3.5 Set

Unordered unique collection. Type: `Set<T>`.

```lp
tags = {"lang", "compiler", "lang"}.
@ Result: {"lang", "compiler"}
```

## 4.4 User-Defined Types

### 4.4.1 Type (Object)

Defined with the `type` keyword. See [Chapter 10](10-object-model.md).

```lp
type User,
    name: String.
    age: Int.
.
```

### 4.4.2 Enum

```lp
enum Color,
    Red.
    Green.
    Blue.
.

enum Status,
    Active(value: Int).
    Inactive.
    Pending(reason: String).
.
```

Enums with associated values support pattern matching.

### 4.4.3 Interface

```lp
interface Drawable,
    function draw() -> Void.
    function bounds() -> (Int, Int, Int, Int).
.
```

### 4.4.4 Type Alias

```lp
type UserId = Int.
type Handler = function(Request) -> Response.
```

## 4.5 Function Types

Functions are first-class values with typed signatures:

```
function Type ::= "function" "(" ParamTypeList? ")" "->" Type
                | "function" "(" ParamTypeList? ")"  @ return type inferred
```

```lp
@ Type: function(Int, Int) -> Int
add: function(Int, Int) -> Int = function(a, b),
    return a + b.
.

result = add(3, 4).
```

## 4.6 Generic Types

Generics use angle-bracket syntax:

```lp
type Box<T>,
    value: T.

    function get() -> T,
        return self.value.
    .
.

box = Box<Int>(value = 42).
```

Generic constraints:

```lp
type Container<T: Comparable>,
    items: List<T>.
.
```

Multiple constraints: `T: Comparable & Serializable`.

## 4.7 Optional and Nullable Types

### 4.7.1 Nullable (`?`)

Any type `T` can be made nullable as `T?`, allowing `null`:

```lp
name: String? = null.
if name != null,
    print name.    @ Smart cast: name is String here
.
```

### 4.7.2 Optional (`Optional<T>`)

Explicit optional type, equivalent to `T?` but used in API signatures for clarity:

```lp
function find_user(id: Int) -> Optional<User>,
    @ returns null if not found
.
```

The `?` suffix is syntactic sugar for `Optional<T>`.

## 4.8 Type Inference Rules

The compiler MUST infer types for:

1. Variable bindings without annotations
2. Function return types (when body is present)
3. Generic type parameters (when unambiguous)
4. Lambda / closure parameter types (from context)
5. Collection literals (from elements or empty context)

The compiler MUST NOT infer:

1. Empty collection types without context — `items = [].` is an error unless annotated:

```lp
@ Error: cannot infer type of empty list
items = [].

@ Fix
items: List<String> = [].
```

2. Function parameter types without annotations in top-level functions:

```lp
@ Parameters MUST be inferrable or annotated
function greet(name),    @ OK — name inferred as Any if no constraint; SHOULD annotate public APIs
    print name.
.
```

## 4.9 Type Compatibility

Lang.P uses **structural typing** for interfaces and **nominal typing** for `type` inheritance.

| Relation | Rule |
|----------|------|
| Assignment | `T` assigned to `U` if `T` is a subtype of `U` |
| Subtyping | Class inheritance, interface implementation |
| Structural | A type satisfies an interface if it implements all required members |
| Union | `T \| U` accepts values of either type (v0.2) |

## 4.10 Type Coercion

Lang.P performs **minimal implicit coercion**:

| From | To | Context |
|------|-----|---------|
| `Int` | `Float64` | Arithmetic with float |
| `Int` | `String` | Via `with` composition |
| `Char` | `String` | Via `with` composition |
| `T` | `T?` | Assignment to nullable |
| Subclass | Superclass | Polymorphism |

No implicit coercion between `String` and `Int` via `+`. Use explicit conversion:

```lp
text = to_string(42).
number = parse_int("42").
```

## 4.11 Type Reflection

Runtime type inspection is available via the `reflect` module (stdlib):

```lp
use reflect.

print reflect.type_of(value).
print reflect.is_instance_of(value, User).
```

Reflection MUST NOT break static type safety — `reflect` operations return nullable or `Any` types requiring explicit handling.

## 4.12 The `Any` Type

`Any` is the top type — every value is assignable to `Any`. Using `Any` disables static checking for that value until a type test or cast:

```lp
value: Any = 42.
value = "hello".

if value is String,
    print value.    @ Smart cast to String
.
```

## 4.13 The `Void` Type

`Void` represents no return value. Functions without a return type that fall off the end return `Void`:

```lp
function log_message(msg: String),
    print msg.
    @ implicit return void
.
```

`Void` is not a valid type for variables.

## 4.14 Input Expression Types

The built-in `input` expression (see [Chapter 6 §6.15](06-expressions.md#615-input-expression)) returns a typed value based on the input type keyword. This section defines the return types and their semantics.

### 4.14.1 Return Type Table

| Input form | Static return type | Notes |
|------------|-------------------|-------|
| `input "..."` | `String` (default) or inferred | Compiler infers from assignment context; see §6.15.4 |
| `input text "..."` | `String` | Always text |
| `input number "..."` | `Int` | Validated integer |
| `input decimal "..."` | `Float64` | Validated floating-point |
| `input boolean "..."` | `Bool` | `true` or `false` |
| `input password "..."` | `String` | Masked; same type as text |
| `input file "..."` | `String` | Absolute file path |
| `input folder "..."` | `String` | Absolute directory path |
| `input date "..."` | `Date` | Calendar date (no time component) |
| `input color "..."` | `Color` | RGBA color value |

### 4.14.2 The `Date` Type

`Date` represents a calendar date (year, month, day) without a time zone or time-of-day component. It is defined in the `datetime` module and returned by `input date`:

```lp
birth_date = input date "Select your birth date".
print birth_date.year.
print birth_date.month.
print birth_date.day.
formatted = birth_date.format("%Y-%m-%d").
```

`Date` values are comparable and support arithmetic with `datetime.Duration`.

### 4.14.3 The `Color` Type

`Color` represents an RGBA color selected via the native color picker:

```lp
theme_color = input color "Choose a theme color".
print theme_color.red.      @ 0.0 – 1.0
print theme_color.green.
print theme_color.blue.
print theme_color.alpha.
hex = theme_color.to_hex(). @ e.g. "#FF5733"
```

`Color` is defined in the `graphics` module. Picker-based input is the primary way beginners obtain `Color` values; advanced use cases may construct colors programmatically.

### 4.14.4 Path Types for File and Folder Input

`input file` and `input folder` return `String` paths (not a dedicated `Path` type in v0.1). Paths are normalized to absolute form by the runtime. Use `filesystem.path` for manipulation:

```lp
resume = input file "Choose your resume".
name = path.filename(resume).
```

### 4.14.5 Type Inference for Default Input

When `input` appears without a type keyword:

| Context | Inferred type |
|---------|---------------|
| `x: Int = input "..."` | `Int` |
| `x: Float64 = input "..."` | `Float64` |
| `x: Bool = input "..."` | `Bool` |
| `x: Date = input "..."` | `Date` |
| `x: Color = input "..."` | `Color` |
| `x = input "..."` (no annotation, no use) | `String` |
| Used in `x + 1` | `Int` (with `W0101` if keyword omitted) |
| Used in `x + 0.5` | `Float64` (with `W0101` if keyword omitted) |

Explicit input type keywords always take precedence over inference.

### 4.14.6 Type Errors

Assigning incompatible input types is a compile error:

```
error[E0301]: type mismatch
  --> main.lp:3:7
   |
 3 | age: String = input number "Age : ".
   |               ^^^^^^^^^^^^^^^^^^^^^^ expected String, found Int
   |
  = help: remove the annotation or use input text
```

## 4.15 Type Errors

The compiler MUST report type errors with:

- File path and line number
- Expected vs. actual type
- Suggestion for fix (when possible)

Example:

```
error[E0301]: type mismatch
  --> src/main.lp:5:5
   |
 5 |     result: String = 42.
   |                      ^^ expected String, found Int
   |
   = help: use to_string(42) to convert
```

---

# Chapter 5 — Variables & Assignment

## 5.1 Variable Declarations

Variables are declared by assignment, exactly like Python:

```lp
age = 18.
name = "Naga".
marks = 95.6.
online = true.
users = [].
```

No explicit datatype declaration is required — the compiler infers the type statically.

Optional type annotations precede the value:

```lp
count: Int = 0.
name: String = "Naga".
```

## 5.2 Mutability

By default, variables are **mutable** (`let` is not required but available for clarity):

```lp
let x = 10.
x = 20.    @ OK
```

Immutable bindings use `let` with no reassignment, or the `const` keyword:

```lp
const PI = 3.14159.
PI = 3.14.    @ Compile error: cannot assign to const

let immutable_name = "Naga".
immutable_name = "Bob".    @ Compile error
```

### 5.2.1 Mutable vs Immutable Semantics

| Binding | Reassign | Mutate contents |
|---------|----------|-----------------|
| `var x = ...` / `x = ...` | Yes | Yes (if type is mutable) |
| `let x = ...` | No | Yes (if type is mutable) |
| `const X = ...` | No | No |

```lp
let list = [1, 2, 3].
list.add(4).       @ OK — mutating contents
list = [5, 6].     @ Error — rebinding
```

## 5.3 Assignment

Simple assignment:

```lp
x = 10.
name = "Naga".
```

Multiple assignment (destructuring):

```lp
x, y = 10, 20.
name, age = person.name, person.age.

(a, b), c = (1, 2), 3.
```

Compound assignment:

```lp
count += 1.
total -= amount.
text = text with " more".
```

## 5.4 Scope

Lang.P uses lexical (static) scoping.

### 5.4.1 Block Scope

Variables declared inside a block are visible only within that block:

```lp
if true,
    local = 42.
    print local.
.
@ print local.    @ Error: local not in scope
```

### 5.4.2 Function Scope

Parameters and local variables are scoped to the function body:

```lp
function process(data),
    temp = transform(data).
    return temp.
.
```

### 5.4.3 Module Scope

Top-level variables are visible throughout the module:

```lp
CONFIG = load_config().

function run(),
    print CONFIG.host.
.
```

### 5.4.4 Closure Capture

Inner functions capture outer variables by reference (for mutable variables) or by value (for immutable `let`/`const`):

```lp
function make_counter(),
    count = 0.
    return function(),
        count += 1.
        return count.
    .
.

counter = make_counter().
print counter().    @ 1
print counter().    @ 2
```

## 5.5 Shadowing

Variable shadowing is permitted in inner scopes:

```lp
x = 10.
if true,
    x = 20.    @ shadows outer x
    print x.   @ 20
.
print x.       @ 10
```

Shadowing MUST NOT cross function boundaries in a way that breaks type safety.

## 5.6 Uninitialized Variables

All variables MUST be initialized before use. The compiler MUST reject:

```lp
x: Int.
print x.    @ Error: variable x used before initialization
```

Exception: type members are initialized via constructor or default values.

## 5.7 Global Variables

Module-level mutable globals are discouraged. The linter SHOULD warn on mutable top-level `var` bindings. Prefer:

```lp
const MAX_RETRIES = 3.

function get_config() -> Config,
    return load_config().
.
```

## 5.8 Environment Variables

Access environment variables via the `env` module:

```lp
api_key = env.get("GROQ_API_KEY").
api_key = env.GROQ_API_KEY.    @ dot syntax for known keys
```

Missing keys return `null` (nullable) unless a default is specified:

```lp
port = env.get("PORT", default = "8080").
```

## 5.9 Property Access

Object and module members use dot notation:

```lp
print user.name.
print user.address.city.
```

Safe navigation for nullable receivers:

```lp
city = user?.address?.city.    @ Returns null if any link is null
```

## 5.10 Index Access

Arrays, lists, dictionaries, and strings support indexing:

```lp
first = items[0].
config["host"] = "localhost".
char = text[0].
```

Slicing:

```lp
sub = text[0..5].       @ Inclusive start, exclusive end
sub = text[2..].        @ From index 2 to end
sub = text[..5].        @ From start to index 5
```

Slice syntax: `[start..end]`, `[start..]`, `[..end]`, `[..]`.

## 5.11 Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Variables | snake_case | `user_name` |
| Functions | snake_case | `fetch_data` |
| Types | PascalCase | `User`, `HttpClient` |
| Constants | SCREAMING_SNAKE | `MAX_SIZE` |
| Enum variants | PascalCase | `Color.Red` |
| Private members | leading underscore (convention) | `_cache` |
| Module files | snake_case | `http_client.lp` |

These are conventions, not enforced by the compiler (except visibility modifiers).

---

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
.
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
.

if not is_empty(list),
    process(list).
.
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
.

@ Exclusive end
for i in 1..<10,
    print i.
.
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
.

@ Negated type test
if value is not Int,
    print "not a number".
.

@ Identity comparison
if a is b,
    print "same object".
.
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
.
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
.

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

---

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
    .
    if skip,
        continue.
    .
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
.
```

Standalone blocks are rarely used but valid for scoping:

```lp
function example(),
    ,
        temp = compute().
        result = transform(temp).
    .
    return result.
.
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
.

type Point,
    x: Float64.
    y: Float64.
.

enum Direction,
    North.
    South.
    East.
    West.
.
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
.
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

---

# Chapter 8 — Functions

## 8.1 Function Definition

Functions are defined with the `function` keyword:

```lp
function greet(name),
    print "Hello " with name.
.
```

With return type annotation:

```lp
function add(a: Int, b: Int) -> Int,
    return a + b.
.
```

With default parameters:

```lp
function greet(name, greeting = "Hello"),
    print greeting with " " with name.
.
```

With variadic parameters:

```lp
function log(level, ...messages),
    for msg in messages,
        print "[" with level with "] " with msg.
    .
.
```

## 8.2 Function Signature

```
function_declaration ::= "function" identifier "(" parameter_list? ")"
                         ( "->" type )? block

parameter_list       ::= parameter ( "," parameter )* ( "," "..." identifier )?

parameter            ::= identifier ( ":" type )? ( "=" expression )?
```

## 8.3 Function Calls

```lp
greet("Naga").
result = add(3, 4).
greet(name = "Naga", greeting = "Hi").
log("INFO", "started", "ready").
```

Named arguments MAY be provided in any order after positional arguments:

```lp
create_user(name = "Naga", age = 25).
create_user(age = 25, name = "Naga").    @ Also valid
```

## 8.4 Return Values

Explicit return:

```lp
function square(n: Int) -> Int,
    return n * n.
.
```

Implicit return (last expression):

```lp
function square(n: Int) -> Int,
    n * n.    @ v0.2 — implicit return of last expression
.
```

In v0.1, `return` is required for non-void functions.

Multiple return values:

```lp
function divmod(a: Int, b: Int) -> (Int, Int),
    return a // b, a % b.
.

quotient, remainder = divmod(10, 3).
```

## 8.5 Function Overloading

Functions MAY be overloaded by parameter types:

```lp
function process(data: String),
    @ handle string
.

function process(data: List<Int>),
    @ handle list
.
```

The compiler selects the best match at compile time. Ambiguous calls are compile errors.

## 8.6 Generic Functions

```lp
function first<T>(items: List<T>) -> T?,
    if items.is_empty(),
        return null.
    .
    return items[0].
.
```

## 8.7 Closures

Functions capture their lexical environment:

```lp
function multiplier(factor),
    return function(n),
        return n * factor.
    .
.

times3 = multiplier(3).
print times3(10).    @ 30
```

Captured variables are shared between closures from the same scope (reference capture for mutable variables).

## 8.8 Higher-Order Functions

Functions are first-class values:

```lp
function apply(fn, value),
    return fn(value).
.

result = apply((x) => x * 2, 21).
```

## 8.9 Methods

Functions defined inside a `type` are methods:

```lp
type Rectangle,
    width: Float64.
    height: Float64.

    function area() -> Float64,
        return self.width * self.height.
    .
.

rect = Rectangle(width = 10, height = 5).
print rect.area().
```

`self` refers to the current instance. `Self` refers to the containing type.

## 8.10 Static Methods

```lp
type MathUtils,
    static function clamp(value, min, max) -> Int,
        if value < min, return min.
        if value > max, return max.
        return value.
    .
.

result = MathUtils.clamp(15, 0, 10).
```

## 8.11 Constructors

Constructors use the type name as a callable:

```lp
type User,
    name: String.
    age: Int.

    function init(name, age = 0),
        self.name = name.
        self.age = age.
    .
.

user = User(name = "Naga", age = 25).
@ Or positional:
user = User("Naga", 25).
```

Named-field construction syntax:

```lp
user = User(),
    name = "Naga".
    age = 25.
.
```

## 8.12 Properties

Computed properties use the `property` keyword:

```lp
type Circle,
    radius: Float64.

    property diameter -> Float64,
        return self.radius * 2.
    .

    property diameter -> Float64 = value,
        self.radius = value / 2.
    .
.
```

## 8.13 Operator Overloading

Operators are overloaded via special method names:

```lp
type Vector,
    x: Float64.
    y: Float64.

    function __add__(other: Vector) -> Vector,
        return Vector(x = self.x + other.x, y = self.y + other.y).
    .
.

v3 = v1 + v2.
```

Supported operator methods:

| Operator | Method |
|----------|--------|
| `+` | `__add__` |
| `-` | `__sub__` |
| `*` | `__mul__` |
| `/` | `__div__` |
| `==` | `__eq__` |
| `<` | `__lt__` |
| `with` | `__compose__` |
| `[]` | `__getitem__`, `__setitem__` |

## 8.14 Extensions

Add methods to existing types without modifying their definition:

```lp
extension String,
    function reversed() -> String,
        return self.chars().reverse().collect().
    .
.

print "hello".reversed().    @ "olleh"
```

Extensions MUST be defined in the same module or an explicitly imported extension module.

## 8.15 Recursion

Recursion is fully supported. The compiler MAY apply tail-call optimization when the recursive call is in tail position:

```lp
function factorial(n: Int) -> Int,
    return factorial_helper(n, 1).
.

function factorial_helper(n: Int, acc: Int) -> Int,
    if n <= 1,
        return acc.
    .
    return factorial_helper(n - 1, acc * n).    @ TCO eligible
.
```

## 8.16 Function Attributes

Metadata annotations (v0.2):

```lp
@deprecated("Use fetch_v2 instead")
function fetch(url),
    @ ...
.
```

In v0.1, use comments for deprecation notices.

## 8.17 Inline Functions

Performance hint for small functions:

```lp
inline function square(n: Int) -> Int,
    return n * n.
.
```

The compiler MAY inline the function body at call sites. `inline` is a hint, not a guarantee.

## 8.18 Main Function

The program entry point:

```lp
function main(),
    print "Hello, Lang.P!".
.
```

`main` MUST take no parameters and return `Int` (exit code) or `Void`. If it returns `Int`, that value is the process exit code.

```lp
function main() -> Int,
    return 0.
.
```

---

# Chapter 9 — Control Flow

## 9.1 Conditional Statements

### 9.1.1 If

```lp
if marks >= 90,
    print "Grade A".
..
```

### 9.1.2 Otherwise If (Else-If)

Lang.P uses `otherwise if` instead of `else if`:

```lp
if marks >= 90,
    print "Grade A".
otherwise if marks >= 80,
    print "Grade B".
otherwise if marks >= 70,
    print "Grade C".
otherwise,
    print "Grade F".
..
```

`otherwise if` is a two-token keyword. It MUST NOT be written as `else if`.

### 9.1.3 Otherwise (Else)

```lp
if online,
    print "Connected".
otherwise,
    print "Offline".
..
```

### 9.1.4 If Expression

Inline conditional:

```lp
label = if score >= 60, "Pass", otherwise, "Fail".
```

## 9.2 Loops

### 9.2.1 Repeat (Counted Loop)

```lp
repeat 5 times,
    print "Hello".
..
```

The counter variable is optionally accessible:

```lp
repeat 5 times as i,
    print "Iteration " with i.
..
```

`i` ranges from `0` to `4` (zero-indexed).

### 9.2.2 Repeat Forever

```lp
repeat forever,
    process_events().
    if should_stop,
        break.
    .
..
```

Equivalent to `while true` but reads more naturally for event loops.

### 9.2.3 For-In Loop

```lp
for student in students,
    print student.name.
.

for i in 0..10,
    print i.
.

for key, value in dictionary,
    print key with ": " with value.
.

for item in list,
    print item.
.
```

For-in supports:

- Lists, arrays, sets, dictionaries (keys), strings (characters), ranges
- Custom iterables implementing the `Iterable<T>` interface

### 9.2.4 While Loop

```lp
while count > 0,
    print count.
    count -= 1.
..
```

The condition is evaluated before each iteration. If false initially, the body never executes.

### 9.2.5 Loop Control

```lp
break.       @ Exit innermost loop
continue.    @ Skip to next iteration
```

## 9.3 Pattern Matching

Lang.P supports `match` for exhaustive pattern matching (v0.2 full, v0.1 basic):

```lp
match status,
    Status.Active => print "Active".
    Status.Inactive => print "Inactive".
    Status.Pending(reason) => print "Pending: " with reason.
.
```

Basic form in v0.1 uses if-chains; full `match` is planned for v0.2.

## 9.4 Iteration Protocol

Custom types implement iteration:

```lp
interface Iterable<T>,
    function iterator() -> Iterator<T>.
.

interface Iterator<T>,
    function has_next() -> Bool.
    function next() -> T.
.
```

Example:

```lp
type Counter,
    max: Int.
    current: Int = 0.

    function iterator() -> CounterIterator,
        return CounterIterator(counter = self).
    .
.

@ for n in Counter(max = 5) iterates 0..4
```

## 9.4 Guard Clauses

Early exit pattern (convention, not syntax):

```lp
function process(user),
    if user == null,
        return.
    .
    if not user.is_active,
        return.
    .
    @ main logic here
    do_work(user).
.
```

## 9.5 Nested Control Flow

```lp
for row in matrix,
    for cell in row,
        if cell > 0,
            print cell.
        .
    .
..
```

Indentation MUST increase by 4 spaces per nesting level.

## 9.6 Control Flow with Blocks

Every control flow construct that takes a body uses `,` / `..`:

```lp
if condition,
    @ body
..

repeat n times,
    @ body
..

for item in items,
    @ body
..

while condition,
    @ body
..
```

The `,` MUST appear at the end of the header line. The `..` MUST align with the header's indentation level.

## 9.7 Boolean Conditions

Conditions MUST evaluate to `Bool`. The compiler MUST NOT allow implicit truthiness:

```lp
@ Error in Lang.P — no implicit truthiness
if name,
    print name.
.

@ Correct
if name != null and name != "",
    print name.
.
```

This prevents common beginner bugs found in Python and JavaScript.

## 9.8 Switch on Values (v0.2)

Future syntax for simple value dispatch:

```lp
switch day,
    "Monday" => print "Start of week".
    "Friday" => print "Almost weekend".
    otherwise => print "Midweek".
..
```

In v0.1, use if/otherwise if chains or match (when available).

---

# Chapter 10 — Object Model

## 10.1 Type Definition

Lang.P uses `type` instead of `class`:

```lp
type User,
    name: String.
    age: Int.
    email: String? = null.
.
```

Fields are declared with optional type annotations and default values. Fields without defaults MUST be set in the constructor.

## 10.2 Object Creation

### 10.2.1 Positional Construction

```lp
user = User("Naga", 25).
```

### 10.2.2 Named Construction

```lp
user = User(),
    name = "Naga".
    age = 25.
    email = "naga@example.com".
.
```

### 10.2.3 Default Construction

```lp
user = User().
@ Fields with defaults are initialized; others must be set in init()
```

## 10.3 Field Access

```lp
user.name = "Naga".
print user.age.
user.email = "naga@example.com".
```

Fields are accessed via dot notation. Private fields are accessible only within the type and its nested types.

## 10.4 Constructors

The `init` method is the constructor:

```lp
type User,
    name: String.
    age: Int.

    function init(name, age = 0),
        self.name = name.
        self.age = age.
    .
.
```

Multiple constructors via overloading:

```lp
function init(name),
    self.init(name, 0).
.
```

## 10.5 Inheritance

Single inheritance using `extends`:

```lp
type Animal,
    name: String.

    function speak(),
        print "...".
    .
.

type Dog extends Animal,
    breed: String.

    function speak(),
        print "Woof!".
    .
.

dog = Dog(name = "Buddy", breed = "Labrador").
dog.speak().    @ "Woof!"
```

Rules:

- A type MAY extend at most one other type (single inheritance).
- All types implicitly extend `Object` if no parent is specified.
- `super` refers to the parent type's methods and constructor.

```lp
type Dog extends Animal,
    function init(name, breed),
        super.init(name).
        self.breed = breed.
    .
.
```

## 10.6 Interfaces

Interface definition:

```lp
interface Drawable,
    function draw() -> Void.
    function bounds() -> (Int, Int, Int, Int).
.

interface Serializable,
    function serialize() -> String.
    function deserialize(data: String) -> Void.
.
```

Implementation is implicit (structural):

```lp
type Rectangle,
    x: Int.
    y: Int.
    width: Int.
    height: Int.

    function draw(),
        @ render rectangle
    .

    function bounds(),
        return (self.x, self.y, self.width, self.height).
    .
.

@ Rectangle satisfies Drawable structurally — no explicit 'implements' needed
```

Explicit implementation for clarity in public APIs:

```lp
type Rectangle implements Drawable, Serializable,
    @ ...
.
```

## 10.7 Polymorphism

```lp
function render(item: Drawable),
    item.draw().
.

rect = Rectangle(x = 0, y = 0, width = 100, height = 50).
render(rect).
```

Subtype polymorphism: a `Dog` is an `Animal`. Interface polymorphism: any type with matching methods satisfies the interface.

## 10.8 Encapsulation

Visibility modifiers on type members:

```lp
type BankAccount,
    public owner: String.
    private balance: Float64 = 0.0.

    public function deposit(amount: Float64),
        if amount > 0,
            self.balance += amount.
        .
    .

    public function get_balance() -> Float64,
        return self.balance.
    .
.
```

| Modifier | Accessible from |
|----------|-----------------|
| `public` | Anywhere |
| `internal` | Same package |
| `private` | Same type (and nested types) |

## 10.9 Static Members

```lp
type Config,
    static instance: Config? = null.

    static function get() -> Config,
        if self.instance == null,
            self.instance = Config().
        .
        return self.instance.
    .
.
```

Static fields and methods belong to the type, not instances. Access via `TypeName.member`.

## 10.10 Properties

Computed fields with getter and/or setter:

```lp
type Temperature,
    _celsius: Float64 = 0.0.

    property celsius -> Float64,
        return self._celsius.
    .

    property celsius -> Float64 = value,
        self._celsius = value.
    .

    property fahrenheit -> Float64,
        return self._celsius * 9.0 / 5.0 + 32.0.
    .

    property fahrenheit -> Float64 = value,
        self._celsius = (value - 32.0) * 5.0 / 9.0.
    .
.
```

## 10.11 Abstract Types

Cannot be instantiated directly; must be extended:

```lp
abstract type Shape,
    abstract function area() -> Float64.
    abstract function perimeter() -> Float64.
.

type Circle extends Shape,
    radius: Float64.

    function area() -> Float64,
        return 3.14159 * self.radius ** 2.
    .

    function perimeter() -> Float64,
        return 2.0 * 3.14159 * self.radius.
    .
.
```

## 10.12 Generics

```lp
type Box<T>,
    value: T.

    function get() -> T,
        return self.value.
    .

    function set(value: T),
        self.value = value.
    .
.

int_box = Box<Int>(value = 42).
str_box = Box<String>(value = "hello").
```

Generic constraints:

```lp
type SortedList<T: Comparable>,
    items: List<T> = [].

    function add(item: T),
        self.items.add(item).
        self.items.sort().
    .
.
```

## 10.13 Operator Overloading

See [Chapter 8 §8.13](08-functions.md#813-operator-overloading).

## 10.14 Extensions

See [Chapter 8 §8.14](08-functions.md#814-extensions).

## 10.15 Object Identity

Every object has a unique identity distinct from its value:

```lp
a = User(name = "Naga").
b = User(name = "Naga").
a is b.     @ false — different objects
a == b.     @ depends on __eq__ implementation
```

Default equality is identity-based. Types SHOULD override `__eq__` for value equality.

## 10.16 Destructors

Cleanup method (called by GC before collection):

```lp
type FileHandle,
    path: String.
    _handle: NativeHandle.

    function destroy(),
        close_native(self._handle).
    .
.
```

Deterministic cleanup uses `try`/`finally` or the `using` statement (v0.2).

## 10.17 Enums as Algebraic Data Types

```lp
enum Result<T, E>,
    Ok(value: T).
    Err(error: E).
.

enum Option<T>,
    Some(value: T).
    None.
.

result = Result.Ok(42).
if result is Result.Ok,
    print result.value.
.
```

## 10.18 The Root Object Type

All types inherit from `Object`:

```lp
@ Built-in
type Object,
    function to_string() -> String.
    function hash() -> Int.
    function equals(other: Object) -> Bool.
.
```

Every value can be converted to `Object`.

---

# Chapter 11 — Modules & Imports

## 11.1 Module System Overview

Lang.P organizes code into **modules** (files) and **packages** (projects). The module system is designed for readability: every dependency is explicitly declared.

## 11.2 Import Syntax

```lp
use module_name.
```

Examples:

```lp
use navigator.
use ai.
use network.
use database.
use filesystem.
use json.
use math.
```

Qualified imports access nested modules:

```lp
use network.http.
use stdlib.collections.list.
```

## 11.3 Import Rules

1. Imports MUST appear at the top of a file, before other declarations (except module declarations).
2. Wildcard imports (`use module.*`) are **forbidden**.
3. Duplicate imports of the same module are a warning.
4. Unused imports are a warning (configurable in linter).

```lp
@ Valid
use json.
use network.

@ Invalid — wildcard
use json.*.

@ Invalid — import after code
x = 10.
use json.
```

## 11.4 Name Resolution

Imported module names are used as prefixes:

```lp
use json.

data = json.parse('{"name": "Naga"}').
text = json.stringify(data).
```

If a name conflicts with a local binding, the local binding takes precedence:

```lp
use json.

function json(),    @ Local function shadows module
    return "custom".
.

print json().    @ Calls local function, not module
```

To disambiguate, use the full module path from the package root.

## 11.5 Module Structure

A standard package layout:

```
my-project/
    lang.toml           @ Package manifest
    main.lp             @ Entry point
    src/
        models/
            user.lp     @ Module: models.user
            post.lp     @ Module: models.post
        services/
            api.lp      @ Module: services.api
    tests/
        test_user.lp
```

## 11.6 Module Declaration

Optional explicit module name:

```lp
module models.user.
```

If omitted, the module name is derived from the file path (see [Chapter 3](03-program-structure.md)).

## 11.7 Re-exports

A module MAY re-export symbols from its dependencies:

```lp
@ In models/__init__.lp (future barrel module support)
use models.user.
use models.post.

@ Re-export (v0.2)
export models.user.User.
export models.post.Post.
```

In v0.1, consumers import submodules directly.

## 11.8 Standard Library Modules

Core stdlib modules are available without installation:

| Module | Description |
|--------|-------------|
| `core` | Built-in types, print, assert |
| `math` | Mathematical functions |
| `json` | JSON parsing and serialization |
| `collections` | List, Dictionary, Set utilities |
| `datetime` | Date and time |
| `filesystem` | File I/O |
| `network` | HTTP, TCP, WebSocket |
| `database` | Database connectivity |
| `crypto` | Cryptographic functions |
| `compression` | Compression algorithms |
| `terminal` | Terminal UI and colors |
| `testing` | Test framework |
| `env` | Environment variables |
| `reflect` | Runtime type reflection |
| `async` | Async runtime primitives |

Framework modules require explicit import:

| Module | Description |
|--------|-------------|
| `navigator` | Browser/desktop UI framework |
| `ai` | AI/LLM framework |

## 11.9 Third-Party Packages

Installed via the `lang` package manager:

```lp
use requests.      @ From lang.toml dependency
use my_lib.utils.
```

See [Chapter 20](20-package-system.md).

## 11.10 Circular Dependencies

Circular module dependencies are **forbidden**. The compiler MUST detect and reject cycles:

```
error[E0501]: circular dependency detected
  models.user → services.api → models.user
```

Resolution strategies:

- Extract shared types into a common module
- Use interfaces to invert dependencies
- Restructure package layout

## 11.11 Conditional Imports

Not supported in v0.1. All imports are static and resolved at compile time.

## 11.12 Module Initialization

Module-level code executes on first import, in dependency order:

```lp
@ In database.lp
print "Initializing database module".
CONNECTION = create_pool().

@ In main.lp
use database.    @ Prints "Initializing database module"
```

Module initialization is thread-safe and occurs exactly once.

---

# Chapter 12 — Events

## 12.1 Overview

Lang.P provides first-class event-driven programming with the `on` keyword. Events enable reactive, declarative code for UI, network, lifecycle, and custom signals.

## 12.2 Event Handler Syntax

```lp
on event_source,
    @ handler body
..
```

Examples:

```lp
on button.clicked,
    print "Button clicked".
.

on Browser.Start,
    print "Browser started".
.

on server.request,
    response = handle(server.request).
    server.respond(response).
.
```

## 12.3 Event Sources

An event source is any expression that produces events:

| Source | Event | Description |
|--------|-------|-------------|
| `button.clicked` | Click | UI button click |
| `Browser.Start` | Lifecycle | Browser initialization |
| `Browser.Close` | Lifecycle | Browser shutdown |
| `server.request` | HTTP | Incoming HTTP request |
| `user.message` | AI | User message in AI chat |
| `timer.elapsed` | Timer | Timer fired |
| `window.resized` | UI | Window size changed |

Event names use dot notation: `object.event` or `Type.Event`.

## 12.4 Event Payload

Events MAY carry data accessible within the handler:

```lp
on server.request,
    path = server.request.path.
    method = server.request.method.
    body = server.request.body.
    print method with " " with path.
.
```

The event source expression provides the context object. In the handler scope, the event source is bound to its current value.

## 12.5 Multiple Handlers

Multiple handlers MAY be registered for the same event:

```lp
on button.clicked,
    print "Handler 1".
.

on button.clicked,
    print "Handler 2".
.
```

Handlers execute in registration order unless priority is specified (v0.2).

## 12.6 Custom Events

Types MAY define custom events:

```lp
type DownloadManager,
    event completed.
    event failed.
    event progress.

    function finish(),
        self.completed.emit(file = self.current_file).
    .
.

on download.completed,
    print "Downloaded: " with download.completed.file.
.
```

### 12.6.1 Event Declaration

```lp
type MyWidget,
    event clicked(x: Int, y: Int).
    event value_changed(old: Int, new: Int).
.
```

### 12.6.2 Event Emission

```lp
self.clicked.emit(x = 10, y = 20).
```

## 12.7 Event Lifecycle

```
Register handler (on) → Event occurs → Handlers invoked in order → Return
```

Handlers run synchronously by default. Async handlers use `wait for`:

```lp
on button.clicked,
    data = wait for fetch(url).
    update_ui(data).
.
```

## 12.8 Removing Handlers

Handlers are automatically removed when their scope is destroyed. Explicit removal (v0.2):

```lp
handler = on button.clicked,
    print "Clicked".
.

@ Later:
handler.disconnect().
```

In v0.1, use object lifecycle for automatic cleanup.

## 12.9 Event Propagation

UI events support bubbling (v0.2):

```lp
on parent.clicked,
    print "Parent clicked".
.

on child.clicked,
    print "Child clicked".
    @ Event bubbles to parent by default
.
```

Use `event.stop()` to prevent propagation (v0.2).

## 12.10 Navigator Events

Browser and UI events from the Navigator framework:

```lp
on browser.tab_changed,
    print "Active tab: " with browser.tab_changed.url.
.

on browser.navigation,
    print "Navigating to: " with browser.navigation.url.
.

on browser.download_started,
    print "Downloading: " with browser.download_started.filename.
.
```

See [Chapter 17](17-navigator.md) for the complete event catalog.

## 12.11 AI Events

```lp
on user.message,
    reply = assistant.chat(user.message).
    print reply.
.

on assistant.response,
    display(assistant.response.text).
.

on assistant.error,
    print "AI Error: " with assistant.error.message.
.
```

See [Chapter 18](18-ai-framework.md).

## 12.12 Error Handling in Event Handlers

Uncaught errors in event handlers are reported to the runtime error handler. By default, they log and continue (non-fatal). Critical handlers SHOULD use try/catch:

```lp
on server.request,
    try,
        response = handle(server.request).
        server.respond(response).
    catch error,
        server.respond(error_response(500, error.message)).
    ..
.
```

## 12.13 Event Loop Integration

Applications with event handlers enter an event loop automatically:

```lp
@ main.lp
browser = Browser(name = "Nova").

on Browser.Start,
    print "Welcome".
.

@ Event loop runs until browser is closed
```

Explicit event loop control:

```lp
EventLoop.run().          @ Block until all events processed
EventLoop.run_async().    @ Non-blocking
EventLoop.stop().         @ Signal shutdown
```

---

# Chapter 13 — Error Handling

## 13.1 Overview

Lang.P uses structured exception handling with `try`, `catch`, and `finally` blocks. Error handling reads as natural instructions.

## 13.2 Try-Catch-Finally

```lp
try,
    risky_function().
catch error,
    print error.
finally,
    cleanup().
..
```

### 13.2.1 Try Block

The `try` block contains code that may throw an error:

```lp
try,
    data = parse_json(raw_text).
    process(data).
..
```

### 13.2.2 Catch Block

The `catch` block receives the error object:

```lp
catch error,
    print "Error: " with error.message.
..
```

Typed catch (specific error types):

```lp
catch error: NetworkError,
    print "Network failed: " with error.message.
catch error: ParseError,
    print "Invalid data: " with error.message.
catch error,
    print "Unknown error: " with error.message.
..
```

Multiple catch blocks are evaluated top-to-bottom; the first matching type handles the error.

### 13.2.3 Finally Block

The `finally` block always executes, regardless of success or failure:

```lp
try,
    file = open("data.txt").
    process(file).
catch error,
    print error.
finally,
    if file != null,
        file.close().
    .
..
```

## 13.3 Throwing Errors

Raise errors with `throw`:

```lp
function divide(a, b),
    if b == 0,
        throw DivisionError("Cannot divide by zero").
    .
    return a / b.
.
```

## 13.4 Error Types

Built-in error hierarchy:

```
Error (base)
├── RuntimeError
│   ├── DivisionError
│   ├── IndexError
│   ├── InputError
│   ├── InputCancelledError
│   ├── InputValidationError
│   ├── KeyError
│   ├── NullError
│   └── TypeError
├── IOError
│   ├── FileNotFoundError
│   ├── PermissionError
│   └── NetworkError
├── ParseError
│   ├── JsonError
│   └── SyntaxError
└── CompileError    @ Compile-time only
```

Custom errors:

```lp
type ValidationError extends Error,
    field: String.

    function init(field, message),
        super.init(message).
        self.field = field.
    .
.

throw ValidationError(field = "email", message = "Invalid email format").
```

## 13.5 Error Properties

All errors implement:

```lp
interface Error,
    property message -> String.
    property cause -> Error?.
    property stack_trace -> StackTrace.
.
```

Access:

```lp
catch error,
    print error.message.
    print error.stack_trace.
..
```

## 13.6 Result Type (Alternative Pattern)

For recoverable errors without exceptions:

```lp
enum Result<T, E>,
    Ok(value: T).
    Err(error: E).
.

function parse_number(text: String) -> Result<Float64, ParseError>,
    try,
        return Result.Ok(parse_float(text)).
    catch error: ParseError,
        return Result.Err(error).
    ..
.
```

Both patterns coexist. Guidelines:

- Use **exceptions** for unexpected failures and boundary crossings
- Use **Result** for expected failure cases in public APIs

## 13.7 Panics

Unrecoverable errors cause a **panic**:

```lp
panic("Invariant violated").
```

Panics terminate the current task and unwind the stack, running `finally` blocks. Uncaught panics terminate the program with a non-zero exit code.

The default panic handler prints the error and stack trace to stderr.

## 13.8 Assertions

```lp
assert condition.
assert x >= 0, "x must be non-negative".
```

Failed assertions panic in debug mode. See [Chapter 7 §7.15](07-statements.md#715-assertion-statement).

## 13.9 Error Propagation

Automatic propagation with `?` operator (v0.2):

```lp
function load_config() -> Config,
    text = read_file("config.json")?.    @ Propagates error if read fails
    return json.parse(text)?.            @ Propagates error if parse fails
.
```

In v0.1, use explicit try/catch or Result types.

## 13.10 Compile-Time Errors

The compiler reports errors with structured format:

```
error[E0301]: type mismatch
  --> src/main.lp:5:5
   |
 5 |     result: String = 42.
   |                      ^^ expected String, found Int
   |
  = help: use to_string(42) to convert

error: compilation failed with 1 error
```

Error codes follow the pattern `E` + 4 digits, categorized by:

| Range | Category |
|-------|----------|
| E01xx | Lexical errors |
| E02xx | Syntax errors |
| E03xx | Type errors |
| E04xx | Name resolution errors |
| E05xx | Module/import errors |
| E06xx | Borrow/ownership errors (future) |

Warnings follow the pattern `W` + 4 digits:

| Range | Category |
|-------|----------|
| W01xx | Input and inference warnings |
| W02xx | Style and lint warnings |

Example:

```
warning[W0101]: input type could be more specific
  --> main.lp:3:7
   |
 3 | age = input "Age : ".
   |       ^^^^^^^^^^^^^^ the value "age" appears to be used as a number
   |
  = help: consider using: age = input number "Age : ".
```

## 13.11 Defer Statement (v0.2)

Alternative to finally for scoped cleanup:

```lp
function process_file(path),
    file = open(path).
    defer file.close().
    @ file.close() runs when function exits, regardless of how
    process(file).
.
```

In v0.1, use try/finally.

## 13.12 Error Handling Best Practices

1. Catch specific error types before general ones.
2. Always clean up resources in `finally`.
3. Never silently swallow errors — log or re-throw.
4. Include context in error messages: `"Failed to load config from " with path`.
5. Use Result for API boundaries where failure is expected.

---

# Chapter 14 — Concurrency & Async

## 14.1 Overview

Lang.P provides async/await for non-blocking operations and primitives for concurrent execution. The primary async syntax uses `wait for`.

## 14.2 Async Functions

Functions that perform async operations are marked with `async`:

```lp
async function fetch_data(url),
    response = wait for get url.
    return response.body.
.
```

Calling an async function without `wait for` returns a `Task<T>`:

```lp
task = fetch_data("https://api.example.com").    @ Returns Task<String>
result = wait for task.                          @ Blocks until complete
```

## 14.3 Wait For

`wait for` suspends the current task until an async operation completes:

```lp
response = wait for get url.
data = wait for fetch_data(url).
results = wait for all(task1, task2, task3).
```

`wait for` MUST only appear inside `async` functions or event handlers.

## 14.4 Async HTTP

```lp
use network.

async function fetch_user(id),
    response = wait for get ("https://api.example.com/users/" with id).
    return json.parse(response.body).
.

user = wait for fetch_user(42).
```

## 14.5 Concurrent Tasks

### 14.5.1 Spawning Tasks

```lp
task = spawn fetch_data(url).
result = wait for task.
```

### 14.5.2 Waiting for Multiple

```lp
@ Wait for all
results = wait for all(
    fetch_data(url1),
    fetch_data(url2),
    fetch_data(url3)
).

@ Wait for first
result = wait for any(task1, task2).
```

### 14.5.3 Task Handles

```lp
task = spawn long_running_work().
task.cancel().
status = task.status().    @ Running, Completed, Cancelled, Failed
```

## 14.6 Async Event Handlers

Event handlers can await without blocking the event loop:

```lp
on button.clicked,
    data = wait for fetch(url).
    update_display(data).
.
```

## 14.7 Synchronization Primitives

### 14.7.1 Mutex

```lp
lock = Mutex().
lock.acquire().
try,
    shared_data += 1.
finally,
    lock.release().
.
```

Scoped lock (v0.2):

```lp
with lock,
    shared_data += 1.
.
```

### 14.7.2 Channel

```lp
channel = Channel<Int>(capacity = 100).

@ Producer
spawn,
    for i in 0..100,
        channel.send(i).
    .
    channel.close().
.

@ Consumer
while true,
    value = wait for channel.receive().
    if value == null, break.
    process(value).
..
```

### 14.7.3 Atomic Types

```lp
counter = AtomicInt(0).
counter.fetch_add(1).
value = counter.load().
```

## 14.8 Thread Safety

- `Send` trait: types safe to transfer between threads
- `Sync` trait: types safe to share between threads via reference
- The compiler MUST enforce Send/Sync bounds on spawned tasks

## 14.9 Async Runtime

The async runtime is provided by the `async` module:

```lp
use async.

async.run(main()).    @ Run async main and block until complete
```

Configuration:

```lp
async.configure(
    workers = 4.
    stack_size = 1024 * 1024.
).
```

## 14.10 Sleep and Timers

```lp
wait for sleep(seconds = 1.5).

timer = Timer(interval = 1000).
on timer.elapsed,
    print "Tick".
.
timer.start().
```

## 14.11 Parallel Iteration

```lp
results = parallel for item in items,
    wait for process(item).
.
```

Results maintain input order.

## 14.12 Async Streams

```lp
async function data_stream() -> AsyncStream<Int>,
    for i in 0..100,
        yield i.
        wait for sleep(seconds = 0.1).
    .
.

async for value in data_stream(),
    print value.
.
```

## 14.13 Error Handling in Async Code

Errors in async functions propagate through `wait for`:

```lp
async function fetch_safe(url),
    try,
        return wait for get url.
    catch error: NetworkError,
        print "Network error: " with error.message.
        return null.
    ..
.
```

Uncaught errors in spawned tasks are reported to the async runtime error handler.

## 14.14 Cancellation

Tasks support cooperative cancellation:

```lp
async function long_task(cancel: CancellationToken),
    repeat forever,
        if cancel.is_cancelled(),
            return.
        .
        wait for do_step().
    ..
.
```

## 14.15 Performance Guidelines

1. Use `spawn` for independent parallel work.
2. Prefer `wait for all` over sequential awaits when operations are independent.
3. Avoid blocking operations in async functions — use async I/O.
4. Keep event handlers fast; offload heavy work to spawned tasks.

---

# Chapter 15 — I/O & Network

## 15.1 Filesystem

Lang.P provides natural-language-style file I/O.

### 15.1.1 Reading Files

```lp
use filesystem.

text = read "settings.txt".
bytes = read_bytes "image.png".
lines = read_lines "data.csv".
```

With error handling:

```lp
try,
    text = read "settings.txt".
catch error: FileNotFoundError,
    text = "".
..
```

### 15.1.2 Writing Files

```lp
write text to "backup.txt".
write_bytes data to "output.bin".
append line to "log.txt".
```

### 15.1.3 File Operations

```lp
exists = file_exists("config.json").
size = file_size("data.bin").
delete "temp.txt".
copy "source.txt" to "dest.txt".
move "old.txt" to "new.txt".
rename "old.txt" to "new.txt".
```

### 15.1.4 Directory Operations

```lp
create_dir "logs".
create_dir_all "path/to/nested/dir".
delete_dir "temp".
list_files "src/".
list_dirs ".".
```

### 15.1.5 Path Operations

```lp
use filesystem.path.

full = path.join("src", "main.lp").
parent = path.parent("/src/main.lp").
name = path.filename("/src/main.lp").
ext = path.extension("main.lp").
absolute = path.resolve("../config.json").
```

### 15.1.6 File Watching

```lp
watcher = watch "config.json".
on watcher.changed,
    reload_config().
.
```

## 15.2 Standard I/O

### 15.2.1 Built-in Input Expression

Interactive input is provided by the built-in `input` expression (see [Chapter 6 §6.15](06-expressions.md#615-input-expression)). This is the **preferred** way to read user input:

```lp
name = input "Enter your name : ".
age = input number "Enter your age : ".
password = input password "Enter your password : ".
resume = input file "Choose your resume".
```

Do **not** use function-call syntax — Lang.P has no `input()` function:

```lp
@ Invalid
name = input("Enter name: ").

@ Correct
name = input "Enter name: ".
```

### 15.2.2 Low-Level Stream I/O

For library authors, scripts, and non-interactive pipelines, lower-level stream functions remain available:

```lp
@ Read a line without the input expression (stdlib / IO module)
name = read_line("Enter name: ").
password = read_line_masked("Password: ").

@ Output
print "Hello".
print inline "Loading".
write stderr "Error occurred".
```

These functions are equivalent to `input text` and `input password` respectively but lack type validation, inference warnings, and IDE quick-fix support. Beginner-facing documentation and tutorials MUST use `input` instead.

## 15.3 HTTP Client

Natural-language HTTP syntax:

```lp
use network.

@ GET
response = get "https://google.com".
response = get url.
response = get (url with "?q=" with query).

@ POST
response = post "https://api.example.com" with data.
response = post url with json_body.

@ Other methods
response = put url with data.
response = delete url.
response = patch url with data.
```

### 15.3.1 Request Options

```lp
response = get url,
    headers = {"Authorization": "Bearer " with token}.
    timeout = 30.
    follow_redirects = true.
.
```

### 15.3.2 Response Object

```lp
response.status.        @ 200
response.body.          @ String
response.headers.       @ Dictionary<String, String>
response.json().        @ Parsed JSON
response.ok.            @ true if 200-299
```

### 15.3.3 Async HTTP

```lp
response = wait for get url.
response = wait for post url with data.
```

## 15.4 HTTP Server

```lp
use network.

function handle(request),
    return response(200, body = "Hello").
.

server = Server(port = 8080).

on server.request,
    reply = handle(server.request).
    server.respond(reply).
.

server.start().
print "Listening on port 8080".
```

### 15.4.1 Route Handling

```lp
server = Server(port = 8080).

on server.request where server.request.path == "/",
    server.respond(response(200, body = "Home")).
.

on server.request where server.request.path == "/api/users",
    users = get_users().
    server.respond(json_response(users)).
.
```

## 15.5 WebSocket

```lp
use network.

socket = WebSocket("wss://echo.example.com").

on socket.message,
    print "Received: " with socket.message.data.
.

on socket.connected,
    socket.send("Hello").
.

socket.connect().
```

## 15.6 TCP/UDP

```lp
@ TCP
listener = TcpListener.bind("0.0.0.0:8080").
on listener.connection,
    handle_connection(listener.connection).
.

@ UDP
socket = UdpSocket.bind("0.0.0.0:9000").
data, addr = socket.receive_from().
socket.send_to(response, addr).
```

## 15.7 DNS

```lp
use network.dns.

addresses = resolve("example.com").
```

## 15.8 JSON

```lp
use json.

data = json.parse('{"name": "Naga", "age": 25}').
text = json.stringify(data).
pretty = json.stringify(data, indent = 2).

@ Typed parsing
user = json.parse_as('{"name": "Naga"}', User).
```

## 15.9 Serialization

Generic serialization beyond JSON:

```lp
use serialization.

bytes = serialize(value).
value = deserialize(bytes, Type).
```

## 15.10 Streaming I/O

```lp
stream = open_stream("large_file.dat").
while true,
    chunk = stream.read(4096).
    if chunk.is_empty(), break.
    process(chunk).
.
stream.close().
```

Async streaming:

```lp
async for chunk in async_read_stream("large_file.dat"),
    wait for process(chunk).
.
```

## 15.11 Compression

```lp
use compression.

compressed = gzip.compress(data).
original = gzip.decompress(compressed).
```

## 15.12 SSL/TLS

TLS is enabled by default for HTTPS. Custom certificates:

```lp
response = get url,
    tls = TlsConfig(
        verify = true.
        ca_cert = read "ca.pem".
    ).
.
```

## 15.13 URL Handling

```lp
use network.url.

parsed = url.parse("https://example.com/path?q=1").
print parsed.host.
print parsed.path.
print parsed.query["q"].

built = url.build(
    scheme = "https".
    host = "example.com".
    path = "/api".
    query = {"page": "1"}.
).
```

---

# Chapter 16 — Standard Library

## 16.1 Overview

The Lang.P standard library provides production-quality modules for common tasks. All stdlib modules follow the language philosophy: APIs read like natural instructions.

## 16.2 Core (`core`)

Built-in types and functions available without import:

| Function | Description |
|----------|-------------|
| `print(value...)` | Print to stdout |
| `assert(condition, message?)` | Debug assertion |
| `panic(message)` | Unrecoverable error |
| `to_string(value)` | Convert to string |
| `parse_int(text)` | Parse integer |
| `parse_float(text)` | Parse float |
| `len(collection)` | Length of collection |
| `type_of(value)` | Runtime type name |
| `exit(code?)` | Exit program |

Built-in types: `Int`, `Float64`, `Bool`, `Char`, `String`, `List`, `Dictionary`, `Set`, `Tuple`, `Object`, `Null`.

Built-in expressions (no import required):

| Expression | Description |
|------------|-------------|
| `input "prompt"` | Read user input (type inferred or `String`) |
| `input text "prompt"` | Read text line from stdin |
| `input number "prompt"` | Read validated integer |
| `input decimal "prompt"` | Read validated decimal |
| `input boolean "prompt"` | Read yes/no confirmation |
| `input password "prompt"` | Read masked text |
| `input file "prompt"` | Native file picker |
| `input folder "prompt"` | Native folder picker |
| `input date "prompt"` | Native date picker |
| `input color "prompt"` | Native color picker |

See [Chapter 6 §6.15](06-expressions.md#615-input-expression) and [Chapter 4 §4.14](04-types.md#414-input-expression-types).

## 16.3 Collections (`collections`)

```lp
use collections.

@ List operations
list = list.of(1, 2, 3).
list.map((x) => x * 2).
list.filter((x) => x > 0).
list.reduce(0, (acc, x) => acc + x).
list.sort().
list.reverse().
list.unique().

@ Dictionary operations
dict = dict.of("a", 1, "b", 2).
dict.keys().
dict.values().
dict.entries().
dict.merge(other).

@ Set operations
set = set.of(1, 2, 3).
set.union(other).
set.intersection(other).
set.difference(other).
```

## 16.4 Math (`math`)

```lp
use math.

math.abs(-5).
math.sqrt(16).
math.pow(2, 10).
math.floor(3.7).
math.ceil(3.2).
math.round(3.5).
math.min(a, b).
math.max(a, b).
math.sin(angle).
math.cos(angle).
math.log(value).
math.random().
math.random_int(1, 100).
```

Constants: `math.PI`, `math.E`.

## 16.5 DateTime (`datetime`)

```lp
use datetime.

now = datetime.now().
today = datetime.today().
parsed = datetime.parse("2026-07-14", format = "%Y-%m-%d").
formatted = now.format("%Y-%m-%d %H:%M:%S").
duration = datetime.Duration(hours = 2, minutes = 30).
future = now + duration.
```

## 16.6 Crypto (`crypto`)

```lp
use crypto.

hash = crypto.sha256(data).
hash = crypto.md5(text).
hmac = crypto.hmac_sha256(key, data).
random_bytes = crypto.random_bytes(32).
uuid = crypto.uuid4().
```

## 16.7 Terminal (`terminal`)

The `terminal` module provides advanced terminal formatting and display — colored output, tables, progress bars, and screen control. It does **not** replace the built-in `input` expression for reading user input.

```lp
use terminal.

@ For basic input, use the built-in input expression instead:
@ name = input "Enter your name : ".

@ Terminal module — formatting and display
terminal.print_colored("Error", color = red).
terminal.clear().
terminal.set_title("My App").
terminal.progress_bar(current, total).
table = terminal.Table().
table.add_row("Name", "Age").
table.add_row("Naga", "25").
table.render().
```

For masked password input at the stdlib level, prefer `input password "..."` over `read_line_masked`. The `terminal` module MAY offer `terminal.prompt()` for styled prompts in v0.2, but `input` remains the canonical beginner API.

## 16.8 Testing (`testing`)

```lp
use testing.

test "addition works",
    assert add(2, 3) == 5.
.

test "division by zero throws",
    assert_throws(DivisionError, function(),
        divide(1, 0).
    ).
.

@ Run with: lang test
testing.run().
```

Test functions:

| Function | Description |
|----------|-------------|
| `test(name, body)` | Define a test case |
| `assert_eq(a, b)` | Assert equality |
| `assert_ne(a, b)` | Assert inequality |
| `assert_throws(type, body)` | Assert exception |
| `assert_true(condition)` | Assert true |
| `assert_false(condition)` | Assert false |

## 16.9 Database (`database`)

```lp
use database.

db = database.connect("postgresql://localhost/mydb").

users = db.query("SELECT * FROM users WHERE age > ?", 18).
db.execute("INSERT INTO users (name) VALUES (?)", "Naga").

@ ORM-style
users = db.table("users").where("age", ">", 18).all().
db.table("users").insert(name = "Naga", age = 25).
```

Supported backends (via drivers):

- PostgreSQL
- SQLite
- MySQL
- MongoDB (document)

## 16.10 Graphics (`graphics`)

2D graphics primitives (v0.2):

```lp
use graphics.

canvas = Canvas(width = 800, height = 600).
canvas.draw_rect(x = 10, y = 10, width = 100, height = 50, color = blue).
canvas.draw_circle(x = 200, y = 200, radius = 50, color = red).
canvas.draw_text("Hello", x = 300, y = 300, font = "Arial", size = 24).
canvas.save("output.png").
```

## 16.11 Audio (`audio`)

```lp
use audio.

sound = audio.load("notification.wav").
sound.play().
audio.record(duration = 5.0, output = "recording.wav").
```

## 16.12 Video (`video`)

```lp
use video.

clip = video.load("intro.mp4").
clip.play().
video.encode(frames, output = "output.mp4", fps = 30).
```

## 16.13 Environment (`env`)

```lp
use env.

key = env.get("API_KEY").
key = env.get("PORT", default = "8080").
home = env.home().
cwd = env.cwd().
env.set("DEBUG", "true").    @ Set env var for child processes
```

## 16.14 Reflection (`reflect`)

```lp
use reflect.

type_name = reflect.type_of(value).
fields = reflect.fields(value).
reflect.call_method(value, "greet", ["Naga"]).
```

## 16.15 Regular Expressions

```lp
use regex.

pattern = regex.compile("\\d+").
matches = pattern.findall("abc123def456").
matched = pattern.match("123").
replaced = pattern.replace("a1b2c3", "X").
```

## 16.16 Logging

```lp
use logging.

logger = logging.get("myapp").
logger.info("Application started").
logger.warning("Low memory").
logger.error("Connection failed").
logger.debug("Processing item " with id).
```

Configuration:

```lp
logging.configure(
    level = logging.INFO.
    format = "{time} [{level}] {message}".
    output = "app.log".
).
```

## 16.17 Standard Library Design Principles

1. **Read like instructions** — `read "file.txt"`, `write data to "output.txt"`.
2. **Sensible defaults** — minimal configuration for common cases.
3. **Progressive disclosure** — simple API first, advanced options available.
4. **Consistent naming** — snake_case functions, PascalCase types.
5. **Documented examples** — every public function includes an example.
6. **Tested** — 100% unit test coverage for stdlib.

## 16.18 Stdlib Versioning

The standard library version matches the language version. Stdlib modules follow semver within the language release cycle. Breaking changes to stdlib require a major language version bump.

---

# Chapter 17 — Navigator Framework

## 17.1 Overview

Navigator is Lang.P's flagship framework for building desktop browsers and rich desktop applications. It enables users to create a complete Chrome-like browser with minimal code.

```lp
use navigator.
```

## 17.2 Design Goals

1. **Beginner-friendly** — a working browser in under 20 lines.
2. **Fully customizable** — every UI element can be configured or replaced.
3. **IDE-integrated** — Lang Studio generates complete browser templates with explanatory comments.
4. **Production-capable** — real Chromium/WebKit rendering engine underneath.

## 17.3 Creating a Browser

### 17.3.1 Minimal Browser

```lp
use navigator.

browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
.

@ Browser event loop starts automatically
```

### 17.3.2 Full Configuration

```lp
use navigator.

browser = Browser(),
    name = "Nova".
    homepage = "https://google.com".
    theme = dark.
    width = 1400.
    height = 900.
    tabs = enabled.
    bookmarks = enabled.
    history = enabled.
    downloads = enabled.
    devtools = enabled.
    user_agent = "Nova/1.0".
    cache_size = 100.
    javascript = enabled.
    images = enabled.
    cookies = enabled.
.
```

### 17.3.3 Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | String | "Browser" | Window title and app name |
| `homepage` | String | "about:blank" | Default URL on new tab |
| `theme` | Theme | light | `light`, `dark`, or custom |
| `width` | Int | 1200 | Window width in pixels |
| `height` | Int | 800 | Window height in pixels |
| `tabs` | Bool | enabled | Enable tab bar |
| `bookmarks` | Bool | enabled | Enable bookmarks bar |
| `history` | Bool | enabled | Enable browsing history |
| `downloads` | Bool | enabled | Enable download manager |
| `devtools` | Bool | disabled | Enable developer tools |
| `user_agent` | String | auto | Custom user agent string |
| `cache_size` | Int | 50 | Cache size in MB |
| `javascript` | Bool | enabled | Enable JavaScript |
| `images` | Bool | enabled | Load images |
| `cookies` | Bool | enabled | Enable cookies |

## 17.4 Browser Events

```lp
on Browser.Start,
    print "Browser started".
.

on Browser.Close,
    print "Browser closed".
    save_session().
.

on browser.navigation,
    url = browser.navigation.url.
    print "Navigating to: " with url.
.

on browser.tab_changed,
    print "Active tab: " with browser.tab_changed.title.
.

on browser.download_started,
    print "Downloading: " with browser.download_started.filename.
.

on browser.download_completed,
    print "Download complete: " with browser.download_completed.path.
.

on browser.title_changed,
    @ Update window title
.

on browser.page_loaded,
    print "Page loaded: " with browser.page_loaded.url.
.
```

## 17.5 Browser API

```lp
@ Navigation
browser.navigate("https://example.com").
browser.go_back().
browser.go_forward().
browser.reload().
browser.stop().

@ Tabs
tab = browser.new_tab("https://google.com").
browser.close_tab(tab).
browser.switch_tab(tab).
tabs = browser.tabs().

@ JavaScript execution
result = browser.execute_js("document.title").
browser.inject_css("body { background: #1a1a2e; }").

@ Screenshots
browser.screenshot("page.png").
browser.screenshot_region(x = 0, y = 0, width = 800, height = 600, path = "region.png").

@ Bookmarks
browser.add_bookmark("Lang.P", "https://langp.dev").
bookmarks = browser.bookmarks().

@ History
history = browser.history().
browser.clear_history().

@ Downloads
browser.download("https://example.com/file.zip").
downloads = browser.downloads().
```

## 17.6 Custom UI Components

Navigator supports custom UI overlays:

```lp
use navigator.

@ Custom toolbar button
toolbar = browser.toolbar().
button = toolbar.add_button(
    icon = "star".
    tooltip = "Bookmark this page".
).

on button.clicked,
    browser.add_bookmark(browser.current_title, browser.current_url).
.

@ Custom sidebar panel
sidebar = browser.sidebar(width = 300).
sidebar.add_panel(
    title = "Notes".
    content = NotesPanel().
).
```

## 17.7 Themes

Built-in themes:

```lp
browser = Browser(), theme = dark.
browser = Browser(), theme = light.
```

Custom themes:

```lp
my_theme = Theme(),
    background = "#1a1a2e".
    foreground = "#eaeaea".
    accent = "#6c63ff".
    toolbar = "#16213e".
    tab_active = "#0f3460".
    tab_inactive = "#1a1a2e".
    font = "Inter".
    font_size = 14.
.

browser = Browser(), theme = my_theme.
```

## 17.8 Browser Extensions

Extensions add functionality to the browser:

```lp
type AdBlocker extends Extension,
    name = "Ad Blocker".

    on browser.page_loaded,
        browser.execute_js("""
            document.querySelectorAll('[class*="ad"]').forEach(el => el.remove());
        """).
    .
.

browser.install_extension(AdBlocker()).
```

## 17.9 IDE Template Generation

Lang Studio generates a complete browser project:

```
my-browser/
    lang.toml
    main.lp              @ Generated with explanatory comments
    assets/
        icon.png
    themes/
        custom.lp
    extensions/
        adblocker.lp
```

Every generated line includes a beginner-friendly `@` comment explaining its purpose. Comments can be hidden/shown via IDE toggle.

Example generated `main.lp`:

```lp
@ Import the Navigator framework — this gives us browser capabilities.
use navigator.

@ Create a new browser window with custom settings.
browser = Browser(),
    @ The name shown in the window title bar.
    name = "Nova".
    @ The page loaded when the browser first opens.
    homepage = "https://google.com".
    @ Use dark color scheme for the browser UI.
    theme = dark.
    @ Window dimensions in pixels.
    width = 1400.
    height = 900.
    @ Enable the tab bar at the top of the window.
    tabs = enabled.
    @ Enable the bookmarks bar below the address bar.
    bookmarks = enabled.
.

@ This event fires when the browser finishes starting up.
on Browser.Start,
    print "Welcome to Nova Browser!".
.
```

## 17.10 Architecture

```
┌─────────────────────────────────────┐
│           Lang.P Application        │
├─────────────────────────────────────┤
│         Navigator Framework         │
│  ┌─────────┐ ┌────────┐ ┌────────┐ │
│  │ Browser │ │  Tabs  │ │  Theme │ │
│  │ Engine  │ │ Manager│ │ Engine │ │
│  └────┬────┘ └────────┘ └────────┘ │
├───────┼─────────────────────────────┤
│       ▼                             │
│  Native WebView (Chromium/WebKit)   │
├─────────────────────────────────────┤
│         Lang.P Runtime              │
└─────────────────────────────────────┘
```

Navigator wraps a native web rendering engine (Chromium on all platforms, WebKit on macOS as fallback). The Lang.P code controls the browser chrome; the engine handles page rendering.

## 17.11 Desktop Applications

Navigator also supports non-browser desktop apps:

```lp
use navigator.

app = Application(),
    name = "My App".
    width = 800.
    height = 600.
.

window = app.window().
window.add(Button(text = "Click Me")).
window.add(TextLabel(text = "Hello, Lang.P!")).

on app.start,
    window.show().
.
```

UI components: `Button`, `TextLabel`, `TextInput`, `Checkbox`, `Dropdown`, `ListView`, `Canvas`, `MenuBar`, `Dialog`.

## 17.12 Platform Support

| Platform | Status | Engine |
|----------|--------|--------|
| macOS | v1.0 | Chromium / WebKit |
| Windows | v1.0 | Chromium |
| Linux | v1.0 | Chromium |
| Mobile | Future | TBD |

---

# Chapter 18 — AI Framework

## 18.1 Overview

The Lang.P AI framework provides first-class support for building AI-powered applications, agents, and LLM integrations. AI is a core part of the Lang.P ecosystem, not an afterthought.

```lp
use ai.
```

## 18.2 Design Goals

1. **Provider-agnostic** — switch between OpenAI, Anthropic, Groq, Google, Ollama without code changes.
2. **Beginner-friendly** — a working chatbot in under 10 lines.
3. **Production-ready** — streaming, tool calling, memory, RAG, agents.
4. **MCP-native** — Model Context Protocol integration built in.

## 18.3 Creating an Assistant

### 18.3.1 Basic Assistant

```lp
use ai.

assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
.

on user.message,
    reply = assistant.chat(user.message).
    print reply.
.
```

### 18.3.2 Full Configuration

```lp
assistant = Assistant(),
    provider = OpenAI.
    api_key = env.OPENAI_API_KEY.
    model = gpt-4o.
    temperature = 0.7.
    max_tokens = 4096.
    system_prompt = "You are a helpful coding assistant.".
    streaming = enabled.
    memory = enabled.
    tools = [search_web, run_code, read_file].
.
```

## 18.4 Supported Providers

| Provider | Configuration | Models |
|----------|--------------|--------|
| `OpenAI` | `api_key = env.OPENAI_API_KEY` | gpt-4o, gpt-4o-mini, o1 |
| `Anthropic` | `api_key = env.ANTHROPIC_API_KEY` | claude-sonnet-4, claude-opus-4 |
| `Google` | `api_key = env.GOOGLE_API_KEY` | gemini-2.0-flash, gemini-2.0-pro |
| `Groq` | `api_key = env.GROQ_API_KEY` | llama-3.3-70b, mixtral-8x7b |
| `OpenRouter` | `api_key = env.OPENROUTER_API_KEY` | All supported models |
| `Ollama` | `host = "http://localhost:11434"` | Local models |

Switching providers requires changing only the provider and model:

```lp
@ Development — local Ollama
assistant = Assistant(),
    provider = Ollama.
    model = llama3.
.

@ Production — Groq
assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
.
```

## 18.5 Chat

### 18.5.1 Simple Chat

```lp
reply = assistant.chat("What is Lang.P?").
print reply.
```

### 18.5.2 Conversation

```lp
conversation = assistant.conversation().

reply1 = conversation.send("Hello").
reply2 = conversation.send("What can you help me with?").
history = conversation.history().
conversation.clear().
```

### 18.5.3 Streaming

```lp
on user.message,
    stream = assistant.stream(user.message).
    async for chunk in stream,
        print inline chunk.
    .
    print "".
.
```

## 18.6 Tool Calling

Define tools that the AI can invoke:

```lp
@ Tool definition
function search_web(query: String) -> String,
    results = wait for get ("https://search.example.com?q=" with query).
    return results.body.
.

function run_code(code: String) -> String,
    result = execute_sandbox(code).
    return result.output.
.

assistant = Assistant(),
    provider = OpenAI.
    api_key = env.OPENAI_API_KEY.
    model = gpt-4o.
    tools = [search_web, run_code].
.

@ The assistant automatically calls tools when needed
reply = assistant.chat("Search for the latest Lang.P news and summarize").
```

Tool definition with metadata:

```lp
tool get_weather,
    description = "Get current weather for a city".
    parameter city: String, description = "City name".
    return fetch_weather(city).
.
```

## 18.7 Agents

Agents are autonomous AI entities that can plan, execute, and iterate:

```lp
agent = Agent(),
    assistant = assistant.
    goal = "Research and write a summary about quantum computing".
    max_steps = 10.
    tools = [search_web, read_page, write_file].
.

result = wait for agent.run().
print result.summary.
```

Agent workflow:

```
Goal → Plan → Execute Tool → Evaluate → Replan → ... → Result
```

## 18.8 Embeddings

```lp
use ai.

embeddings = Embeddings(),
    provider = OpenAI.
    model = text-embedding-3-small.
.

vector = embeddings.embed("Hello, world").
vectors = embeddings.embed_batch(["text1", "text2"]).
similarity = embeddings.cosine_similarity(vector1, vector2).
```

## 18.9 RAG (Retrieval-Augmented Generation)

```lp
use ai.

@ Build a knowledge base
kb = KnowledgeBase(),
    embeddings = embeddings.
.

kb.add_document("manual.pdf").
kb.add_text("Lang.P is a readable programming language.").
kb.add_directory("docs/").

@ Query with context
assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
    knowledge = kb.
.

reply = assistant.chat("How do I create a browser in Lang.P?").
@ Response is grounded in the knowledge base
```

## 18.10 Memory

Persistent conversation memory:

```lp
assistant = Assistant(),
    provider = OpenAI.
    model = gpt-4o.
    memory = Memory(),
        type = persistent.
        storage = "memory.db".
        max_entries = 1000.
.
.

@ The assistant remembers past conversations
reply = assistant.chat("What did we discuss yesterday?").
```

Memory types:

| Type | Description |
|------|-------------|
| `session` | Current session only |
| `persistent` | Stored across sessions |
| `semantic` | Vector-based semantic recall |

## 18.11 MCP Integration

Model Context Protocol support:

```lp
use ai.mcp.

@ Connect to MCP servers
mcp = MCPClient(),
    servers = [
        "filesystem": "npx @modelcontextprotocol/server-filesystem /path".
        "github": "npx @modelcontextprotocol/server-github".
    ].
.

assistant = Assistant(),
    provider = Anthropic.
    model = claude-sonnet-4.
    mcp = mcp.
.

@ Assistant can use MCP tools (file access, GitHub, etc.)
reply = assistant.chat("List the files in my project and create a README").
```

## 18.12 AI Events

```lp
on user.message,
    @ Fires when user sends a message
.

on assistant.response,
    @ Fires when assistant completes a response
    print assistant.response.text.
.

on assistant.stream_chunk,
    @ Fires for each streaming chunk
    print inline assistant.stream_chunk.text.
.

on assistant.tool_call,
    @ Fires when assistant invokes a tool
    print "Calling: " with assistant.tool_call.name.
.

on assistant.error,
    @ Fires on API or processing errors
    print "Error: " with assistant.error.message.
.
```

## 18.13 Structured Output

```lp
type Analysis = ,
    summary: String.
    sentiment: String.
    keywords: List<String>.
.

result = assistant.structured("Analyze this text: " with text, type = Analysis).
print result.summary.
print result.sentiment.
```

## 18.14 Multi-Modal

```lp
@ Image understanding
reply = assistant.chat("Describe this image", image = "photo.jpg").

@ Image generation (provider-dependent)
image = assistant.generate_image("A sunset over mountains").
write_bytes image to "sunset.png".
```

## 18.15 AI in Lang Studio

Lang Studio integrates AI throughout the IDE:

- **AI Assistant panel** — chat with AI about your code.
- **Inline suggestions** — AI-powered code completion.
- **Explain code** — select code, ask AI to explain.
- **Generate from comment** — write a `@` comment describing what you want, AI generates code.
- **Fix errors** — AI suggests fixes for compiler errors.
- **Generate tests** — AI writes test cases for functions.

## 18.16 Cost Tracking

```lp
usage = assistant.usage().
print "Tokens used: " with usage.total_tokens.
print "Estimated cost: $" with usage.estimated_cost.
```

## 18.17 Error Handling

```lp
try,
    reply = assistant.chat("Hello").
catch error: ai.RateLimitError,
    print "Rate limited. Retry after " with error.retry_after with " seconds".
    wait for sleep(seconds = error.retry_after).
    reply = assistant.chat("Hello").
catch error: ai.AuthenticationError,
    print "Invalid API key".
catch error: ai.ModelNotFoundError,
    print "Model not available: " with error.model.
..
```

## 18.18 Best Practices

1. Store API keys in environment variables, never in source code.
2. Use streaming for long responses to improve perceived latency.
3. Set `max_tokens` to prevent runaway costs.
4. Use RAG for domain-specific knowledge instead of long system prompts.
5. Define focused tools — one tool per capability.
6. Use `structured` output when you need parseable responses.

---

# Chapter 19 — Runtime & Memory

## 19.1 Overview

The Lang.P runtime provides memory management, execution, foreign function interface, and platform abstraction. The runtime is embedded in every compiled Lang.P binary.

## 19.2 Execution Models

Lang.P supports two execution modes:

| Mode | Description | Use case |
|------|-------------|----------|
| **Interpreted** | Tree-walking interpreter via AST/bytecode | Development, REPL, scripting |
| **Compiled** | Native machine code via LLVM backend | Production, performance-critical |

Both modes share the same runtime and semantics.

## 19.3 Compilation Pipeline

```
Source (.lp)
    │
    ▼
┌─────────┐
│  Lexer  │  Token stream
└────┬────┘
     ▼
┌─────────┐
│  Parser │  Abstract Syntax Tree
└────┬────┘
     ▼
┌──────────────┐
│  Semantic    │  Typed AST + diagnostics
│  Analyzer    │
└────┬─────────┘
     ▼
┌──────────────┐
│  Optimizer   │  Optimized IR
└────┬─────────┘
     ▼
┌──────────────┐     ┌──────────────┐
│  Interpreter │ OR  │  Compiler    │  Native binary
│  (bytecode)  │     │  (LLVM IR)   │
└────┬─────────┘     └──────┬───────┘
     │                      │
     └──────────┬───────────┘
                ▼
         ┌──────────────┐
         │   Runtime    │
         │  (GC, FFI)   │
         └──────────────┘
```

## 19.4 Intermediate Representation (IR)

Lang.P IR is a typed, SSA-based intermediate representation:

```
function @add(a: i64, b: i64) -> i64 {
entry:
    %0 = add i64 %a, %b
    ret i64 %0
}
```

IR properties:

- Static single assignment (SSA) form
- Typed operations matching Lang.P type system
- Platform-independent
- Optimizable (dead code elimination, inlining, constant folding)

## 19.5 Memory Management

### 19.5.1 Garbage Collection

Lang.P uses a **generational, concurrent garbage collector**:

| Generation | Description | Collection frequency |
|------------|-------------|---------------------|
| Young | New allocations | Frequent, stop-the-world (fast) |
| Old | Long-lived objects | Infrequent, concurrent |
| Permanent | Static constants, types | Never collected |

GC properties:

- **Concurrent marking** — minimal pause times.
- **Write barriers** — track old→young references.
- **Finalizers** — `destroy()` methods called before collection.
- **GC tuning** — `--gc-threshold`, `--gc-debug` flags.

### 19.5.2 Stack vs Heap

| Location | Types |
|----------|-------|
| Stack | Primitives (`Int`, `Float64`, `Bool`, `Char`), references |
| Heap | `String`, collections, objects, closures |

Value types (structs without heap allocation) MAY be stack-allocated when the optimizer determines it is safe.

### 19.5.3 Memory Safety

- **No manual memory management** — no `malloc`/`free`.
- **Null safety** — nullable types require explicit `?` annotation.
- **Bounds checking** — array/list index access is bounds-checked (can be optimized away when proven safe).
- **No dangling pointers** — GC prevents use-after-free.

## 19.6 Calling Conventions

### 19.6.1 Lang.P Functions

Arguments passed left-to-right, with the receiver (`self`) as the first argument for methods. Return value in a designated register or via hidden pointer for large returns.

### 19.6.2 Foreign Function Interface (FFI)

Call C functions from Lang.P:

```lp
use ffi.

@ Declare external C function
extern function strlen(s: Pointer<Byte>) -> Int from "libc".

length = strlen(c_string("hello")).
```

Call Lang.P from C:

```c
// Generated header: mylib.h
int64_t langp_add(int64_t a, int64_t b);
void langp_greet(const char* name);
```

FFI rules:

- `extern` functions MUST specify the library name.
- String marshalling: Lang.P `String` ↔ C `const char*`.
- Memory ownership: caller owns arguments; callee owns return values.

## 19.7 Threading Model

- **OS threads** — mapped 1:1 to Lang.P threads.
- **Async tasks** — multiplexed on a thread pool (work-stealing).
- **Thread-local storage** — supported via `thread_local` keyword (v0.2).
- **No global lock** — concurrent GC, lock-free data structures where possible.

## 19.8 Runtime Initialization

```
1. Platform abstraction init (signals, locale)
2. GC init (heap allocation)
3. Standard library init (register built-in modules)
4. Module loader init
5. Execute module initializers (in dependency order)
6. Execute main() or top-level statements
7. Enter event loop (if applicable)
8. Shutdown: finalize, GC sweep, platform cleanup
```

## 19.9 Error Reporting at Runtime

Runtime errors include:

- Stack trace with Lang.P source locations
- Variable values at each frame (in debug mode)
- Suggestion for common errors

```
panic: IndexError: index 5 out of bounds (length 3)
  --> src/main.lp:12:5
   |
12 |     print items[5].
   |           ^^^^^^^^
   |
  stack trace:
    main at src/main.lp:12
    process at src/utils.lp:8
```

## 19.10 Platform Abstraction

The runtime abstracts platform differences:

| Feature | macOS | Windows | Linux |
|---------|-------|---------|-------|
| File paths | POSIX + `/` | Win32 + `\` (normalized) | POSIX |
| Dynamic loading | dlopen | LoadLibrary | dlopen |
| Threads | pthread | Win32 threads | pthread |
| Signals | POSIX signals | SEH | POSIX signals |

Lang.P code is platform-independent unless using platform-specific modules.

## 19.11 Binary Format

Compiled Lang.P binaries:

| Extension | Description |
|-----------|-------------|
| `.lpc` | Lang.P compiled object file |
| (none) | Executable binary |

Object files contain IR metadata, debug symbols, and dependency information for linking.

## 19.12 Debug Information

Debug builds include:

- Source file mapping (IR ↔ source)
- Variable names and types
- Line number tables
- Inline stack frames

Used by the debugger, profiler, and error reporting.

## 19.13 Performance Characteristics

Target performance (compiled mode, relative to C):

| Benchmark | Target |
|-----------|--------|
| Numeric computation | 80-95% of C |
| String processing | 70-85% of C |
| Object-oriented code | 75-90% of C |
| Async I/O | Comparable to Go |
| Startup time | < 50ms (compiled) |

The interpreter runs 10-50x slower than compiled code and is intended for development only.

## 19.14 Resource Limits

Configurable limits:

| Resource | Default | Flag |
|----------|---------|------|
| Stack size | 8 MB | `--stack-size` |
| Heap size | Unlimited (GC) | `--max-heap` |
| Recursion depth | 10,000 | `--max-recursion` |
| Open files | OS limit | — |
| Thread count | 10,000 | `--max-threads` |

Exceeding limits causes a catchable `ResourceError`.

---

# Chapter 20 — Package System

## 20.1 Overview

The `lang` package manager handles dependency resolution, installation, publishing, and project management for Lang.P packages.

## 20.2 Project Manifest

Every Lang.P project has a `lang.toml` manifest:

```toml
[package]
name = "my-browser"
version = "1.0.0"
description = "A custom web browser built with Lang.P"
authors = ["Naga <naga@example.com>"]
license = "MIT"
entry = "main.lp"

[dependencies]
navigator = "1.0"
requests = "2.1"
my-utils = { git = "https://github.com/user/my-utils", branch = "main" }

[dev-dependencies]
testing = "1.0"

[build]
target = "native"
optimization = 2
```

### 20.2.1 Manifest Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Package name (snake_case) |
| `version` | Yes | Semver version |
| `description` | No | Human-readable description |
| `authors` | No | List of authors |
| `license` | No | SPDX license identifier |
| `entry` | No | Entry point file (default: `main.lp`) |
| `dependencies` | No | Runtime dependencies |
| `dev-dependencies` | No | Development/test dependencies |
| `build.target` | No | `native`, `bytecode`, `wasm` |
| `build.optimization` | No | 0-3 (none to aggressive) |

## 20.3 CLI Commands

### 20.3.1 Project Management

```bash
lang init my-project          @ Create new project
lang init --template browser  @ Create from template
lang build                    @ Build the project
lang run                      @ Build and run
lang test                     @ Run tests
lang clean                    @ Remove build artifacts
```

### 20.3.2 Dependency Management

```bash
lang add requests             @ Add dependency
lang add --dev testing        @ Add dev dependency
lang remove requests          @ Remove dependency
lang update                   @ Update all dependencies
lang update requests          @ Update specific dependency
lang install                  @ Install dependencies from lock file
```

### 20.3.3 Publishing

```bash
lang publish                  @ Publish to registry
lang search browser           @ Search packages
lang info navigator           @ Show package info
```

## 20.4 Package Layout

Standard project structure:

```
my-project/
    lang.toml           @ Manifest
    lang.lock           @ Lock file (auto-generated)
    main.lp             @ Entry point
    src/                @ Source modules
        utils.lp
        models/
            user.lp
    tests/              @ Test files
        test_utils.lp
    assets/             @ Static assets
    docs/               @ Documentation
    .lang/              @ Local cache (gitignored)
```

## 20.5 Dependency Resolution

The resolver:

1. Reads `lang.toml` dependencies.
2. Fetches packages from registry, git, or local path.
3. Resolves version constraints (semver).
4. Detects and rejects circular dependencies.
5. Writes resolved versions to `lang.lock`.

Version constraints:

```toml
navigator = "1.0"           @ Exactly 1.0.x (>= 1.0.0, < 1.1.0)
requests = "^2.1"           @ Compatible (>= 2.1.0, < 3.0.0)
my-lib = "~1.2.3"           @ Patch updates (>= 1.2.3, < 1.3.0)
utils = ">=1.0.0"           @ Minimum version
pinned = "=1.5.0"           @ Exact version
```

## 20.6 Lock File

`lang.lock` pins exact versions for reproducible builds:

```toml
[package]
name = "my-browser"
version = "1.0.0"

[[dependencies]]
name = "navigator"
version = "1.0.3"
checksum = "sha256:abc123..."

[[dependencies]]
name = "requests"
version = "2.1.0"
checksum = "sha256:def456..."
```

Lock files MUST be committed to version control.

## 20.7 Package Registry

Packages are published to `registry.langp.dev` (default):

```bash
lang publish
@ Uploads package after validation:
@   - Valid lang.toml
@   - Passes all tests
@   - Version not already published
@   - License specified
```

Private registries:

```bash
lang config set registry "https://my-registry.company.com"
```

## 20.8 Project Templates

Built-in templates:

| Template | Command | Description |
|----------|---------|-------------|
| `default` | `lang init` | Empty project |
| `browser` | `lang init --template browser` | Navigator browser |
| `api` | `lang init --template api` | HTTP API server |
| `agent` | `lang init --template agent` | AI agent |
| `cli` | `lang init --template cli` | CLI tool |
| `library` | `lang init --template library` | Library package |

Custom templates:

```bash
lang init --template https://github.com/user/my-template
```

## 20.9 Workspaces

Monorepo support (v0.2):

```toml
[workspace]
members = ["compiler", "runtime", "stdlib", "lang-studio"]
```

## 20.10 Versioning Policy

Packages follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** — incompatible API changes
- **MINOR** — backward-compatible new features
- **PATCH** — backward-compatible bug fixes

Pre-release versions: `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.1`.

## 20.11 Package Naming

- Package names: `snake_case`, lowercase, 3-64 characters.
- Scoped packages (v0.2): `@org/package-name`.
- Names MUST NOT conflict with stdlib module names.

Reserved names: `core`, `stdlib`, `test`, `lang`, `langc`, `navigator`, `ai`.

## 20.12 Local Dependencies

```toml
[dependencies]
my-lib = { path = "../my-lib" }
other = { path = "./vendor/other" }
```

Local dependencies are useful for monorepos and development.

## 20.13 Git Dependencies

```toml
[dependencies]
my-lib = { git = "https://github.com/user/my-lib" }
my-lib = { git = "https://github.com/user/my-lib", branch = "dev" }
my-lib = { git = "https://github.com/user/my-lib", tag = "v1.0.0" }
my-lib = { git = "https://github.com/user/my-lib", rev = "abc123" }
```

## 20.14 Feature Flags

Optional features (v0.2):

```toml
[dependencies]
database = { version = "1.0", features = ["postgres", "sqlite"] }
```

```lp
@ Code conditional on features
when feature("postgres"),
    use database.postgres.
.
```

---

# Chapter 21 — Tooling

## 21.1 Overview

Lang.P tooling is a first-class part of the ecosystem. The IDE teaches the language; tools make development productive from day one.

| Tool | Binary | Purpose |
|------|--------|---------|
| Compiler | `langc` | Compile `.lp` to native code or bytecode |
| Package Manager | `lang` | Dependencies, build, test, publish |
| Language Server | `lang-lsp` | LSP for IDE integration |
| IDE | Lang Studio | Official integrated development environment |
| Formatter | `langfmt` | Code formatting (part of lang-lsp) |
| REPL | `lang-repl` | Interactive interpreter |
| Debugger | `lang-debug` | Debug adapter protocol |

## 21.2 Compiler (`langc`)

### 21.2.1 Usage

```bash
langc main.lp                    @ Compile to executable
langc main.lp -o myapp           @ Specify output name
langc main.lp --mode interpret   @ Run via interpreter
langc main.lp --mode compile     @ Compile to native (default)
langc main.lp --mode bytecode    @ Compile to bytecode
langc main.lp --mode check       @ Type-check only
langc main.lp --emit ast         @ Dump AST
langc main.lp --emit ir          @ Dump IR
langc main.lp --emit tokens      @ Dump tokens
```

### 21.2.2 Flags

| Flag | Description |
|------|-------------|
| `-o <file>` | Output file path |
| `--mode <mode>` | `compile`, `interpret`, `bytecode`, `check` |
| `--emit <target>` | `ast`, `ir`, `tokens`, `llvm` |
| `-O <level>` | Optimization level (0-3) |
| `-g` | Debug symbols |
| `--warn <level>` | Warning level: `off`, `default`, `pedantic` |
| `--target <triple>` | Target platform (cross-compilation) |
| `--stdlib <path>` | Custom stdlib path |
| `--version` | Print version |
| `--help` | Print help |

### 21.2.3 Error Output

```
error[E0301]: type mismatch
  --> src/main.lp:5:5
   |
 5 |     result: String = 42.
   |                      ^^ expected String, found Int
   |
  = help: use to_string(42) to convert

error: compilation failed with 1 error
```

Colors enabled by default; `--color=never` to disable.

## 21.3 Language Server (`lang-lsp`)

### 21.3.1 Capabilities

| Feature | LSP Method | Status |
|---------|-----------|--------|
| Diagnostics | `textDocument/publishDiagnostics` | v0.1 |
| Autocomplete | `textDocument/completion` | v0.1 |
| Hover documentation | `textDocument/hover` | v0.1 |
| Go to definition | `textDocument/definition` | v0.1 |
| Find references | `textDocument/references` | v0.1 |
| Rename symbol | `textDocument/rename` | v0.1 |
| Formatting | `textDocument/formatting` | v0.1 |
| Semantic highlighting | `textDocument/semanticTokens` | v0.1 |
| Signature help | `textDocument/signatureHelp` | v0.1 |
| Code actions | `textDocument/codeAction` | v0.1 |
| Inlay hints | `textDocument/inlayHint` | v0.2 |
| Folding ranges | `textDocument/foldingRange` | v0.2 |
| Document symbols | `textDocument/documentSymbol` | v0.1 |

### 21.3.2 Input Type Quick-Fix

When the compiler emits warning `W0101` (input type could be more specific), the language server MUST offer a code action to convert generic `input` to typed `input`:

**Before:**

```lp
age = input "Age : ".
print age + 1.
```

**Diagnostic:**

```
warning[W0101]: input type could be more specific
  --> main.lp:1:7
   |
 1 | age = input "Age : ".
   |       ^^^^^^^^^^^^^^ the value "age" appears to be used as a number
   |
  = help: consider using: age = input number "Age : ".
```

**Quick-fix action:** `Convert to typed input (number)`

**After:**

```lp
age = input number "Age : ".
print age + 1.
```

The quick-fix MUST:

1. Insert the correct input type keyword (`text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, or `color`) based on inferred usage.
2. Preserve the prompt string exactly.
3. Suppress `W0101` after application.

Lang Studio displays this as a lightbulb action and inline "Fix in Lang Studio" link on the warning underline.

Additional input-related code actions (v0.2):

| Action | Description |
|--------|-------------|
| `Add explicit type annotation` | Add `: Int` etc. instead of input type keyword |
| `Wrap in try/catch for InputCancelledError` | For picker-based input expressions |

### 21.3.3 Configuration

```json
{
    "langp.lsp.path": "/usr/local/bin/lang-lsp",
    "langp.lsp.trace": "off",
    "langp.format.indentSize": 4,
    "langp.format.maxLineLength": 100,
    "langp.inlayHints.enabled": true,
    "langp.diagnostics.enabled": true
}
```

## 21.4 Lang Studio (IDE)

### 21.4.1 Features

| Feature | Description |
|---------|-------------|
| **Auto indentation** | Typing `,` indents; typing `..` dedents |
| **Syntax highlighting** | Full semantic highlighting via LSP |
| **Autocomplete** | Context-aware suggestions with documentation |
| **Hover documentation** | Type info, docs, and examples on hover |
| **Go to definition** | Jump to symbol definition |
| **Rename symbol** | Refactor names across project |
| **Formatting** | Format on save, format selection |
| **Debugger** | Breakpoints, step, watch, call stack |
| **Profiler** | CPU and memory profiling |
| **Package manager** | GUI for `lang add/remove/update` |
| **AI Assistant** | Built-in AI chat and code generation |
| **Visual Browser Designer** | Drag-and-drop browser UI builder |
| **Live Preview** | Run and preview in IDE |
| **Integrated terminal** | Built-in terminal |
| **Integrated documentation** | Browse stdlib docs in IDE |
| **Project templates** | One-click project creation |
| **Comment toggle** | Show/hide `@` comments in generated code |
| **Error explanations** | AI-powered error fix suggestions |

### 21.4.2 Auto Indentation Behavior

When the user types `,` at the end of a block header:

```
if age >= 18,█
```

The IDE automatically transforms to:

```
if age >= 18,
    █
```

When the user types `..`:

```
if age >= 18,
    print "Adult".
    ..█
```

The IDE dedents:

```
if age >= 18,
    print "Adult".
..█
```

### 21.4.3 Visual Browser Designer

A drag-and-drop interface for building Navigator browser UIs:

- Component palette (toolbar, tabs, sidebar, address bar)
- Property inspector for selected components
- Live preview of browser chrome
- Generates commented Lang.P source code
- Theme editor with color picker

### 21.4.4 AI Integration

Lang Studio's AI assistant can:

- Explain selected code in plain language
- Generate code from `@` comments
- Fix compiler errors
- Suggest completions
- Generate tests
- Answer Lang.P language questions

## 21.5 Formatter (`langfmt`)

Enforces consistent style:

```bash
langfmt main.lp              @ Format file in place
langfmt --check main.lp      @ Check formatting without changes
langfmt --diff main.lp       @ Show formatting diff
```

Formatting rules:

- 4-space indentation
- One statement per line
- Blank line between top-level declarations
- No trailing whitespace
- Maximum line length: 100 characters (soft wrap)
- Block opener `,` at end of header line
- Block closer `..` at parent indentation level

## 21.6 REPL (`lang-repl`)

Interactive interpreter:

```bash
lang-repl
```

```
Lang.P v0.1.0
>>> name = "Naga".
>>> print "Hello " with name.
Hello Naga
>>> function greet(n), print "Hi " with n. .
>>> greet("World").
Hi World
>>> :type name
String
>>> :help
>>> :exit
```

REPL commands:

| Command | Description |
|---------|-------------|
| `:type <expr>` | Show type of expression |
| `:ast <expr>` | Show AST |
| `:help` | Show help |
| `:load <file>` | Load and execute file |
| `:reset` | Reset environment |
| `:exit` | Exit REPL |

## 21.7 Debugger (`lang-debug`)

Debug Adapter Protocol (DAP) compatible:

```bash
lang-debug main.lp
```

Features:

- Breakpoints (line, conditional, logpoint)
- Step over, step into, step out, continue
- Variable inspection and modification
- Watch expressions
- Call stack navigation
- Exception breakpoints

## 21.8 Profiler

```bash
langc main.lp --profile cpu.
langc main.lp --profile memory.
```

Output formats: text, flamegraph (HTML), JSON.

## 21.9 Editor Extensions

Official extensions for:

| Editor | Extension |
|--------|-----------|
| Lang Studio | Built-in |
| VS Code | `langp-langp` |
| Cursor | `langp-langp` |
| Neovim | `langp.nvim` |
| JetBrains | `langp-plugin` |

All extensions use `lang-lsp` for language features.

## 21.10 Documentation Generator

```bash
lang doc                     @ Generate docs for current project
lang doc --output docs/      @ Specify output directory
```

Generates HTML documentation from `@` doc comments and type signatures.

Doc comment syntax:

```lp
@ Creates a new user with the given name and age.
@ Returns a User object ready for use.
@
@ Example:
@   user = create_user("Naga", 25).
@   print user.name.
function create_user(name: String, age: Int) -> User,
    @ ...
.
```

## 21.11 Continuous Integration

Recommended CI pipeline:

```yaml
@ .github/workflows/ci.yml (conceptual)
steps:
  - lang install
  - lang build
  - lang test
  - langfmt --check .
  - langc main.lp --mode check
```

## 21.12 Toolchain Installation

```bash
@ Install entire toolchain
curl -fsSL https://langp.dev/install | sh

@ Or via package manager
brew install langp          @ macOS
scoop install langp         @ Windows
apt install langp           @ Debian/Ubuntu
```

Installed tools: `langc`, `lang`, `lang-lsp`, `langfmt`, `lang-repl`, `lang-debug`.

Lang Studio is installed separately (desktop application).

---

# Chapter 22 — Compatibility & Versioning

## 22.1 Overview

Lang.P uses semantic versioning for the language specification, compiler toolchain, standard library, and package ecosystem. This chapter defines version numbering, compatibility guarantees, deprecation policy, and migration procedures.

## 22.2 Version Numbering

All Lang.P artifacts follow [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
```

| Component | Version scope | Example |
|-----------|---------------|---------|
| Language specification | Independent semver | `0.1.0` |
| Compiler (`langc`) | Matches language minor | `0.1.3` |
| Package manager (`lang`) | Independent semver | `0.1.1` |
| Standard library | Matches language minor | `0.1.0` |
| Language Server | Independent semver | `0.1.2` |
| Lang Studio | Independent semver | `1.0.0` |
| Third-party packages | Independent semver | `2.4.1` |

### 22.2.1 Version Components

- **MAJOR** — incompatible language or API changes requiring source migration.
- **MINOR** — backward-compatible new features, new stdlib modules, new keywords (rare).
- **PATCH** — backward-compatible bug fixes, documentation corrections, performance improvements.

### 22.2.2 Pre-release Identifiers

| Identifier | Meaning |
|------------|---------|
| `-alpha.N` | Early development, unstable |
| `-beta.N` | Feature-complete, testing |
| `-rc.N` | Release candidate |

Example: `0.2.0-beta.1`

## 22.3 Language Specification Versioning

The specification version is declared in `docs/spec/README.md` and propagated to all chapter headers.

Current version: **0.1.0**

### 22.3.1 What Constitutes a Spec Change

| Change type | Version bump | Example |
|-------------|-------------|---------|
| New keyword | MINOR (rare) or MAJOR | Adding `defer` |
| New stdlib module | MINOR | Adding `graphics` module |
| Syntax clarification | PATCH | Clarifying block indentation rules |
| Breaking syntax change | MAJOR | Changing statement terminator |
| New type | MINOR | Adding `Duration` type |
| Semantic change | MAJOR or MINOR | Changing integer division behavior |

### 22.3.2 Specification Amendment Process

1. **Proposal** — Open an amendment document describing the change, rationale, and alternatives considered.
2. **Review** — Evaluate against language philosophy (readability, simplicity, one obvious way).
3. **Impact analysis** — Identify affected chapters, tooling, stdlib, and migration path.
4. **Update** — Modify spec chapters, examples, and conformance tests.
5. **Version bump** — Increment spec version per §22.3.1.
6. **Announcement** — Publish changelog entry with migration guide.

## 22.4 Compatibility Guarantees

### 22.4.1 Within a Major Version

Within the same MAJOR version, Lang.P guarantees:

- Source code written for version `X.Y.0` MUST compile and run correctly on version `X.Y.Z` (any patch).
- Source code written for version `X.0.0` SHOULD compile on version `X.Y.Z` (any minor), except when using newly added features.
- The standard library MUST maintain backward compatibility within a major version.
- Package manifests (`lang.toml`) using semver constraints MUST resolve correctly across patches and minors.

### 22.4.2 Across Major Versions

Major version bumps MAY include:

- Removed keywords or syntax
- Changed default behavior
- Removed stdlib modules or functions
- Changed type system rules

Major migrations MUST include:

- A migration guide in `docs/migrations/vX-to-vY.md`
- Compiler warnings for deprecated features (one minor version before removal)
- Automated migration tool when changes affect > 10% of common patterns

### 22.4.3 Toolchain Compatibility Matrix

| langc version | Spec version | lang version | Minimum lang-lsp |
|---------------|-------------|--------------|------------------|
| 0.1.x | 0.1.0 | 0.1.x | 0.1.0 |
| 0.2.x | 0.2.0 | 0.2.x | 0.2.0 |

The compiler MUST report a warning when the project's `lang.toml` specifies a language version newer than the compiler supports.

## 22.5 Deprecation Policy

### 22.5.1 Timeline

| Stage | Duration | Behavior |
|-------|----------|----------|
| Active | Indefinite | Fully supported, no warnings |
| Deprecated | 1 MINOR version | Compiles with deprecation warning |
| Removed | Next MAJOR version | Compile error with migration hint |

Example timeline for deprecating a function:

```
v0.1.0  — function active
v0.2.0  — function deprecated (warning emitted)
v0.3.0  — function deprecated (warning emitted)
v1.0.0  — function removed (compile error)
```

### 22.5.2 Deprecation Syntax

```lp
@deprecated("Use fetch_v2 instead. Will be removed in v1.0.0.")
function fetch(url),
    return fetch_v2(url).
.
```

The compiler MUST emit:

```
warning[W0100]: deprecated
  --> src/api.lp:5:1
   |
 5 | function fetch(url),
   | ^^^^^^^^^^^^^^^^^^^^
   |
   = note: Use fetch_v2 instead. Will be removed in v1.0.0.
```

### 22.5.3 What Can Be Deprecated

- Functions and methods
- Type fields
- Stdlib modules (entire modules)
- Compiler flags
- Syntax forms (with long deprecation period)

Keywords MUST NOT be deprecated without a MAJOR version bump.

## 22.6 Edition System (Future)

Lang.P MAY adopt an edition system (similar to Rust) for major language evolution without breaking existing code:

```toml
[package]
name = "my-app"
edition = "2026"
```

Editions allow incompatible changes while letting projects opt in at their own pace. Editions are planned for v1.0+ if needed.

## 22.7 Feature Stability Levels

| Level | Label | Guarantee |
|-------|-------|-----------|
| Stable | (none) | Full compatibility within major version |
| Beta | `@beta` | API may change in patches; stabilized in next minor |
| Experimental | `@experimental` | No compatibility guarantee; may be removed anytime |
| Internal | `@internal` | Not part of public API; may change without notice |

Stdlib modules start as `@experimental`, graduate to `@beta`, then become stable.

## 22.8 Changelog

All releases MUST include a changelog following [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
# Changelog

## [0.2.0] - 2026-09-01

### Added
- `match` expression for pattern matching
- `defer` statement for scoped cleanup

### Changed
- Improved type inference for empty collections

### Deprecated
- `fetch()` in favor of `fetch_v2()`

### Removed
- (nothing)

### Fixed
- Fixed off-by-one in range expressions
```

Changelog location: `CHANGELOG.md` at repository root.

## 22.9 Package Version Constraints

Package dependencies use semver constraints in `lang.toml`:

```toml
[dependencies]
navigator = "1.0"       @ >= 1.0.0, < 1.1.0
requests = "^2.1"       @ >= 2.1.0, < 3.0.0
utils = "~1.2.3"        @ >= 1.2.3, < 1.3.0
pinned = "=1.5.0"       @ exactly 1.5.0
any = "*"               @ any version (discouraged)
```

The lock file (`lang.lock`) pins exact resolved versions for reproducible builds.

## 22.10 Language Version Declaration

Projects declare the language version they target:

```toml
[package]
name = "my-app"
version = "1.0.0"
lang-version = "0.1"
```

The compiler uses this to:

- Enable or disable features gated by version
- Emit appropriate warnings for deprecated features
- Select correct stdlib version

If `lang-version` is omitted, the compiler uses the latest stable version.

## 22.11 Backward Compatibility Checklist

Before releasing a new version, verify:

- [ ] All conformance tests pass
- [ ] No breaking changes without MAJOR bump
- [ ] Deprecated features have migration paths documented
- [ ] Changelog is updated
- [ ] Spec version is bumped
- [ ] Examples compile with new version
- [ ] Lock file format is compatible (or migration provided)

## 22.12 Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.0 | 2026-07-14 | Initial language specification |

---

# Glossary

Definitions of terms used throughout the Lang.P specification. Normative keywords appear in lowercase; type names in PascalCase.

## A

**Assignment statement** — A statement of the form `variable = expression.` that binds or rebinds a variable. See [Chapter 7 §7.2](07-statements.md#72-assignment-statement).

## B

**Built-in expression** — A language construct that produces a value without a function call or import, such as `input`, collection literals, or the `with` operator. See [Chapter 6](06-expressions.md).

## C

**Color** — An RGBA color value type returned by `input color` and defined in the `graphics` module. See [Chapter 4 §4.14.3](04-types.md#4143-the-color-type).

**Contextual keyword** — A token that is treated as a keyword only in specific syntactic positions (e.g., `text` after `input`, `otherwise if`). See [Chapter 2 §2.8.1](02-lexical-structure.md#281-input-type-keywords).

**Conformance example** — A code example marked as required behavior that implementations MUST satisfy. See [README](README.md#conformance).

## D

**Date** — A calendar date type (year, month, day) returned by `input date` and defined in the `datetime` module. See [Chapter 4 §4.14.2](04-types.md#4142-the-date-type).

## I

**InputCancelledError** — A `RuntimeError` thrown when the user cancels a native picker (`file`, `folder`, `date`, `color`). See [Chapter 6 §6.15.6](06-expressions.md#6156-error-handling).

**Input expression** — The built-in `input` keyword expression for reading user input from the terminal or native system pickers. No parentheses are used. See [Chapter 6 §6.15](06-expressions.md#615-input-expression).

**Input type keyword** — A contextual keyword after `input` that selects the input mode and return type: `text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, or `color`. See [Chapter 2 §2.8.1](02-lexical-structure.md#281-input-type-keywords).

**InputError** — A `RuntimeError` thrown when input fails after retries, on EOF, or when a picker is unavailable. See [Chapter 6 §6.15.6](06-expressions.md#6156-error-handling).

## P

**Picker input** — Input modes that open a native system dialog: `file`, `folder`, `date`, and `color`. See [Chapter 6 §6.15.3](06-expressions.md#6153-semantics).

**Prompt** — The string literal displayed to the user before input is collected, e.g., `"Enter your name : "`.

## Q

**Quick-fix** — An IDE code action that automatically applies a suggested fix, such as converting generic `input` to typed `input number`. See [Chapter 21 §21.3.2](21-tooling.md#2132-input-type-quick-fix).

## S

**Statement terminator** — The period (`.`) that ends every statement. See [Chapter 2 §2.4](02-lexical-structure.md#24-statement-terminator).

## T

**Type inference** — Compile-time deduction of types without explicit annotations. For `input`, the compiler infers the return type from assignment context and usage. See [Chapter 4 §4.14.5](04-types.md#4145-type-inference-for-default-input).

## W

**W0101** — Compiler warning: input type could be more specific. Emitted when generic `input` is used but usage suggests a typed variant. See [Chapter 6 §6.15.4](06-expressions.md#6154-type-inference).

**Warning** — A compile-time diagnostic that does not prevent compilation. Input-related warnings use the `W01xx` range.

---

