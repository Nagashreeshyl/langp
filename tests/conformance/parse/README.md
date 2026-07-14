# Parse Conformance Tests

Grammar conformance tests verify that the lexer and parser accept or reject programs according to the formal grammar in [`docs/grammar/`](../../docs/grammar/).

## Layout

```
parse/
    valid/       Programs that MUST parse successfully
    invalid/     Programs that MUST fail with syntax errors
    README.md    This file
```

## Valid Tests

Each `.lp` file in `valid/` represents a syntactic construct that must parse without errors. Files are named by feature:

| File | Construct tested |
|------|------------------|
| `hello.lp` | Top-level statements, print, assignment |
| `blocks.lp` | if/repeat/for/while blocks with `,` / `..` |
| `functions.lp` | Function declarations and calls |
| `types.lp` | Type definitions and object creation |
| `input.lp` | Built-in input expression (all variants) |
| `with_expr.lp` | `with` composition operator |
| `events.lp` | `on` event handlers |
| `try_catch.lp` | try/catch/finally |
| `io_natural.lp` | read, write, get, post forms |
| `control_flow.lp` | otherwise if, repeat forever |

## Invalid Tests

Each `.lp` file in `invalid/` MUST produce at least one syntax error:

| File | Expected error |
|------|----------------|
| `missing_assign_stmt_end.lp` | Assignment without terminating `.` |
| `dot_block_close.lp` | Block closed with `.` instead of `..` |
| `missing_stmt_end.lp` | Statement without terminating `.` |
| `else_if_instead_of_otherwise.lp` | `else if` is not valid |
| `input_with_parens.lp` | `input("prompt")` — no parentheses |
| `brace_block.lp` | `{ }` blocks not supported |
| `semicolon_terminator.lp` | `;` not valid as terminator |

## Running Tests

```bash
@ Once langc parser exists:
lang test --filter conformance/parse

@ Or directly:
langc --mode parse tests/conformance/parse/valid/hello.lp
langc --mode parse tests/conformance/parse/invalid/missing_stmt_end.lp  # must fail
```

## Adding Tests

1. Derive test from a **MUST** requirement in the spec or grammar.
2. Place in `valid/` or `invalid/`.
3. Name file descriptively: `feature_name.lp` or `error_case.lp`.
4. Add entry to the table in this README.

## Phase

Introduced in **Phase 2** (grammar). Executed by parser in **Phase 4**.
