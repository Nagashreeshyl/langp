@ hello.lp — The canonical Lang.P hello world program.

@ Define a function that greets someone by name.
function greet(name),
    print "Hello " with name with "!".
..

@ Call the function with a name.
greet("World").

@ Loop 5 times (like Python: for _ in range(5))
repeat 5 times,
    greet("Naga").
..

@ Loop with counter (like Python: for i in range(5))
repeat 5 times as i,
    print "i = " with i.
..

print "Done!".
