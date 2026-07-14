# 07 — Functions

**Status: Implemented (v0.1)**

---

## Defining functions

A function opens its body with `,` and closes with `..` (Grammar Freeze v1.0):

### Learning version

```lp
@ Greet someone by name.
function greet(name),
    print "Hello " with name with "!".
..

@ Call the function.
greet("Naga").
```

### Professional version

```lp
function greet(name),
    print "Hello " with name with "!".
..

greet("Naga").
```

---

## Parameters

Multiple parameters are comma-separated:

```lp
function add(a, b),
    print (a + b).
..

add(3, 5).
```

---

## Return values

Use `return` to send a value back (when the expression is supported in your program):

```lp
function double(n),
    return n * 2.
..

@ result = double(4).
```

---

## Recursion

### Learning version

```lp
@ Factorial using recursion.
function factorial(n),
    if n <= 1,
        return 1.
    otherwise,
        return n * factorial(n - 1).
    ..
..

print factorial(5).
```

---

## Function calls

Always use parentheses:

```lp
greet("World").
add(10, 20).
```

---

## Nested control flow inside functions

Control-flow blocks inside a function still close with `..`:

```lp
function classify(age),
    if age >= 18,
        print "Adult".
    otherwise,
        print "Minor".
    ..
..

classify(20).
```

The outer function block closes with `..` (same rule as every other block).

---

## Next steps

- [08 — Loops](08-loops.md)
- [Functions (spec)](../spec/08-functions.md)
