# Interpreter

Tree-walking interpreter for Lang.P programs.

## Supported (v0.1)

- Functions, calls, `print`, `with` string concat
- `input` (text, number, decimal, boolean, password via stdin)
- Control flow: `if`, `while`, `for`, `repeat`, `try/catch`
- Collections: lists, dicts, indexing
- Assignments including `self.name`
- Builtins: `len`, `to_string`, `assert`
- File I/O: `read`, `write`, `copy`, `move`, `delete`

## Status

Phase 7 complete.

## Usage

```bash
cargo test -p langp-interpreter
langc run examples/hello.lp
```

## Not yet implemented

- HTTP expressions (`get`, `post`, …)
- Async / `wait for`
- Lambdas, `is` type checks, full OOP
