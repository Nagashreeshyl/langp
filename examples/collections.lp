@ collections.lp — List, Dict, Set, Tuple examples

numbers: List<Int> = [1, 2, 3].
names = ["Naga", "Alex", "John"].
mixed = [1, "Hello", true].

print numbers[0].
numbers[1] = 20.
numbers.append(4).
numbers.insert(2, 100).
numbers.remove(3).
print numbers.pop().
numbers.clear().
numbers = [3, 1, 2].
numbers.sort().
numbers.reverse().
print numbers.contains(2).
print numbers.length().

for item in numbers,
    print item.
..

student = {
    name : "Naga",
    age : 18,
    college : "DSU"
}.

print student.name.
print student["name"].
student.age = 19.
student["age"] = 20.
print student.keys().
print student.values().
print student.contains("name").
student.remove("college").
student.clear().

student = { name : "Naga", age : 18 }.
for item in student.items(),
    print item.
..

set_a = {1, 2, 3}.
set_b = {3, 4, 5}.
set_a.add(6).
print set_a.contains(2).
print set_a.union(set_b).
print set_a.intersection(set_b).
print set_a.difference(set_b).

point = (10, 20).
print point[0].
print point.length().

print "Collections demo done.".
