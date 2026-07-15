# Inheritance

> **Experimental: Beta** — `extends` and method override are Beta in v0.2.0. There is no `super` keyword yet.

## Introduction

**Inheritance** lets one type reuse and extend another. A `Dog` type can **extend** `Animal`, inheriting fields like `name` and methods like `speak`, then **override** `speak` to print `"Woof"` instead of `"..."`.

**Why inheritance:** Shared structure without duplicating fields and methods. Several types can start from a common base.

**When to use it:** Clear "is-a" relationships — a `Greeter` **is a** `User` with a different greeting. Prefer composition (fields holding other objects) when the relationship is "has-a".

**Not implemented:** `interface`, `super`, and property inheritance. Only single inheritance via `extends` is supported.

---

## Syntax

### Declaring extension

```lp
type Animal,
    name.

    function speak(),
        print "...".
    ..
..

type Dog extends Animal,
    breed.

    function speak(),
        print "Woof".
    ..
..
```

- Child header: `type Child extends Parent,`
- Parent type must be defined (or at least registered) before the child is merged at runtime
- Child may add new fields and methods
- Child may override parent methods by declaring the same method name

### Using inherited fields

```lp
dog = Dog().
dog.name = "Buddy".      @ field from Animal
dog.breed = "Lab".       @ field from Dog
dog.speak().             @ calls Dog's speak(), not Animal's
```

### Override pattern from examples

```lp
type User,
    name.
    @ ... init, greet, etc.
..

type Greeter extends User,

    function greet(),
        print "Welcome " with self.name with "!".
    ..
..

admin = Greeter().
admin.name = "Admin".
admin.greet().
```

`Greeter` inherits `User`'s fields and `init`; `greet` is overridden.

---

## Examples

### Simple — override speak()

**Learning version:**

```lp
@ Dog replaces Animal.speak with its own version.
type Animal,
    name.

    function speak(),
        print "...".
    ..
..

type Dog extends Animal,
    breed.

    function speak(),
        print "Woof".
    ..
..

dog = Dog().
dog.name = "Buddy".
dog.breed = "Lab".
dog.speak().
```

**Professional version:**

```lp
type Animal,
    name.

    function speak(),
        print "...".
    ..
..

type Dog extends Animal,
    breed.

    function speak(),
        print "Woof".
    ..
..

dog = Dog().
dog.name = "Buddy".
dog.breed = "Lab".
dog.speak().
```

### Intermediate — Greeter extends User

**Learning version:**

```lp
@ From examples/oop.lp — override greet only.
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..

    function greet(),
        print "Hello " with self.name with "!".
    ..
..

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
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..

    function greet(),
        print "Hello " with self.name with "!".
    ..
..

type Greeter extends User,

    function greet(),
        print "Welcome " with self.name with "!".
    ..
..

admin = Greeter().
admin.name = "Admin".
admin.greet().
```

### Advanced — child adds fields and methods

**Learning version:**

```lp
@ Child keeps parent fields and adds new ones.
type Vehicle,
    wheels.

    function describe(),
        print "Wheels: " with self.wheels.
    ..
..

type Car extends Vehicle,
    brand.

    function describe(),
        print self.brand with " car, " with self.wheels with " wheels".
    ..
..

c = Car().
c.wheels = 4.
c.brand = "Lang".
c.describe().
```

**Professional version:**

```lp
type Vehicle,
    wheels.

    function describe(),
        print "Wheels: " with self.wheels.
    ..
..

type Car extends Vehicle,
    brand.

    function describe(),
        print self.brand with " car, " with self.wheels with " wheels".
    ..
..

c = Car().
c.wheels = 4.
c.brand = "Lang".
c.describe().
```

---

## Common Mistakes

**Mistake:** Expecting `super.speak()` to call the parent method.

Lang.P v0.2.0 does **not** implement `super`. Overriding replaces the parent method entirely.

---

**Mistake:** Extending a type that does not exist.

```lp
type Puppy extends Unknown,    @ runtime error: unknown parent
    name.
..
```

**Fix:** Define the parent type first in the same file.

---

**Mistake:** Assuming multiple inheritance.

Only one parent per `extends` clause is supported.

---

## Best Practices

- Override sparingly — change behavior only when the child truly **is-a** specialized version of the parent.
- Keep parent types minimal — shared fields and hooks children can override.
- When you need parent behavior **and** child behavior in one method, duplicate the shared logic or call a standalone function until `super` exists.
- Test inheritance with patterns from `interpreter/tests/oop.rs` (`inheritance_extends`).
- Document override intent in a comment above the child method.

---

## Exercises

### Beginner

1. Copy the `Animal` / `Dog` example and change `speak` to print a breed-specific message.
2. Set `name` on a `Dog` instance and print it — inherited field.
3. Run `lang run examples/oop.lp` and note how `Greeter.greet` differs from `User.greet`.
4. Define a parent with one field; extend it with one new field.
5. List features **not** available for inheritance (interfaces, super).

### Intermediate

1. Implement `Greeter extends User` with full `init` inherited from `User`.
2. Override one method but keep using a non-overridden parent method on the same instance.
3. Create `Cat extends Animal` with a different `speak` output.
4. Run the `inheritance_extends` test locally.
5. Explain when **not** to use inheritance (has-a relationships).

### Advanced

1. Build a three-level chain `Living → Animal → Dog` with one overridden method at each level if supported; otherwise two levels with documented limits.
2. Refactor duplicated field lists into a base type and measure lines saved.

---

## Summary

**Inheritance** uses **`type Child extends Parent`** to reuse fields and methods. Child types may **override** methods by redeclaring them — as in `Dog.speak()` or `Greeter.greet()`. Parent fields remain accessible on child instances. **`super` and interfaces are not implemented** in v0.2.0.

**Previous:** [21 — Methods](21-methods.md) · **Next:** [23 — Static Members](23-static-members.md)

**See also:** [18 — Types and OOP](18-type-oop.md), [21 — Methods](21-methods.md), [examples/oop.lp](../../examples/oop.lp), [interpreter/tests/oop.rs](../../interpreter/tests/oop.rs), [STATUS.md](../../STATUS.md)
