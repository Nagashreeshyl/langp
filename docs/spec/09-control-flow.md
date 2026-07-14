# Chapter 9 — Control Flow

## 9.1 Conditional Statements

### 9.1.1 If

```lp
if marks >= 90,
    print "Grade A".
..
```

### 9.1.2 Otherwise If (Else-If)

Lang.P uses `otherwise if` instead of `else if`:

```lp
if marks >= 90,
    print "Grade A".
otherwise if marks >= 80,
    print "Grade B".
otherwise if marks >= 70,
    print "Grade C".
otherwise,
    print "Grade F".
..
```

`otherwise if` is a two-token keyword. It MUST NOT be written as `else if`.

### 9.1.3 Otherwise (Else)

```lp
if online,
    print "Connected".
otherwise,
    print "Offline".
..
```

### 9.1.4 If Expression

Inline conditional:

```lp
label = if score >= 60, "Pass", otherwise, "Fail".
```

## 9.2 Loops

### 9.2.1 Repeat (Counted Loop)

```lp
repeat 5 times,
    print "Hello".
..
```

The counter variable is optionally accessible:

```lp
repeat 5 times as i,
    print "Iteration " with i.
..
```

`i` ranges from `0` to `4` (zero-indexed).

### 9.2.2 Repeat Forever

```lp
repeat forever,
    process_events().
    if should_stop,
        break.
    .
..
```

Equivalent to `while true` but reads more naturally for event loops.

### 9.2.3 For-In Loop

```lp
for student in students,
    print student.name.
.

for i in 0..10,
    print i.
.

for key, value in dictionary,
    print key with ": " with value.
.

for item in list,
    print item.
.
```

For-in supports:

- Lists, arrays, sets, dictionaries (keys), strings (characters), ranges
- Custom iterables implementing the `Iterable<T>` interface

### 9.2.4 While Loop

```lp
while count > 0,
    print count.
    count -= 1.
..
```

The condition is evaluated before each iteration. If false initially, the body never executes.

### 9.2.5 Loop Control

```lp
break.       @ Exit innermost loop
continue.    @ Skip to next iteration
```

## 9.3 Pattern Matching

Lang.P supports `match` for exhaustive pattern matching (v0.2 full, v0.1 basic):

```lp
match status,
    Status.Active => print "Active".
    Status.Inactive => print "Inactive".
    Status.Pending(reason) => print "Pending: " with reason.
.
```

Basic form in v0.1 uses if-chains; full `match` is planned for v0.2.

## 9.4 Iteration Protocol

Custom types implement iteration:

```lp
interface Iterable<T>,
    function iterator() -> Iterator<T>.
.

interface Iterator<T>,
    function has_next() -> Bool.
    function next() -> T.
.
```

Example:

```lp
type Counter,
    max: Int.
    current: Int = 0.

    function iterator() -> CounterIterator,
        return CounterIterator(counter = self).
    .
.

@ for n in Counter(max = 5) iterates 0..4
```

## 9.4 Guard Clauses

Early exit pattern (convention, not syntax):

```lp
function process(user),
    if user == null,
        return.
    .
    if not user.is_active,
        return.
    .
    @ main logic here
    do_work(user).
.
```

## 9.5 Nested Control Flow

```lp
for row in matrix,
    for cell in row,
        if cell > 0,
            print cell.
        .
    .
..
```

Indentation MUST increase by 4 spaces per nesting level.

## 9.6 Control Flow with Blocks

Every control flow construct that takes a body uses `,` / `..`:

```lp
if condition,
    @ body
..

repeat n times,
    @ body
..

for item in items,
    @ body
..

while condition,
    @ body
..
```

The `,` MUST appear at the end of the header line. The `..` MUST align with the header's indentation level.

## 9.7 Boolean Conditions

Conditions MUST evaluate to `Bool`. The compiler MUST NOT allow implicit truthiness:

```lp
@ Error in Lang.P — no implicit truthiness
if name,
    print name.
.

@ Correct
if name != null and name != "",
    print name.
.
```

This prevents common beginner bugs found in Python and JavaScript.

## 9.8 Switch on Values (v0.2)

Future syntax for simple value dispatch:

```lp
switch day,
    "Monday" => print "Start of week".
    "Friday" => print "Almost weekend".
    otherwise => print "Midweek".
..
```

In v0.1, use if/otherwise if chains or match (when available).
