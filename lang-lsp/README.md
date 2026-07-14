# Lang LSP

Language Server Protocol implementation for Lang.P. Powers IDE features across all editors.

## Capabilities

| Feature | LSP Method | Version |
|---------|-----------|---------|
| Diagnostics | `textDocument/publishDiagnostics` | v0.1 |
| Autocomplete | `textDocument/completion` | v0.1 |
| Hover documentation | `textDocument/hover` | v0.1 |
| Go to definition | `textDocument/definition` | v0.1 |
| Find references | `textDocument/references` | v0.1 |
| Rename symbol | `textDocument/rename` | v0.1 |
| Formatting | `textDocument/formatting` | v0.1 |
| Semantic highlighting | `textDocument/semanticTokens` | v0.1 |
| Signature help | `textDocument/signatureHelp` | v0.1 |

## Responsibilities

- Incremental parsing and type checking
- Symbol indexing across project files
- Auto-indentation on `,` and dedentation on `..`
- Code formatting (`langfmt` integration)
- Refactoring support

## Status

Phase 13 (not yet implemented). See [Chapter 21 — Tooling](../docs/spec/21-tooling.md).

## Dependencies

- `compiler/` — parser and semantic analyzer
- `ast/` — tree definitions

## Editor Integration

Used by Lang Studio, VS Code, Cursor, Neovim, and JetBrains via the standard LSP protocol.
