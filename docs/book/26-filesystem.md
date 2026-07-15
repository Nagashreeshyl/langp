# Filesystem

## Introduction

The **filesystem** features in Lang.P let your programs read, write, copy, move, and delete files and folders on disk. You can use **top-level statements** (`read`, `write`, `delete`, …) without any import, or the **`filesystem` module** for function-style access ([24 — Modules](24-modules.md)).

**Why filesystem I/O:** Persist data between runs, process log files, build small tools, and organize project output.

**When to use it:** Scripts that save results, read configuration, or manage directories. Always handle errors with `try`/`catch` when failure is possible ([27 — Error Handling](27-error-handling.md)).

---

## Syntax

### Read expressions

| Expression | Description |
|------------|-------------|
| `read "path"` | File contents as text |
| `read_bytes "path"` | Raw bytes as string |
| `read_lines "path"` | Lines as a list |

```lp
content = read "notes.txt".
lines = read_lines "log.txt".
```

### Write statements

| Statement | Description |
|-----------|-------------|
| `write value to "path".` | Write text (overwrites) |
| `write_bytes value to "path".` | Write raw bytes |
| `append value to "path".` | Append text |

```lp
write "Hello file" to "out.txt".
append "\nSecond line" to "out.txt".
```

### File management statements

| Statement | Description |
|-----------|-------------|
| `copy "src" to "dest".` | Copy file |
| `move "src" to "dest".` | Move file |
| `rename "src" to "dest".` | Rename file (same as move) |
| `delete "path".` | Delete file or directory tree |

```lp
copy "notes.txt" to "backup.txt".
move "old.txt" to "archive/old.txt".
rename "draft.txt" to "final.txt".
delete "temp.txt".
```

### `use filesystem.` module API

```lp
use filesystem.

text = filesystem.read("notes.txt").
filesystem.write("data", "out.txt").
filesystem.append("more", "out.txt").
filesystem.copy("a.txt", "b.txt").
filesystem.move("a.txt", "c.txt").
filesystem.delete("c.txt").
exists = filesystem.exists("notes.txt").
names = filesystem.list("Project").
filesystem.create_folder("Project").
filesystem.remove_folder("Project").
```

Module functions return values; top-level statements are imperative and return nothing.

---

## Examples

### Simple — read and write

**Learning version:**

```lp
@ Create a file, read it back, print contents.
write "Hello Lang.P" to "greeting.txt".
text = read "greeting.txt".
print text.
delete "greeting.txt".
```

**Professional version:**

```lp
write "Hello Lang.P" to "greeting.txt".
print read "greeting.txt".
delete "greeting.txt".
```

### Intermediate — full demo pattern

**Learning version:**

```lp
@ Based on examples/filesystem_demo.lp
use filesystem.

write "Hello Lang.P filesystem" to "notes.txt".
text = read "notes.txt".
write text to "backup.txt".

print filesystem.exists("notes.txt").
print filesystem.exists("missing.txt").

copy "notes.txt" to "notes-copy.txt".
delete "notes-copy.txt".

filesystem.create_folder("Project").
write "data" to "Project/data.txt".
print filesystem.list("Project").

filesystem.remove_folder("Project").
delete "backup.txt".
delete "notes.txt".

print "Filesystem demo done.".
```

**Professional version:**

```lp
use filesystem.

write "Hello Lang.P filesystem" to "notes.txt".
print filesystem.exists("notes.txt").
filesystem.create_folder("Project").
write "data" to "Project/data.txt".
print filesystem.list("Project").
filesystem.remove_folder("Project").
delete "notes.txt".
```

Run: `lang run examples/filesystem_demo.lp`

### Advanced — safe read with try/catch

**Learning version:**

```lp
@ Missing files cause runtime errors — catch them.
try,
    data = read "config.txt".
    print data.
catch err,
    print "Could not read config: " with err.
..
```

**Professional version:**

```lp
try,
    print read "config.txt".
catch err,
    print "Error: " with err.
..
```

---

## Common Mistakes

**Mistake:** Forgetting quotes around paths

```lp
write "hi" to notes.txt.    @ wrong
```

**Fix:**

```lp
write "hi" to "notes.txt".
```

---

**Mistake:** Assuming `delete` only removes files

```lp
delete "Project".    @ removes directory tree if path is a folder
```

**Fix:** Use `filesystem.remove_folder("Project")` or `delete` knowing it handles both files and directories.

---

**Mistake:** Writing to paths your environment cannot access (CI sandboxes)

**Fix:** Use relative paths in the project directory; see [KNOWN_LIMITATIONS.md](../../KNOWN_LIMITATIONS.md) platform notes.

---

## Best Practices

- Use `try`/`catch` around reads of user-supplied paths ([27 — Error Handling](27-error-handling.md)).
- Clean up temporary files at the end of scripts (`delete`, `remove_folder`).
- Prefer `filesystem.exists` before conditional reads when the file may be missing.
- Use `append` for logs; `write` for fresh output.
- Run `lang check` before `lang run` on file-heavy programs ([32 — Best Practices](32-best-practices.md)).

---

## Exercises

### Beginner

1. Write `"Lang.P"` to `"test.txt"`, read it, print it, delete it.
2. Use `append` to add two lines to the same file.
3. Copy a file to `"backup.txt"` then delete the backup.
4. Import `filesystem` and print `filesystem.exists(".")`.
5. Run [examples/filesystem_demo.lp](../../examples/filesystem_demo.lp).

### Intermediate

1. Create folder `"data"`, write three files inside, list the folder, remove the folder.
2. Read a file with `try`/`catch` and print a friendly message on failure.
3. Compare `move` vs. `rename` — run both and observe behavior.
4. Write a program that reads `read_lines` and prints each line in a `for` loop ([13 — Loops](13-loops.md)).
5. Use module API only (no top-level `write`) for a mini backup script.

### Advanced

1. Build a simple journal: append a timestamp line each run, then print all lines.
2. Document which filesystem operations can fail in sandboxed CI and how to test locally.

---

## Summary

Lang.P provides **top-level file statements** (`read`, `write`, `append`, `copy`, `move`, `rename`, `delete`) and a full **`filesystem` module** with matching functions plus `exists`, `list`, `create_folder`, and `remove_folder`. Combine with `try`/`catch` for robust programs.

**Previous:** [25 — Imports](25-imports.md) · **Next:** [27 — Error Handling](27-error-handling.md)

**See also:** [24 — Modules](24-modules.md), [25 — Imports](25-imports.md), [27 — Error Handling](27-error-handling.md), [examples/filesystem_demo.lp](../../examples/filesystem_demo.lp), [Manual: File System](../manual/16-filesystem.md)
