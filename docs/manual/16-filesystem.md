# 16 — File System

**Status: Implemented (v0.1)**

---

## Read expressions

| Expression | Description |
|------------|-------------|
| `read "path"` | File as text |
| `read_bytes "path"` | Raw bytes |
| `read_lines "path"` | Lines as list |

### Learning version

```lp
@ Read a text file.
content = read "notes.txt".
print content.

@ Read line by line.
lines = read_lines "log.txt".
for line in lines,
    print line.
..
```

---

## Write statements

| Statement | Description |
|-----------|-------------|
| `write value to "path".` | Write text |
| `write_bytes value to "path".` | Write bytes |
| `append value to "path".` | Append text |

```lp
write "Hello file" to "out.txt".
append "\nSecond line" to "out.txt".
```

---

## File management

| Statement | Description |
|-----------|-------------|
| `copy "src" to "dest".` | Copy file |
| `move "src" to "dest".` | Move file |
| `rename "src" to "dest".` | Rename file |
| `delete "path".` | Delete file |

```lp
copy "backup/old.txt" to "archive/old.txt".
delete "temp.txt".
```

---

## Next steps

- [19 — Collections](19-collections.md)
- [I/O (spec)](../spec/15-io-network.md)
