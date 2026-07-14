# Compiler

Core compiler pipeline for Lang.P. Orchestrates the full compilation process from source to executable.

## Pipeline

```
Source (.lp) → Lexer → Parser → AST → Semantic Analyzer → Optimizer → IR → Codegen
```

## Responsibilities

- Coordinate compilation phases
- Error reporting and diagnostics
- Optimization passes
- Code generation (bytecode and native via LLVM)
- Cross-compilation support

## Status

Phase 3–8 (not yet implemented). See the [language specification](../docs/spec/) for requirements.

## Dependencies

- `lexer/` — tokenization
- `parser/` — syntax analysis
- `ast/` — tree definitions
- `runtime/` — code generation targets
