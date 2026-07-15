# 12 — Classes (`type`)

**Status: Beta (v0.2)**

---

> OOP is **implemented in v0.2** with fields, methods, constructors, and single inheritance. Interfaces, properties, and visibility enforcement are planned. See [STATUS.md](../../STATUS.md).

---

## Defining a type

Types use `type`, open with `,`, and close with `..`:

```lp
type User,
    name.
    age.
..
```

Typed fields are also supported: `name: String.`

---

## Creating instances

```lp
user = User().
user.name = "Naga".
user.age = 18.
```

Constructor with `init`:

```lp
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..
..

user = User("Naga", 18).
```

---

## Methods

```lp
type Counter,
    value.

    function increment(),
        self.value = self.value + 1.
    ..
..

c = Counter().
c.increment().
```

---

## Inheritance

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

---

## Example

See [examples/oop.lp](../../examples/oop.lp).

---

## Next steps

- [13 — Modules](13-modules.md)
- [Object model (spec)](../spec/10-object-model.md)
- [Known limitations](../../KNOWN_LIMITATIONS.md)
