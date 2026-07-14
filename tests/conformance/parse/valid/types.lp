@ valid: type definitions and object creation
type User,
    name: String.
    age: Int.

    function init(name, age = 0),
        self.name = name.
        self.age = age.
    ..
..

user = User("Naga", 25).

user = User(),
    name = "Naga".
    age = 25.
..

enum Color,
    Red.
    Green.
    Blue.
..
