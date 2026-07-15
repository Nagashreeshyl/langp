# Types and OOP

> **Experimental: Beta** — Object-oriented features in Lang.P v0.2.0 are usable but incomplete. APIs may change in future releases. See [STATUS.md](../../STATUS.md).

## Introduction

**Object-oriented programming (OOP)** organizes code around **types** (blueprints) and **instances** (objects created from those blueprints). A `User` type might declare fields like `name` and `age`, plus methods like `greet`. Each `User("Naga", 18)` instance carries its own field values.

**Why OOP in Lang.P:** When several functions operate on the same data (name, age, email), grouping fields and behavior in one `type` block keeps programs readable. You call `user.greet()` instead of passing `user` into a standalone `greet(user)` function everywhere.

**When to use it:** Models with identity and behavior — users, animals, game entities, configuration objects. For simple scripts, plain variables and functions ([11 — Functions](11-functions.md)) may be enough.

**What is implemented today:**

| Feature | Status |
|---------|--------|
| `type` declarations | Beta |
| Fields | Beta |
| `function init` constructor | Beta |
| Instance methods with `self` | Beta |
| `extends` inheritance | Beta |
| **Interfaces** | Not implemented (planned) |
| **Properties** | Not implemented (planned) |

---

## Syntax

### Type declaration

```lp
type User,
    name.
    age.
    email.

    function greet(),
        print "Hello " with self.name with "!".
    ..
..
```

- Header: `type Name,` optionally `type Child extends Parent,`
- Members are indented inside the type block
- Type block closes with `..`
- Fields end with `.`; methods are `function` blocks ending with `..`

### Optional field types

```lp
type User,
    name: String.
    age: Int.
..
```

Type annotations on fields are parsed for documentation and future checking; runtime behavior uses dynamic field storage in v0.2.0.

### Creating instances

```lp
user = User().
user = User("Naga", 18).
```

See [19 — Objects](19-objects.md) and [20 — Constructors](20-constructors.md).

### What is NOT available

```lp
@ NOT in v0.2.0:
@ interface Drawable, ...
@ property name, ...
@ super.greet()
```

---

## Examples

### Simple — type with one method

**Learning version:**

```lp
@ A type groups data and behavior.
type Greeter,
    name.

    function hello(),
        print "Hi " with self.name with "!".
    ..
..

g = Greeter().
g.name = "World".
g.hello().
```

**Professional version:**

```lp
type Greeter,
    name.

    function hello(),
        print "Hi " with self.name with "!".
    ..
..

g = Greeter().
g.name = "World".
g.hello().
```

### Intermediate — full User type

**Learning version:**

```lp
@ Matches examples/oop.lp
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

### Advanced — inheritance preview

**Learning version:**

```lp
@ Child types override parent methods.
type Greeter extends User,

    function greet(),
        print "Welcome " with self.name with "!".
    ..
..

admin = Greeter().
admin.name = "Admin".
admin.greet().
```

**Professional version:**

```lp
type Greeter extends User,

    function greet(),
        print "Welcome " with self.name with "!".
    ..
..

admin = Greeter().
admin.name = "Admin".
admin.greet().
```

Full inheritance details: [22 — Inheritance](22-inheritance.md).

---

## Common Mistakes

**Mistake:** Expecting interfaces or property syntax from other languages.

Lang.P v0.2.0 has no `interface` keyword and no `property` blocks. Use fields and methods only.

---

**Mistake:** Forgetting the trailing `..` on the type block.

```lp
type User,
    name.
@ WRONG — missing closing ..
```

**Fix:**

```lp
type User,
    name.
..
```

---

**Mistake:** Treating Beta OOP as frozen forever.

Check [STATUS.md](../../STATUS.md) and release notes when upgrading. Behavior may tighten as the semantic analyzer grows.

---

## Best Practices

- Start with plain functions; introduce `type` when several functions share the same field set.
- Keep types focused — one clear responsibility per type.
- Use `init` for required setup; avoid leaving instances half-initialized ([20 — Constructors](20-constructors.md)).
- Run `lang run examples/oop.lp` and read `interpreter/tests/oop.rs` for canonical patterns.
- Do not depend on unimplemented features (interfaces, properties, `super`) in production code yet.

---

## Exercises

### Beginner

1. Define a `type Book` with fields `title` and `pages`; create an instance and print a field.
2. Add a method `describe()` that prints the title using `self`.
3. Close your type block correctly with `..`.
4. Run `examples/oop.lp` and trace which line creates the `User` instance.
5. List two OOP features that are **planned** but not implemented.

### Intermediate

1. Write a `type Counter` with field `count` and methods `increment()` and `show()`.
2. Add `function init(start)` that sets `self.count`.
3. Create two instances and show they have independent `count` values.
4. Compare your type to an equivalent dictionary `{count: 0}` — when is the type better?
5. Read [STATUS.md](../../STATUS.md) and note the OOP stability label.

### Advanced

1. Build `type Animal` and `type Dog extends Animal` with overridden `speak()` (see [22 — Inheritance](22-inheritance.md)).
2. Document in comments what would change if Lang.P added interfaces — without using syntax that does not exist yet.

---

## Summary

Lang.P **Beta** OOP centers on `type` declarations with **fields**, **`init` constructors**, **methods** using `self`, and **`extends` inheritance**. Interfaces and properties are not implemented in v0.2.0. Use OOP when data and behavior belong together; otherwise prefer functions and collections.

**Previous:** [17 — Tuples](17-tuples.md) · **Next:** [19 — Objects](19-objects.md)

**See also:** [19 — Objects](19-objects.md), [20 — Constructors](20-constructors.md), [21 — Methods](21-methods.md), [22 — Inheritance](22-inheritance.md), [examples/oop.lp](../../examples/oop.lp), [STATUS.md](../../STATUS.md)
