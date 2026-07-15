@ OOP example — types, fields, methods, constructors

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

type Greeter extends User,

    function greet(),
        print "Welcome " with self.name with "!".
    ..
..

admin = Greeter().
admin.name = "Admin".
admin.greet().
