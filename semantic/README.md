# Semantic Analyzer

Static analysis for Lang.P — name resolution, duplicate detection, and undefined-name diagnostics.

## Status

Phase 6 complete.

## Usage

```bash
cargo test -p langp-semantic
langc check examples/hello.lp
```

## Output

`Diagnostic` values with error codes `E0201`–`E0204` and warning `W0101`.
