# Chapter 8 — Functions

## 8.1 Function Definition

Functions are defined with the `function` keyword:

```lp
function greet(name),
    print "Hello " with name.
..
```

With return type annotation:

```lp
function add(a: Int, b: Int) -> Int,
    return a + b.
..
```

With default parameters:

```lp
function greet(name, greeting = "Hello"),
    print greeting with " " with name.
..
```

With variadic parameters:

```lp
function log(level, ...messages),
    for msg in messages,
        print "[" with level with "] " with msg.
    ..
..
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
..
```

Implicit return (last expression):

```lp
function square(n: Int) -> Int,
    n * n.    @ v0.2 — implicit return of last expression
..
```

In v0.1, `return` is required for non-void functions.

Multiple return values:

```lp
function divmod(a: Int, b: Int) -> (Int, Int),
    return a // b, a % b.
..

quotient, remainder = divmod(10, 3).
```

## 8.5 Function Overloading

Functions MAY be overloaded by parameter types:

```lp
function process(data: String),
    @ handle string
..

function process(data: List<Int>),
    @ handle list
..
```

The compiler selects the best match at compile time. Ambiguous calls are compile errors.

## 8.6 Generic Functions

```lp
function first<T>(items: List<T>) -> T?,
    if items.is_empty(),
        return null.
    ..
    return items[0].
..
```

## 8.7 Closures

Functions capture their lexical environment:

```lp
function multiplier(factor),
    return function(n),
        return n * factor.
    ..
..

times3 = multiplier(3).
print times3(10).    @ 30
```

Captured variables are shared between closures from the same scope (reference capture for mutable variables).

## 8.8 Higher-Order Functions

Functions are first-class values:

```lp
function apply(fn, value),
    return fn(value).
..

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
    ..
..

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
    ..
..

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
    ..
..

user = User(name = "Naga", age = 25).
@ Or positional:
user = User("Naga", 25).
```

Named-field construction syntax:

```lp
user = User(),
    name = "Naga".
    age = 25.
..
```

## 8.12 Properties

Computed properties use the `property` keyword:

```lp
type Circle,
    radius: Float64.

    property diameter -> Float64,
        return self.radius * 2.
    ..

    property diameter -> Float64 = value,
        self.radius = value / 2.
    ..
..
```

## 8.13 Operator Overloading

Operators are overloaded via special method names:

```lp
type Vector,
    x: Float64.
    y: Float64.

    function __add__(other: Vector) -> Vector,
        return Vector(x = self.x + other.x, y = self.y + other.y).
    ..
..

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
    ..
..

print "hello".reversed().    @ "olleh"
```

Extensions MUST be defined in the same module or an explicitly imported extension module.

## 8.15 Recursion

Recursion is fully supported. The compiler MAY apply tail-call optimization when the recursive call is in tail position:

```lp
function factorial(n: Int) -> Int,
    return factorial_helper(n, 1).
..

function factorial_helper(n: Int, acc: Int) -> Int,
    if n <= 1,
        return acc.
    ..
    return factorial_helper(n - 1, acc * n).    @ TCO eligible
..
```

## 8.16 Function Attributes

Metadata annotations (v0.2):

```lp
@deprecated("Use fetch_v2 instead")
function fetch(url),
    @ ...
..
```

In v0.1, use comments for deprecation notices.

## 8.17 Inline Functions

Performance hint for small functions:

```lp
inline function square(n: Int) -> Int,
    return n * n.
..
```

The compiler MAY inline the function body at call sites. `inline` is a hint, not a guarantee.

## 8.18 Main Function

The program entry point:

```lp
function main(),
    print "Hello, Lang.P!".
..
```

`main` MUST take no parameters and return `Int` (exit code) or `Void`. If it returns `Int`, that value is the process exit code.

```lp
function main() -> Int,
    return 0.
..
```
