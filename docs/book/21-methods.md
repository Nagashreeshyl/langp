# Methods

> **Experimental: Beta** — Instance methods and `self` are Beta in v0.2.0.

## Introduction

A **method** is a function defined inside a type that operates on an **instance**. Methods read and update fields through **`self`**, which refers to the object the method was called on. `user.greet()` runs `greet` with `self` bound to `user`.

**Why methods:** Behavior lives next to the data it uses. `greet()` can access `self.name` without passing `name` as a parameter every time.

**When to add one:** Whenever an operation naturally belongs to an object — printing a greeting, incrementing a counter, formatting output.

---

## Syntax

### Method definition

Methods are `function` blocks inside the type body:

```lp
type User,
    name.

    function greet(),
        print "Hello " with self.name with "!".
    ..
..
```

Methods may take parameters:

```lp
function set_age(age),
    self.age = age.
..
```

`init` is a special method used as the constructor ([20 — Constructors](20-constructors.md)).

### Calling methods

Use dot notation on an **instance**:

```lp
user = User("Naga", 18).
user.greet().
user.set_age(19).
```

### `self`

Inside a method, `self` is the current instance:

| Expression | Meaning |
|------------|---------|
| `self.name` | Read field `name` |
| `self.name = x` | Write field `name` |
| `self.other_method()` | Call another method on same instance |

`self` is only valid inside type methods, not in top-level functions.

---

## Examples

### Simple — method with no parameters

**Learning version:**

```lp
type User,
    name.

    function greet(),
        print "Hello " with self.name.
    ..
..

user = User().
user.name = "Naga".
user.greet().
```

**Professional version:**

```lp
type User,
    name.

    function greet(),
        print "Hello " with self.name.
    ..
..

user = User().
user.name = "Naga".
user.greet().
```

### Intermediate — methods that mutate state

**Learning version:**

```lp
@ Methods can change self's fields.
type Counter,
    value.

    function init(start),
        self.value = start.
    ..

    function increment(),
        self.value = self.value + 1.
    ..

    function show(),
        print self.value.
    ..
..

c = Counter(0).
c.increment().
c.increment().
c.show().
```

**Professional version:**

```lp
type Counter,
    value.

    function init(start),
        self.value = start.
    ..

    function increment(),
        self.value = self.value + 1.
    ..

    function show(),
        print self.value.
    ..
..

c = Counter(0).
c.increment().
c.increment().
c.show().
```

### Advanced — multiple methods cooperating

**Learning version:**

```lp
@ From examples/oop.lp — init + greet + field read from outside.
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
@ email was set inside init; greet uses self.name
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

---

## Common Mistakes

**Mistake:** Calling a method on the type instead of an instance.

```lp
User.greet().    @ WRONG
```

**Fix:**

```lp
user = User("Naga", 18).
user.greet().
```

---

**Mistake:** Using `self` outside a type method.

`self` is only defined inside methods declared in a `type` block.

---

**Mistake:** Expecting `super.method()` for parent calls.

Parent method **override** replaces the child version entirely. Calling the parent implementation via `super` is **not implemented** in v0.2.0. See [22 — Inheritance](22-inheritance.md).

---

## Best Practices

- Name methods with verbs (`greet`, `increment`, `save`) and fields with nouns (`name`, `count`).
- Methods should not surprise callers — prefer clear effects (print, assign) over hidden global state.
- Keep methods small; extract repeated logic into standalone functions when it does not need `self`.
- Use `init` for setup, ordinary methods for behavior after construction.
- Test methods by running `lang run examples/oop.lp`.

---

## Exercises

### Beginner

1. Add `function greet()` to a type that prints `self.name`.
2. Call the method on an instance after setting `name`.
3. Add a method with one parameter that assigns to a field.
4. Fix code that says `greet(user)` instead of `user.greet()`.
5. Identify all methods in `examples/oop.lp`.

### Intermediate

1. Implement `Counter` with `increment`, `decrement`, and `show`.
2. Write two methods where the second reads a field the first set.
3. Add `function describe()` that prints multiple fields with `with`.
4. Run `type_fields_and_methods` from `interpreter/tests/oop.rs`.
5. Explain what `self` refers to when two instances call the same method.

### Advanced

1. Build a `type BankAccount` with `deposit`, `withdraw`, and `balance` methods that guard against negative balance.
2. Compare Lang.P methods to standalone `function greet(user)` — list pros and cons in comments.

---

## Summary

**Methods** are functions inside a `type` that use **`self`** to access the instance. Call them with **`instance.method()`**. `init` is the constructor method; other methods implement object behavior. `super` is not available in v0.2.0.

**Previous:** [20 — Constructors](20-constructors.md) · **Next:** [22 — Inheritance](22-inheritance.md)

**See also:** [19 — Objects](19-objects.md), [20 — Constructors](20-constructors.md), [22 — Inheritance](22-inheritance.md), [examples/oop.lp](../../examples/oop.lp)
