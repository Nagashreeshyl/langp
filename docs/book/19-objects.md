# Objects

> **Experimental: Beta** — Fields, instances, and `Type()` construction are Beta in v0.2.0.

## Introduction

An **object** ( **instance** ) is a value created from a **type** definition. The type is the blueprint; the object is one concrete copy with its own field values. `User("Naga", 18)` produces a `User` instance whose `name` is `"Naga"` and whose `age` is `18`.

**Why instances:** The same type can create many objects, each with different data but the same shape and methods.

**When to create one:** After defining a `type`, call the type name like a function — `User()` or `User(args)` — to allocate an instance.

---

## Syntax

### Field declarations

Fields are listed at the top of the type body. Each field ends with `.`:

```lp
type User,
    name.
    age.
    email.
..
```

With optional type hints:

```lp
type User,
    name: String.
    age: Int.
..
```

### Instance creation

| Form | Meaning |
|------|---------|
| `User()` | Empty instance; set fields manually |
| `User("Naga", 18)` | Positional args map to fields / `init` |
| Block form | Field assignments in a trailing block |

**Empty constructor:**

```lp
user = User().
user.name = "Naga".
user.age = 18.
```

**Positional constructor** (requires `init` — see [20 — Constructors](20-constructors.md)):

```lp
user = User("Naga", 18).
```

**Block constructor:**

```lp
user = User(),
    name = "Naga".
    age = 25.
..
```

### Field access

```lp
print user.name.
user.email = "naga@example.com".
```

Use dot notation on the instance variable, not on the type name (except when calling `Type()` to construct).

---

## Examples

### Simple — empty instance

**Learning version:**

```lp
@ Create, then fill in fields.
type User,
    name.
    age.
..

user = User().
user.name = "Naga".
user.age = 18.
print user.name.
```

**Professional version:**

```lp
type User,
    name.
    age.
..

user = User().
user.name = "Naga".
user.age = 18.
print user.name.
```

### Intermediate — block initialization

**Learning version:**

```lp
@ Block form sets fields at construction time.
type Point,
    x.
    y.
..

origin = Point(),
    x = 0.
    y = 0.
..

print origin.x with ", " with origin.y.
```

**Professional version:**

```lp
type Point,
    x.
    y.
..

origin = Point(),
    x = 0.
    y = 0.
..

print origin.x with ", " with origin.y.
```

### Advanced — multiple independent instances

**Learning version:**

```lp
@ Each instance has its own field storage.
type Account,
    owner.
    balance.

    function show(),
        print self.owner with ": " with self.balance.
    ..
..

a = Account(),
    owner = "Alice".
    balance = 100.
..

b = Account(),
    owner = "Bob".
    balance = 250.
..

a.show().
b.show().
a.balance = 50.
a.show().
@ b.balance is still 250
```

**Professional version:**

```lp
type Account,
    owner.
    balance.

    function show(),
        print self.owner with ": " with self.balance.
    ..
..

a = Account(),
    owner = "Alice".
    balance = 100.
..

b = Account(),
    owner = "Bob".
    balance = 250.
..
a.show().
b.show().
a.balance = 50.
a.show().
```

---

## Common Mistakes

**Mistake:** Accessing fields on the type instead of the instance.

```lp
type User,
    name.
..

@ WRONG — User is a type, not an instance
print User.name.
```

**Fix:**

```lp
user = User().
user.name = "Naga".
print user.name.
```

---

**Mistake:** Assuming fields auto-initialize to zero or empty string.

Unassigned fields are absent until you set them. Always assign in `init`, block construction, or immediately after `User()`.

---

**Mistake:** Confusing objects with dictionaries.

Both use dot access, but objects come from `type` definitions and support methods with `self`. Dictionaries use `{key: value}` literals ([15 — Dictionaries](15-dictionaries.md)).

---

## Best Practices

- Declare every field the instance needs inside the `type` block — even if `init` sets them later.
- Prefer `User("Naga", 18)` or block form over manual assignment scattered across many lines.
- Name instances with lowercase (`user`, `admin`); reserve PascalCase for type names (`User`).
- One instance per logical entity — do not reuse one object variable for two different people.
- Test with `lang run examples/oop.lp` after changes.

---

## Exercises

### Beginner

1. Define `type Pet` with `name` and `species`; create with `Pet()` and assign fields.
2. Print both fields in one `print` using `with`.
3. Create two pets and show they are independent.
4. Use block form `Pet(), name = "...", species = "..." ..` once.
5. Identify fields vs methods in `examples/oop.lp`.

### Intermediate

1. Add a third field `email` to a User type and set it after construction.
2. Write block-form construction for three different users.
3. Compare object field access to dictionary dot access in comments.
4. Run the `type_fields_and_methods` pattern from `interpreter/tests/oop.rs` locally.
5. List which fields `User` has in `examples/oop.lp`.

### Advanced

1. Model a `type Product` with `sku`, `title`, `price`; create a "catalog" of three instances and print each title.
2. Explain why Lang.P objects are Beta — read STATUS.md and summarize in three sentences.

---

## Summary

**Objects** are instances of a **type**, created with `Type()`, `Type(args)`, or block initialization. **Fields** are declared in the type body and accessed with dot notation on the instance. Each object holds its own field values.

**Previous:** [18 — Types and OOP](18-type-oop.md) · **Next:** [20 — Constructors](20-constructors.md)

**See also:** [20 — Constructors](20-constructors.md), [21 — Methods](21-methods.md), [examples/oop.lp](../../examples/oop.lp), [interpreter/tests/oop.rs](../../interpreter/tests/oop.rs)
