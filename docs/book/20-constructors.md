# Constructors

> **Experimental: Beta** — The `init` method is the supported constructor pattern in v0.2.0.

## Introduction

A **constructor** prepares a new object so it is ready to use the moment it exists. In Lang.P, construction happens through the special method **`init`**. When you write `User("Naga", 18)`, the runtime creates the instance, then calls `init` with those arguments.

**Why `init`:** One place to enforce required fields, compute derived values (like building an email from a name), and establish invariants before any other method runs.

**When to define one:** Whenever instances should not exist with missing or invalid field values.

---

## Syntax

### `init` method

Define `init` inside the type block like any other method. Use `self` to assign fields:

```lp
type User,
    name.
    age.
    email.

    function init(name, age),
        self.name = name.
        self.age = age.
        self.email = name with "@example.com".
    ..
..
```

### Calling the constructor

**Positional arguments** map to `init` parameters in order:

```lp
user = User("Naga", 18).
```

**No arguments** when `init` has no parameters, or when you use empty `User()` and set fields elsewhere:

```lp
user = User().
```

**Block form** still works alongside or instead of positional args:

```lp
user = User(),
    name = "Naga".
    age = 25.
..
```

Positional args passed to `User(...)` are forwarded to `init` after the instance shell is allocated.

### Default parameter values

The parser accepts default values on `init` parameters (conformance fixture):

```lp
function init(name, age = 0),
    self.name = name.
    self.age = age.
..
```

---

## Examples

### Simple — basic init

**Learning version:**

```lp
@ init runs automatically on User(...).
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..
..

user = User("Alex", 30).
print user.name.
print user.age.
```

**Professional version:**

```lp
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..
..

user = User("Alex", 30).
print user.name.
print user.age.
```

### Intermediate — derived fields in init

**Learning version:**

```lp
@ Compute email inside init — callers cannot forget.
type User,
    name.
    age.
    email.

    function init(name, age),
        self.name = name.
        self.age = age.
        self.email = name with "@example.com".
    ..

    function greet(),
        print "Hello " with self.name with "!".
    ..
..

user = User("Naga", 18).
user.greet().
print user.email.
```

**Professional version:**

```lp
type User,
    name.
    age.
    email.

    function init(name, age),
        self.name = name.
        self.age = age.
        self.email = name with "@example.com".
    ..

    function greet(),
        print "Hello " with self.name with "!".
    ..
..

user = User("Naga", 18).
user.greet().
print user.email.
```

### Advanced — init vs manual setup

**Learning version:**

```lp
@ Without init, empty User() leaves fields unset until you assign.
type Config,
    host.
    port.

    function init(host, port),
        self.host = host.
        self.port = port.
    ..

    function url(),
        print "http://" with self.host with ":" with self.port.
    ..
..

cfg = Config("localhost", 8080).
cfg.url().

@ Manual path — no init call
legacy = Config().
legacy.host = "127.0.0.1".
legacy.port = 3000.
legacy.url().
```

**Professional version:**

```lp
type Config,
    host.
    port.

    function init(host, port),
        self.host = host.
        self.port = port.
    ..

    function url(),
        print "http://" with self.host with ":" with self.port.
    ..
..

cfg = Config("localhost", 8080).
cfg.url().

legacy = Config().
legacy.host = "127.0.0.1".
legacy.port = 3000.
legacy.url().
```

---

## Common Mistakes

**Mistake:** Naming the constructor something other than `init`.

Only `function init(...)` is invoked automatically on `Type(...)`. A method named `create` or `new` will not run by itself.

---

**Mistake:** Forgetting `self.` when assigning fields inside `init`.

```lp
function init(name),
    name = name.    @ WRONG — local variable, not field
..
```

**Fix:**

```lp
function init(name),
    self.name = name.
..
```

---

**Mistake:** Passing the wrong number of positional arguments.

Arguments to `User(...)` must match `init`'s parameter count (unless defaults apply). Mismatches cause runtime errors.

---

## Best Practices

- Put **all required setup** in `init` so `User(...)` always yields a valid object.
- Derive computed fields (emails, slugs, display names) inside `init`, not scattered in callers.
- Keep `init` short — delegate complex work to private helper functions if needed.
- Match parameter order to field importance: `(name, age)` reads naturally for `User`.
- Document expected arguments in a comment above the type when non-obvious.

---

## Exercises

### Beginner

1. Add `function init(name, age)` to a `User` type with two fields.
2. Create a user with `User("Test", 21)` and print both fields.
3. Fix an `init` that assigns to `name` instead of `self.name`.
4. Run the `constructor_init` test pattern from `interpreter/tests/oop.rs`.
5. Explain when `User()` without args is still valid.

### Intermediate

1. Build `init` that sets `email` from `name` and `"@example.com"`.
2. Add a third parameter or default for `age = 0` if your parser supports it.
3. Compare block-form `User(), name = ... ..` with positional `User(...)`.
4. Write `init` for a `Point(x, y)` type.
5. Trace execution order: which runs first — field block assignments or `init`?

### Advanced

1. Design a `type Rectangle` with `init(width, height)` and a method `area()` that uses fields set in `init`.
2. Refactor a program that used manual field assignment after `User()` to use `init` only; list lines removed.

---

## Summary

Lang.P constructors are **`function init`** methods inside a type. Call `Type(arg1, arg2)` to create an instance and run `init` automatically. Use `self.field = value` for all field assignments. Derived values belong in `init` so instances start life in a consistent state.

**Previous:** [19 — Objects](19-objects.md) · **Next:** [21 — Methods](21-methods.md)

**See also:** [19 — Objects](19-objects.md), [21 — Methods](21-methods.md), [examples/oop.lp](../../examples/oop.lp), [interpreter/tests/oop.rs](../../interpreter/tests/oop.rs)
