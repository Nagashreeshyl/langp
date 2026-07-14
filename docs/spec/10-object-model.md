# Chapter 10 — Object Model

> **Implementation note (v0.1):** `type`, OOP, inheritance, and interfaces are **specification only**. The v0.1 interpreter does not run object-oriented programs yet. See [12 — Classes (manual)](../manual/12-classes.md).

## 10.1 Type Definition

Lang.P uses `type` instead of `class`:

```lp
type User,
    name: String.
    age: Int.
    email: String? = null.
..
```

Fields are declared with optional type annotations and default values. Fields without defaults MUST be set in the constructor.

## 10.2 Object Creation

### 10.2.1 Positional Construction

```lp
user = User("Naga", 25).
```

### 10.2.2 Named Construction

```lp
user = User(),
    name = "Naga".
    age = 25.
    email = "naga@example.com".
..
```

### 10.2.3 Default Construction

```lp
user = User().
@ Fields with defaults are initialized; others must be set in init()
```

## 10.3 Field Access

```lp
user.name = "Naga".
print user.age.
user.email = "naga@example.com".
```

Fields are accessed via dot notation. Private fields are accessible only within the type and its nested types.

## 10.4 Constructors

The `init` method is the constructor:

```lp
type User,
    name: String.
    age: Int.

    function init(name, age = 0),
        self.name = name.
        self.age = age.
    ..
..
```

Multiple constructors via overloading:

```lp
function init(name),
    self.init(name, 0).
..
```

## 10.5 Inheritance

Single inheritance using `extends`:

```lp
type Animal,
    name: String.

    function speak(),
        print "...".
    ..
..

type Dog extends Animal,
    breed: String.

    function speak(),
        print "Woof!".
    ..
..

dog = Dog(name = "Buddy", breed = "Labrador").
dog.speak().    @ "Woof!"
```

Rules:

- A type MAY extend at most one other type (single inheritance).
- All types implicitly extend `Object` if no parent is specified.
- `super` refers to the parent type's methods and constructor.

```lp
type Dog extends Animal,
    function init(name, breed),
        super.init(name).
        self.breed = breed.
    ..
..
```

## 10.6 Interfaces

Interface definition:

```lp
interface Drawable,
    function draw() -> Void.
    function bounds() -> (Int, Int, Int, Int).
..

interface Serializable,
    function serialize() -> String.
    function deserialize(data: String) -> Void.
..
```

Implementation is implicit (structural):

```lp
type Rectangle,
    x: Int.
    y: Int.
    width: Int.
    height: Int.

    function draw(),
        @ render rectangle
    ..

    function bounds(),
        return (self.x, self.y, self.width, self.height).
    ..
..

@ Rectangle satisfies Drawable structurally — no explicit 'implements' needed
```

Explicit implementation for clarity in public APIs:

```lp
type Rectangle implements Drawable, Serializable,
    @ ...
..
```

## 10.7 Polymorphism

```lp
function render(item: Drawable),
    item.draw().
..

rect = Rectangle(x = 0, y = 0, width = 100, height = 50).
render(rect).
```

Subtype polymorphism: a `Dog` is an `Animal`. Interface polymorphism: any type with matching methods satisfies the interface.

## 10.8 Encapsulation

Visibility modifiers on type members:

```lp
type BankAccount,
    public owner: String.
    private balance: Float64 = 0.0.

    public function deposit(amount: Float64),
        if amount > 0,
            self.balance += amount.
        ..
    ..

    public function get_balance() -> Float64,
        return self.balance.
    ..
..
```

| Modifier | Accessible from |
|----------|-----------------|
| `public` | Anywhere |
| `internal` | Same package |
| `private` | Same type (and nested types) |

## 10.9 Static Members

```lp
type Config,
    static instance: Config? = null.

    static function get() -> Config,
        if self.instance == null,
            self.instance = Config().
        ..
        return self.instance.
    ..
..
```

Static fields and methods belong to the type, not instances. Access via `TypeName.member`.

## 10.10 Properties

Computed fields with getter and/or setter:

```lp
type Temperature,
    _celsius: Float64 = 0.0.

    property celsius -> Float64,
        return self._celsius.
    ..

    property celsius -> Float64 = value,
        self._celsius = value.
    ..

    property fahrenheit -> Float64,
        return self._celsius * 9.0 / 5.0 + 32.0.
    ..

    property fahrenheit -> Float64 = value,
        self._celsius = (value - 32.0) * 5.0 / 9.0.
    ..
..
```

## 10.11 Abstract Types

Cannot be instantiated directly; must be extended:

```lp
abstract type Shape,
    abstract function area() -> Float64.
    abstract function perimeter() -> Float64.
..

type Circle extends Shape,
    radius: Float64.

    function area() -> Float64,
        return 3.14159 * self.radius ** 2.
    ..

    function perimeter() -> Float64,
        return 2.0 * 3.14159 * self.radius.
    ..
..
```

## 10.12 Generics

```lp
type Box<T>,
    value: T.

    function get() -> T,
        return self.value.
    ..

    function set(value: T),
        self.value = value.
    ..
..

int_box = Box<Int>(value = 42).
str_box = Box<String>(value = "hello").
```

Generic constraints:

```lp
type SortedList<T: Comparable>,
    items: List<T> = [].

    function add(item: T),
        self.items.add(item).
        self.items.sort().
    ..
..
```

## 10.13 Operator Overloading

See [Chapter 8 §8.13](08-functions.md#813-operator-overloading).

## 10.14 Extensions

See [Chapter 8 §8.14](08-functions.md#814-extensions).

## 10.15 Object Identity

Every object has a unique identity distinct from its value:

```lp
a = User(name = "Naga").
b = User(name = "Naga").
a is b.     @ false — different objects
a == b.     @ depends on __eq__ implementation
```

Default equality is identity-based. Types SHOULD override `__eq__` for value equality.

## 10.16 Destructors

Cleanup method (called by GC before collection):

```lp
type FileHandle,
    path: String.
    _handle: NativeHandle.

    function destroy(),
        close_native(self._handle).
    ..
..
```

Deterministic cleanup uses `try`/`finally` or the `using` statement (v0.2).

## 10.17 Enums as Algebraic Data Types

```lp
enum Result<T, E>,
    Ok(value: T).
    Err(error: E).
..

enum Option<T>,
    Some(value: T).
    None.
..

result = Result.Ok(42).
if result is Result.Ok,
    print result.value.
..
```

## 10.18 The Root Object Type

All types inherit from `Object`:

```lp
@ Built-in
type Object,
    function to_string() -> String.
    function hash() -> Int.
    function equals(other: Object) -> Bool.
..
```

Every value can be converted to `Object`.
