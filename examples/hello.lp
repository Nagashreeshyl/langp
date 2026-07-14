@ hello.lp — The canonical Lang.P hello world program.

@ Define a function that greets someone by name.
function greet(name),
    print "Hello " with name with "!".
.

@ Call the function with a name.
greet("World").

@ Loop 5 times and greet "Naga" each time.
@ Note: loop variable `i` only exists inside the repeat block.
repeat 5 times as i,
    greet("Naga").
..

print "Loop ran successfully!".
i = 5.
while i > 0,
    print "Hello".
    i = i - 1.
..
print "Loop ran successfully!".