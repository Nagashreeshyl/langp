# lang-lsp

Language Server Protocol (LSP) implementation for Lang.P — powers IDE features in Cursor, VS Code, and any LSP-compatible editor.

## Features

- **Diagnostics** — parse errors and semantic analysis (undefined symbols, type errors, etc.)
- **Autocomplete** — keywords, builtins, snippets, and symbols from the current file
- **Hover** — keyword docs and function signatures
- **Go to definition** — jump to function, type, or enum declarations
- **Document symbols** — outline view in the sidebar

## Build

```bash
cargo build --profile release-fast -p langp-lsp
# binary: target/release-fast/lang-lsp
```

## Use with editors

Install the official extension: [`editors/vscode-langp`](../editors/vscode-langp/README.md)

The extension spawns `lang-lsp` on stdio when you open a `.lp` file. Ensure `lang-lsp` is on your PATH (included in the main install script).

## Manual test

```bash
lang-lsp   # waits on stdin/stdout — normally started by the editor
```
