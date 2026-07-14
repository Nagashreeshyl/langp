# langc

Command-line compiler for Lang.P. Compiles `.lp` source files to native binaries or bytecode.

## Usage

```bash
cargo run -p langc -- examples/hello.lp
cargo run -p langc -- --emit tokens examples/hello.lp
```

Planned flags (see [Chapter 21 — Tooling](../docs/spec/21-tooling.md)):

```bash
langc main.lp                    # Compile to executable
langc main.lp -o myapp           # Specify output name
langc main.lp --mode interpret   # Run via interpreter
langc main.lp --mode check       # Type-check only
langc main.lp --emit ast         # Dump AST for debugging
```

## Status

**Phase 3 (partial):** `--emit tokens` via `langp-lexer`. Full compilation pipeline pending (Phases 4–8).

## Dependencies

- `langp-lexer` — tokenization (Phase 3)
- `compiler/` — compilation pipeline (future)
- `runtime/` — linking and execution (future)
