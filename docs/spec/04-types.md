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
