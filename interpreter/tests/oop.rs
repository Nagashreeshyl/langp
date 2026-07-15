use langp_interpreter::run;
use langp_parser::parse;

fn run_source(source: &str) -> Result<(), String> {
    let program = parse(source).map_err(|e| e.to_string())?;
    run(&program).map_err(|e| e.to_string())?;
    Ok(())
}

#[test]
fn type_fields_and_methods() {
    let src = r#"
type User,
    name.
    age.

    function greet(),
        print "Hello " with self.name.
    ..
..

user = User().
user.name = "Naga".
user.age = 18.
user.greet().
"#;
    run_source(src).expect("oop program should run");
}

#[test]
fn constructor_init() {
    let src = r#"
type User,
    name.
    age.

    function init(name, age),
        self.name = name.
        self.age = age.
    ..
..

user = User("Alex", 30).
"#;
    run_source(src).expect("init should run");
}

#[test]
fn inheritance_extends() {
    let src = r#"
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
"#;
    run_source(src).expect("inheritance should run");
}
