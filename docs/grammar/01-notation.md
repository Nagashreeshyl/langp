# Grammar Notation

**Version:** 0.1.0  
**Status:** Phase 2 — Formal Grammar  
**Last updated:** 2026-07-14

This document defines the notation used in the Lang.P formal grammar.

## EBNF Variant

Lang.P grammar uses **ISO/IEC 14977 Extended BNF** with these conventions:

| Symbol | Meaning |
|--------|---------|
| `=` | Definition |
| `\|` | Alternative |
| `[ ... ]` | Optional (zero or one) |
| `{ ... }` | Repetition (zero or more) |
| `( ... )` | Grouping |
| `"..."` | Terminal string (literal token) |
| `'...'` | Terminal character |
| `?` suffix on rule | Greedy optional (documentation only) |

## Lexical vs Syntactic

The grammar is split into two layers:

1. **Lexical grammar** ([`02-lexical-grammar.ebnf`](02-lexical-grammar.ebnf)) — character sequences → tokens
2. **Syntactic grammar** ([`03-syntactic-grammar.ebnf`](03-syntactic-grammar.ebnf)) — tokens → parse tree

The lexer emits all tokens in §Lexical. The parser consumes tokens and produces an AST.

## Special Tokens

These tokens are **not** written literally in source code. The lexer synthesizes them:

| Token | Description |
|-------|-------------|
| `INDENT` | Increased indentation (4 spaces) after a block-opening `,` |
| `DEDENT` | Decreased indentation back to the block opener's level |
| `NEWLINE` | Line break (significant for statement boundaries) |
| `STMT_END` | Statement terminator `.` when followed by whitespace/newline |
| `DOT` | Member-access `.` when followed by an identifier or digit |
| `BLOCK_CLOSE` | Block closer `..` at the opener's indentation level |

### Statement Terminator Disambiguation

The character `.` has dual roles:

```
user.name = "Naga".     @ DOT (member) ... STMT_END (terminator)
print user.name.        @ DOT ... STMT_END
```

**Rule:** If `.` is followed by an identifier, digit, or `(`, emit `DOT` (member access / method call). If followed by whitespace, comment, or end-of-line, emit `STMT_END`.

### Block Structure

Lang.P blocks use `,` to open and `..` to close:

```lp
if age >= 18,          @ COMMA + NEWLINE + INDENT
    print "Adult".      @ block body
..                     @ DEDENT + BLOCK_CLOSE
```

Token stream for the above:

```
IF IDENT(STMT_END) GE INTEGER(STMT_END) COMMA NEWLINE INDENT PRINT STRING(STMT_END) NEWLINE DEDENT BLOCK_CLOSE
```

Wait — `age >= 18` doesn't have STMT_END before comma. Let me reconsider.

Actually:
```
if age >= 18,
```

Token stream:
```
KW_IF expr COMMA NEWLINE INDENT ... DEDENT BLOCK_CLOSE
```

The comma is `COMMA` token (block opener), not argument separator in this context.

### Two-Token Keywords

The lexer MUST recognize these as single tokens:

| Source | Token |
|--------|-------|
| `otherwise if` | `KW_OTHERWISE_IF` |
| `repeat forever` | `KW_REPEAT_FOREVER` |
| `wait for` | `KW_WAIT_FOR` |

### Contextual Keywords

Recognized only in specific syntactic positions (see syntactic grammar):

| Tokens | Position |
|--------|----------|
| `text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, `color` | After `input` |
| `inline` | After `print` |
| `times`, `as` | In `repeat` headers |
| `to` | In `write` / `copy` / `move` statements |
| `enabled`, `disabled` | Configuration values (identifiers in v0.1) |

## Precedence

Expression precedence is defined in the syntactic grammar using layered rules (`expr` → `with_expr` → `or_expr` → ...). See [Chapter 6 §6.7](../spec/06-expressions.md#67-operator-precedence).

## Start Symbol

```
program = module .
```

## Conformance

A parser is **grammar-conformant** if it accepts all programs in `tests/conformance/parse/valid/` and rejects all programs in `tests/conformance/parse/invalid/` with syntax errors.

## Relationship to Specification

The grammar is derived from the language specification ([`docs/spec/`](../spec/)). When the spec and grammar disagree, the **specification** is authoritative; file a grammar bug.

Amendments MUST update both spec and grammar in the same change set.
