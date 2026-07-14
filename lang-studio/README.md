# Lang Studio

Official integrated development environment for Lang.P.

## Features

| Feature | Description |
|---------|-------------|
| Auto indentation | `,` indents, `..` dedents automatically |
| Syntax highlighting | Full semantic highlighting via Lang LSP |
| Autocomplete | Context-aware with inline documentation |
| Debugger | Breakpoints, step, watch, call stack |
| Profiler | CPU and memory profiling |
| Package manager | GUI for `lang add/remove/update` |
| AI Assistant | Built-in chat and code generation |
| Visual Browser Designer | Drag-and-drop Navigator UI builder |
| Live Preview | Run and preview in IDE |
| Project templates | One-click browser, API, agent, CLI projects |
| Comment toggle | Show/hide `@` comments in generated code |

## Responsibilities

- Provide a complete development environment for Lang.P
- Integrate Lang LSP for all language features
- Host the Visual Browser Designer for Navigator
- Embed AI assistant powered by the Lang.P AI framework
- Manage projects via the `lang` package manager

## Status

Phase 14 (not yet implemented). See [Chapter 21 — Tooling](../docs/spec/21-tooling.md).

## Dependencies

- `lang-lsp/` — language features
- `lang/` — package management
- `langc/` — compilation
- `navigator/` — browser designer and live preview
