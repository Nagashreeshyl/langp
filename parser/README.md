# Parser

Syntax analysis module for the Lang.P compiler. Builds an Abstract Syntax Tree from token streams.

## Responsibilities

- Parse all Lang.P syntactic constructs (statements, blocks, types, functions, events)
- Enforce block structure (`,` opener, `..` closer, indentation)
- Handle expression parsing with correct operator precedence
- Produce error messages for syntax violations
- Support incremental parsing for LSP integration

## Status

Phase 4 complete. Recursive-descent parser over the Phase 2 grammar; outputs `langp-ast` nodes with source spans.

## Usage

```bash
cargo test -p langp-parser
cargo run -p langc -- --emit ast examples/hello.lp
```

## Output

Abstract Syntax Tree (AST) consumed by the semantic analyzer.

## Dependencies

- `lexer/` — input token stream
- `ast/` — output tree node definitions
