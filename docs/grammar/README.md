# Lang.P Formal Grammar

**Version:** 0.1.0  
**Status:** Phase 2 Complete  
**Last updated:** 2026-07-14

This directory contains the authoritative formal grammar for Lang.P, expressed in Extended BNF (EBNF). Parser and lexer implementations **must** conform to this grammar.

## Documents

| File | Description |
|------|-------------|
| [01-notation.md](01-notation.md) | EBNF conventions, special tokens, block algorithm |
| [02-lexical-grammar.ebnf](02-lexical-grammar.ebnf) | Token definitions (characters → tokens) |
| [03-syntactic-grammar.ebnf](03-syntactic-grammar.ebnf) | Syntax rules (tokens → parse tree) |
| [LANGP-EBNF.md](LANGP-EBNF.md) | Combined human-readable reference |

## Start Symbol

```
program = module EOF
```

## Special Lexer Tokens

Lang.P uses indentation-based blocks with explicit delimiters. The lexer emits synthetic tokens:

| Token | Trigger |
|-------|---------|
| `INDENT` | Deeper indentation after block-opening `,` |
| `DEDENT` | Return to opener's indentation level |
| `STMT_END` | `.` followed by whitespace/newline (statement terminator) |
| `DOT` | `.` followed by identifier (member access) |
| `BLOCK_CLOSE` | `..` at block opener's indentation |

## Grammar Highlights

Unique Lang.P constructs captured in the grammar:

- **Statement terminator:** every statement ends with `STMT_END` (`.`)
- **Blocks:** `COMMA` + `INDENT` body + `BLOCK_CLOSE` (`..`)
- **Composition:** `with_expr` rule for the `with` operator
- **Input:** `input_expr` with optional typed keyword
- **Natural I/O:** `read`, `write ... to`, `get`, `post ... with`
- **Events:** `on expr, block ..`
- **Control flow:** `otherwise if`, `repeat N times`, `repeat forever`
- **Async:** `wait for expr` in `http_expr`
- **Object creation:** positional `Type(args)` and named-field body blocks

## Conformance Tests

Grammar conformance tests live in [`tests/conformance/parse/`](../../tests/conformance/parse/):

| Directory | Expectation |
|-----------|-------------|
| `valid/` | MUST parse without syntax errors |
| `invalid/` | MUST produce syntax errors |

Run (once parser exists):

```bash
lang test --filter conformance/parse
```

## Relationship to Specification

Derived from [`docs/spec/`](../spec/). The specification is authoritative when spec and grammar disagree.

## Change Process

1. Amend the language specification first.
2. Update lexical and/or syntactic grammar.
3. Add/update conformance fixtures in `tests/conformance/parse/`.
4. Bump grammar version to match spec version.

## Next Phase

**Phase 3:** Implement the lexer (`lexer/`) according to `02-lexical-grammar.ebnf`.
