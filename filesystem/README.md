# Filesystem

File and directory I/O with natural-language syntax for Lang.P programs.

## Capabilities

- Read/write files: `read "file.txt"`, `write text to "output.txt"`
- Directory operations: `create_dir`, `list_files`, `delete_dir`
- Path manipulation: `path.join`, `path.resolve`, `path.extension`
- File watching with event handlers
- Async streaming for large files

## Quick Example

```lp
use filesystem.

text = read "settings.txt".
write text to "backup.txt".
exists = file_exists("config.json").
```

## Status

Part of the standard library. See [Chapter 15 — I/O & Network](../docs/spec/15-io-network.md).

## Dependencies

- `runtime/` — platform-specific file system calls
