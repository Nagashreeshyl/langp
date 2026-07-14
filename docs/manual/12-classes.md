# 12 — Classes (`type`)

**Status: Specification — syntax defined; full OOP runtime pending**

---

> This chapter documents the **official language design**. The v0.1 interpreter does not yet run `type` declarations or method dispatch. See [Language Reference v0.1](../guides/LANGUAGE-REFERENCE.md) for what works today.

---

## Defining a type

Types use `type`, open with `,`, and close with `..`:

### Learning version

```lp
@ A simple user record.
type User,
    name.
    age.
..

@ Create an instance.
user = User().
print user.name.
```

User's field syntax (no types on fields in simple form):

```lp
type User,
    name.
    age.
..
```

The specification also allows typed fields: `name: String.`

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

## Properties and constructors

Constructor syntax uses the type name as a call:

```lp
user = User().
```

Advanced features (inheritance, interfaces, `extends`) are in [Object model (spec)](../spec/10-object-model.md).

---

## Do not confuse with v0.1

Until the runtime implements OOP, use dictionaries or plain variables:

```lp
@ v0.1 pattern — use a dict instead of type.
user = {"name": "Naga", "age": 18}.
print user["name"].
```

---

## Next steps

- [13 — Modules](13-modules.md)
- [Object model (spec)](../spec/10-object-model.md)
