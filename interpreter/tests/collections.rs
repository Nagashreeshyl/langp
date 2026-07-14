use langp_interpreter::run;
use langp_parser::parse;

fn run_source(source: &str) -> langp_runtime::RuntimeResult<()> {
    let program = parse(source).expect("parse");
    run(&program).map(|_| ())
}

#[test]
fn list_methods_and_index() {
    let src = r#"
nums = [1, 2, 3].
print nums[0].
nums[1] = 20.
nums.append(4).
nums.insert(1, 99).
print nums.contains(20).
print nums.length().
nums.sort().
nums.reverse().
print len(nums).
"#;
    run_source(src).expect("run");
}

#[test]
fn dict_access_and_methods() {
    let src = r#"
student = { name : "Naga", age : 18 }.
print student.name.
print student["name"].
student.age = 19.
student["age"] = 20.
print student.contains("name").
keys = student.keys().
print len(keys).
"#;
    run_source(src).expect("run");
}

#[test]
fn set_operations() {
    let src = r#"
a = {1, 2, 3}.
b = {3, 4}.
a.add(5).
print a.contains(2).
u = a.union(b).
print u.length().
"#;
    run_source(src).expect("run");
}

#[test]
fn tuple_is_immutable() {
    let src = r#"
point = (10, 20).
print point[0].
print point[1].
print point.length().
"#;
    run_source(src).expect("run");
}

#[test]
fn typed_list_annotation() {
    let src = r#"
nums: List<Int> = [1, 2, 3].
print nums[0].
"#;
    run_source(src).expect("run");
}

#[test]
fn for_over_collections() {
    let src = r#"
for x in [1, 2, 3],
    print x.
..
d = { a : 1, b : 2 }.
for pair in d.items(),
    print pair.
..
"#;
    run_source(src).expect("run");
}

#[test]
fn tuple_assign_rejected_on_write() {
    let src = r#"
point = (1, 2).
point[0] = 5.
"#;
    assert!(run_source(src).is_err());
}
