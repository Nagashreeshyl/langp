# Complete Projects

## Introduction

Reading tutorials builds knowledge. **Building projects** builds skill. This chapter walks through eight applications step by step. Every line is explained. Every program uses **only features that work in Lang.P v0.2.0**.

| Project | Skills practiced | Status |
|---------|------------------|--------|
| Calculator | Input, operators, functions | ✅ Full |
| Todo App | Lists, loops, functions | ✅ Full |
| Bank System | Types, methods, dict *(Beta OOP)* | ✅ Full |
| Student Management | Dict, list, loops | ✅ Full |
| File Manager | Filesystem, modules | ✅ Full |
| Notes App | Write, read, append | ✅ Full |
| Mini Browser | Module stub | 🟡 Stub demo only |
| Mini AI Chat | Module stub | 🟡 Stub demo only |

**Why line-by-line explanations:** You should understand *why* each statement exists, not only copy code.

**When to build these:** After [11 — Functions](11-functions.md) for Calculator; after [14 — Collections](14-collections.md) for Todo; after [18 — Types and OOP](18-type-oop.md) for Bank System.

---

## Syntax

Each project is a single `.lp` file (or small folder for larger apps). Save code, then:

```bash
lang check my_project.lp
lang run my_project.lp
```

Interactive programs need stdin — pipe values in CI:

```bash
printf '10\n20\n' | lang run calculator.lp
```

---

## Project 1 — Calculator

### Goal

Ask for two numbers and print sum, difference, product, and quotient.

### Full program

**Learning version:**

```lp
@ calculator.lp — basic arithmetic CLI

@ Read two numbers from the user.
num1 = input number "Enter first number: ".
num2 = input number "Enter second number: ".

@ Show what was entered.
print "You entered " with num1 with " and " with num2.

@ Compute and display results.
print "Sum: " with (num1 + num2).
print "Difference: " with (num1 - num2).
print "Product: " with (num1 * num2).
print "Quotient: " with (num1 / num2).
```

### Line-by-line

| Line | Explanation |
|------|-------------|
| `@ calculator.lp…` | File header comment — not executed |
| `num1 = input number …` | Prompt user; store integer in `num1` |
| `num2 = input number …` | Second integer |
| `print "You entered " with …` | Join text and values with `with` |
| `(num1 + num2)` | Parentheses group math before printing |
| Each `print … .` | One result per line; statement ends with `.` |

See also: [examples/calculator.lp](../../examples/calculator.lp).

---

## Project 2 — Todo App

### Goal

Maintain a list of tasks in memory; add tasks and list them.

**Learning version:**

```lp
@ todo.lp — in-memory task list

@ Start with an empty list.
tasks = [].

function add_task(title),
    tasks.append(title).
    print "Added: " with title.
..

function list_tasks(),
    print "--- Tasks ---".
    if tasks.length() == 0,
        print "(empty)".
    otherwise,
        for t in tasks,
            print "- " with t.
        ..
    ..
..

@ Demo usage.
add_task("Learn Lang.P").
add_task("Build Todo App").
list_tasks().
```

### Line-by-line highlights

- `tasks = [].` — empty list literal
- `tasks.append(title).` — mutates list in place
- `tasks.length() == 0` — check before looping
- `for t in tasks,` — iterate each task string

---

## Project 3 — Bank System *(OOP Beta)*

### Goal

Model accounts with deposit, withdraw, and balance using `type`.

**Learning version:**

```lp
@ bank.lp — simple account type

type Account,
    owner.
    balance.

    function init(owner, start_balance),
        self.owner = owner.
        self.balance = start_balance.
    ..

    function deposit(amount),
        self.balance = self.balance + amount.
        print "Deposited " with amount.
    ..

    function withdraw(amount),
        if amount <= self.balance,
            self.balance = self.balance - amount.
            print "Withdrew " with amount.
        otherwise,
            print "Insufficient funds.".
        ..
    ..

    function show(),
        print self.owner with ": " with self.balance.
    ..
..

acct = Account("Naga", 1000).
acct.deposit(250).
acct.withdraw(100).
acct.show().
```

### Line-by-line highlights

- `type Account,` — opens type block with `,`
- `function init(...)` — constructor runs on `Account("Naga", 1000).`
- `self.balance` — instance field access
- `..` closes type and each function

---

## Project 4 — Student Management System

### Goal

Store students in a dictionary keyed by id; add and look up records.

**Learning version:**

```lp
@ students.lp — dict of student records

@ Map id -> {name, grade}
students = {}.

function add_student(id, name, grade),
    students[id] = { name : name, grade : grade }.
    print "Registered " with name.
..

function show_student(id),
    if students.contains(id),
        s = students[id].
        print s.name with " — grade " with s.grade.
    otherwise,
        print "Unknown id.".
    ..
..

add_student(1, "Naga", 92).
add_student(2, "Alex", 88).
show_student(1).
show_student(99).
```

Uses [15 — Dictionaries](15-dictionaries.md) — no OOP required.

---

## Project 5 — File Manager

### Goal

Create, list, and delete files using the filesystem module.

**Learning version:**

```lp
@ file_manager.lp — filesystem operations

use filesystem.

folder = "demo_folder".
file_path = "demo_folder/readme.txt".

filesystem.create_folder(folder).
write "Hello from File Manager" to file_path.

print "Exists: " with filesystem.exists(file_path).
print "Contents:".
print filesystem.list(folder).

delete file_path.
filesystem.remove_folder(folder).
print "Cleanup done.".
```

**Note:** String building with `with` for paths — keep paths simple on your OS.

See [examples/filesystem_demo.lp](../../examples/filesystem_demo.lp).

---

## Project 6 — Notes App

### Goal

Append notes to a file and read them back.

**Learning version:**

```lp
@ notes.lp — persistent notes file

use filesystem.

notes_file = "my_notes.txt".

function add_note(text),
    append text to notes_file.
    append "\n---\n" to notes_file.
    print "Note saved.".
..

function read_notes(),
    if filesystem.exists(notes_file),
        print read notes_file.
    otherwise,
        print "No notes yet.".
    ..
..

add_note("Buy milk").
add_note("Finish Lang.P chapter 35").
read_notes().
```

---

## Project 7 — Mini Browser *(stub)*

### Goal

Show how future Navigator integration will look. **v0.2 only loads the stub module.**

**Learning version:**

```lp
@ mini_browser.lp — navigator stub (v0.2)

use navigator.

print "Navigator version: " with navigator.version.
print "Full browser UI is planned — see Future Roadmap.".
```

This matches [examples/browser.lp](../../examples/browser.lp). A real browser requires the Navigator framework ([37 — Future Roadmap](37-future-roadmap.md)).

---

## Project 8 — Mini AI Chat *(stub)*

### Goal

Verify the AI module loads. **No LLM calls in v0.2.**

**Learning version:**

```lp
@ mini_ai.lp — AI module stub (v0.2)

use ai.

print "AI module version: " with ai.version.
print "Chat functionality is planned for a future release.".
```

See [examples/agent.lp](../../examples/agent.lp).

---

## Common mistakes

**Mistake:** Building Mini Browser/AI Chat expecting real network or GUI behavior.

**Why:** Only stub modules exist in v0.2.

**Fix:** Treat projects 7–8 as integration placeholders; read [37 — Future Roadmap](37-future-roadmap.md).

---

**Mistake:** Skipping `lang check` and debugging runtime errors from typos.

**Fix:** Always check first ([30 — Debugging](30-debugging.md)).

---

## Best practices

- Start from the learning version; refactor to professional version when comfortable.
- One project per folder when using `lang init` ([29 — Project Structure](29-project-structure.md)).
- Add `tests/` files and run `lang test` for projects 1–6.

---

## Exercises

### Beginner

1. Run the Calculator with piped input `10` and `20`.
2. Add a "clear all tasks" function to Todo App.
3. Add one more student to the management system.
4. Save one note and read it back in Notes App.
5. Run Mini Browser stub and record the version string.

### Intermediate

1. Extend Bank System with transfer between two accounts.
2. Persist Todo list to a file on disk.
3. Add `remove_student(id)` to Student Management.
4. List only `.txt` files in File Manager (filter in loop).
5. Combine Calculator logic into functions `add`, `sub`, `mul`, `div`.

### Advanced

1. Merge Bank System and Student Management using shared `type` patterns.
2. Create a `lang init todo-app` project with `tests/todo.lp`.
3. Design the API you want for Mini AI Chat when implemented — document in comments only.

---

## Summary

You built eight projects from calculator to filesystem tools, using lists, dicts, functions, types, and modules. Stub projects show where Lang.P is heading without pretending features exist today.

**Previous:** [34 — Language Reference](34-language-reference.md) · **Next:** [36 — FAQ](36-faq.md)

**See also:** [examples/](../../examples/), [28 — Package Manager](28-package-manager.md)
