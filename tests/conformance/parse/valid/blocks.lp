@ valid: block structures with comma/dotdot
if age >= 18,
    print "Adult".
otherwise if age >= 13,
    print "Teen".
otherwise,
    print "Child".
..

repeat 5 times,
    print "Hello".
..

repeat 5 times as i,
    print "Iteration " with i.
..

repeat forever,
    if done,
        break.
    ..
..

for item in items,
    print item.
..

for key, value in config,
    print key with ": " with value.
..

while count > 0,
    count -= 1.
..
