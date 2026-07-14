# Lexer

Tokenization module for the Lang.P compiler. Converts source text into a token stream.

## Implementation

- **Language:** Rust
- **Crate:** `langp-lexer` (library)
- **Grammar:** [`docs/grammar/02-lexical-grammar.ebnf`](../docs/grammar/02-lexical-grammar.ebnf)

## Responsibilities

- Recognize comments (`@`), statement terminators (`.`), block delimiters (`,` / `..`)
- Tokenize identifiers, keywords, literals, and operators
- Handle compound keywords (`otherwise if`, `wait for`, `repeat forever`)
- Disambiguate period as `STMT_END` vs member-access `DOT`
- Emit `INDENT` / `DEDENT` for indentation-based blocks
- Recognize contextual input-type keywords after `input`
- Report lexical errors with source locations (`E0100`–`E0104`)

## Status

**Phase 3 complete.** Implemented in Rust with unit and conformance tests.

## Usage

```rust
use langp_lexer::{lex, format_tokens};

let source = r#"print "Hello, Lang.P!"."#;
let tokens = lex(source).unwrap();
println!("{}", format_tokens(&tokens));
```

## CLI

```bash
cargo run -p langc -- --emit tokens examples/hello.lp
```

## Testing

```bash
cargo test -p langp-lexer
```

Tests include:
- Unit tests in `src/lexer.rs`
- Conformance fixtures in `tests/conformance/parse/valid/`

## Output

Token stream consumed by the parser (`parser/` — Phase 4).

## Module layout

```
lexer/
    src/
        lib.rs      Public API
        token.rs    TokenKind, Keyword, Token
        span.rs     Source locations
        error.rs    LexError types
        lexer.rs    Lexer implementation
    tests/
        conformance.rs
```
