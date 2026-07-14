# 08 — Loops

**Status: Implemented (v0.1)**

---

## `repeat N times`

Like Python's `for _ in range(N)`:

### Learning version

```lp
@ Print Hello five times.
repeat 5 times,
    print "Hello".
..
```

---

## `repeat N times as i`

Counter from **0** to **N−1**:

```lp
@ Print index each iteration.
repeat 5 times as i,
    print "i = " with i.
..
```

`i` exists only inside the block.

---

## `repeat forever`

```lp
@ Infinite loop — exit with break.
repeat forever,
    print "Running...".
    break.
..
```

---

## `for item in collection`

```lp
@ Loop over a list.
items = [10, 20, 30].
for item in items,
    print item.
..
```

---

## `while`

```lp
@ Count down.
count = 5.
while count > 0,
    print count.
    count = count - 1.
..
```

---

## `break` and `continue`

```lp
repeat 10 times as i,
    if i == 5,
        continue.
    ..
    if i == 8,
        break.
    ..
    print i.
..
```

---

## Examples by difficulty

**Beginner — fixed repetitions**

```lp
repeat 3 times,
    print "*".
..
```

**Intermediate — counter**

```lp
repeat 5 times as i,
    print "Step " with i.
..
```

**Advanced — nested loops**

```lp
repeat 3 times as row,
    repeat 3 times as col,
        print row with "," with col.
    ..
..
```

---

## Next steps

- [09 — Conditions](09-conditions.md)
- [Control flow (spec)](../spec/09-control-flow.md)
- [examples/loops.lp](../../examples/loops.lp)
