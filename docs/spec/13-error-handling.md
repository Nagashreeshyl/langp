# Chapter 13 — Error Handling

> **Implementation note (v0.1):** Basic `try` / `catch` / `finally` / `throw` work in the interpreter. Advanced error types and propagation operators are specification. See [14 — Error Handling (manual)](../manual/14-error-handling.md).

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
    ..
..
```

## 13.3 Throwing Errors

Raise errors with `throw`:

```lp
function divide(a, b),
    if b == 0,
        throw DivisionError("Cannot divide by zero").
    ..
    return a / b.
..
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
    ..
..

throw ValidationError(field = "email", message = "Invalid email format").
```

## 13.5 Error Properties

All errors implement:

```lp
interface Error,
    property message -> String.
    property cause -> Error?.
    property stack_trace -> StackTrace.
..
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
..

function parse_number(text: String) -> Result<Float64, ParseError>,
    try,
        return Result.Ok(parse_float(text)).
    catch error: ParseError,
        return Result.Err(error).
    ..
..
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
..
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
..
```

In v0.1, use try/finally.

## 13.12 Error Handling Best Practices

1. Catch specific error types before general ones.
2. Always clean up resources in `finally`.
3. Never silently swallow errors — log or re-throw.
4. Include context in error messages: `"Failed to load config from " with path`.
5. Use Result for API boundaries where failure is expected.
