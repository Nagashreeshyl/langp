# Static Members

> **Experimental: Beta** — Static field syntax is **parsed** in v0.2.0, but runtime support is **partial**. Read the limitations below before relying on static members in production code.

## Introduction

**Static members** belong to a **type** itself, not to any single instance. A classic use is a shared counter: every `Order` instance might increment `Order.count`, so you can ask how many orders were created without holding a specific order object.

**Why static members:** Shared state or utility behavior that does not need an instance — factory counters, configuration flags, or helper functions scoped to a type.

**When to use them:** Sparingly, when the data truly belongs to the type as a whole. In v0.2.0, prefer module-level variables or a dedicated manager instance until static runtime support matures.

**Honest status in v0.2.0:**

| Feature | Status |
|---------|--------|
| `static` prefix on fields (parser) | ✅ Parsed |
| `static_fields` map on types (runtime) | ✅ Present in `TypeRegistry` |
| Default values for static fields | ❌ Not initialized into `static_fields` yet |
| `TypeName.field` read access | ⚠️ Works only if the field was stored at runtime |
| Static methods (`static function`) | ❌ Not implemented |
| `public` / `private` / `internal` on static fields | Parsed only — **not enforced** |

See [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md) and [STATUS.md](../../STATUS.md).

---

## Syntax

### Static field declaration

Declare a static field with the `static` prefix inside a type body:

```lp
type Counter,
    static count.
..
```

Combined with visibility modifiers (parsed, not enforced):

```lp
type Counter,
    public static count.
    private static secret.
..
```

Static fields are **not** included in instance construction — `Counter()` does not allocate per-instance copies of `count`.

### Access via the type name

When runtime support is complete, access shared data on the type:

```lp
Counter.count = Counter.count + 1.
print Counter.count.
```

The interpreter resolves `Counter` as a `LangType` value and looks up the member in `static_fields` (see `interpreter/src/oop.rs` and `interpreter/src/objects.rs`).

### What works today vs. what does not

| Syntax | v0.2.0 |
|--------|--------|
| `static count.` inside `type` | Parses ✅ |
| `TypeName.count` | Lookup exists; values may be empty ⚠️ |
| `instance.count` for static field | Not the intended pattern |
| `static function name(),` | Not implemented ❌ |
| `public static` enforcement | Not enforced ❌ |

For working shared behavior today, use a module ([24 — Modules](24-modules.md)) or a plain variable at program scope ([05 — Variables](05-variables.md)).

---

## Examples

### Simple — declare a static counter (syntax only)

**Learning version:**

```lp
@ Static field syntax — runtime init is partial in v0.2.0.
type Counter,
    static count.
..

@ Instance fields still work normally.
c = Counter().
print "Instance created.".
```

**Professional version:**

```lp
type Counter,
    static count.
..

c = Counter().
```

If `Counter.count` returns an error at runtime, that reflects the partial `static_fields` initialization — not a mistake in your syntax.

### Intermediate — type-level vs instance-level

**Learning version:**

```lp
@ Compare instance field (works) with static field (partial).
type Widget,
    static total.
    id.

    function init(id),
        self.id = id.
    ..
..

w = Widget(1).
print w.id.
@ Widget.total may not be initialized yet — see limitations above.
```

**Professional version:**

```lp
type Widget,
    static total.
    id.

    function init(id),
        self.id = id.
    ..
..

w = Widget(1).
```

### Advanced — workaround with a module

**Learning version:**

```lp
@ Reliable shared counter until static runtime is complete.
use math.

function bump(ref counter),
    counter = counter + 1.
..

global_count = 0.
global_count = global_count + 1.
print "Count: " with global_count.
print "abs(-3): " with math.abs(-3).
```

**Professional version:**

```lp
global_count = 0.
global_count = global_count + 1.
print global_count.
```

Use [24 — Modules](24-modules.md) when shared utilities should live outside a type.

---

## Common Mistakes

**Mistake:** Expecting `static function` to work

```lp
type Util,
    static function clamp(x),
        @ not implemented
    ..
..
```

**Fix:** Use a top-level `function` or a module export until static methods ship.

---

**Mistake:** Assuming `public static` hides a field from other types

```lp
type Secret,
    private static key.
..
@ Another type can still reference Secret.key — visibility is not enforced.
```

**Fix:** Treat visibility keywords as documentation only in v0.2.0. Do not rely on them for security.

---

**Mistake:** Accessing static members on an instance

```lp
c = Counter().
print c.count.    @ wrong pattern — use Counter.count when supported
```

**Fix:** Use `TypeName.field` for static data, `instance.field` for instance data ([19 — Objects](19-objects.md)).

---

## Best Practices

- Treat static members as **experimental** — test with `lang run` before committing to the pattern.
- Prefer module-level functions and variables for shared state until static initialization lands.
- Keep static field names in `snake_case` like instance fields ([32 — Best Practices](32-best-practices.md)).
- Document why shared state lives on the type vs. in a module.
- Re-read [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md) when upgrading Lang.P versions.

---

## Exercises

### Beginner

1. Declare a type `Ticket` with a static field `serial` and one instance field `seat`.
2. Create `Ticket()` and set `seat` on the instance; print it.
3. Write what `TypeName.field` means in one sentence.
4. List two limitations of static members in v0.2.0 from this chapter.
5. Run [examples/oop.lp](../../examples/oop.lp) to confirm instance OOP still works.

### Intermediate

1. Declare `type Stats, static wins. static losses. ..` and explain which parts parse vs. run.
2. Write a program-level counter variable that replaces a static field for a `Game` type.
3. Compare static fields to fields on a single shared instance object — when is each better?
4. Add `public static` to a field and note that it does not change runtime behavior today.
5. Read `interpreter/src/oop.rs` — find where `static ` is stripped from field names.

### Advanced

1. Design a singleton pattern using a module instead of `static function get()` (spec-only today).
2. Write a short note on what must change in the interpreter before `Counter.count = 1` works reliably.

---

## Summary

Static members express **type-level** data with the `static` prefix on fields. The parser and type registry recognize them, but v0.2.0 **partially** implements runtime storage and access. Visibility modifiers are parsed but not enforced. Use modules or program variables as a reliable alternative until static support is complete.

**Previous:** [22 — Inheritance](22-inheritance.md) · **Next:** [24 — Modules](24-modules.md)

**See also:** [18 — Types and OOP](18-type-oop.md), [19 — Objects](19-objects.md), [24 — Modules](24-modules.md), [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md), [Object model (spec)](../spec/10-object-model.md)
