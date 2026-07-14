@ Loop examples — Python `for i in range(5)` equivalents in Lang.P

@ --- Repeat N times (like Python: for _ in range(5)) ---
repeat 5 times,
    print "Hello".
..

@ --- Repeat with counter (like Python: for i in range(5)) ---
@ i goes from 0 to 4 (zero-indexed)
repeat 5 times as i,
    print "i = " with i.
..

print "Done with loops!".
