# AST

Abstract Syntax Tree definitions for the Lang.P compiler. Shared data structures used across the pipeline.

## Node Categories

| Category | Examples |
|----------|----------|
| Declarations | `FunctionDecl`, `TypeDecl`, `EnumDecl`, `ImportDecl` |
| Statements | `AssignStmt`, `PrintStmt`, `ReturnStmt`, `IfStmt`, `TryStmt` |
| Expressions | `BinaryExpr`, `CallExpr`, `WithExpr`, `LambdaExpr` |
| Types | `NamedType`, `GenericType`, `FunctionType`, `OptionalType` |
| Events | `EventHandler`, `EventDecl`, `EventEmit` |

## Responsibilities

- Define all AST node types with source location metadata
- Provide visitor/walker patterns for tree traversal
- Serialization for `--emit ast` debugging
- Stable API for semantic analyzer, optimizer, and LSP

## Status

Phase 4/5 complete. AST node types and JSON serialization via `to_json()` for `--emit ast`.

## Usage

```bash
cargo test -p langp-ast
```
