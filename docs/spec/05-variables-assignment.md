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
..
@ print local.    @ Error: local not in scope
```

### 5.4.2 Function Scope

Parameters and local variables are scoped to the function body:

```lp
function process(data),
    temp = transform(data).
    return temp.
..
```

### 5.4.3 Module Scope

Top-level variables are visible throughout the module:

```lp
CONFIG = load_config().

function run(),
    print CONFIG.host.
..
```

### 5.4.4 Closure Capture

Inner functions capture outer variables by reference (for mutable variables) or by value (for immutable `let`/`const`):

```lp
function make_counter(),
    count = 0.
    return function(),
        count += 1.
        return count.
    ..
..

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
..
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
..
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
