# Lang.P

**Lang.P** (spoken name: **Lang**) is a production-oriented programming language designed to be the world's most readable programming language — powerful enough for desktop apps, browsers, AI agents, APIs, games, and systems software, yet approachable enough that a beginner can understand most code after seeing it once.

| Component       | Name        |
|-----------------|-------------|
| File extension  | `.lp`       |
| Compiler        | `langc`     |
| Package manager | `lang`      |
| IDE             | Lang Studio |
| Language Server | Lang LSP    |

## Install (macOS, Linux, Windows)

**One-line install** (downloads pre-built binary when available, otherwise builds from source):

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex
```

**Build from source** (fast profile — recommended):

```bash
git clone https://github.com/Nagashreeshyl/langp.git
cd langp
./scripts/build-fast.sh
export PATH="$PWD/target/release-fast:$PATH"
```

Add `~/.local/bin` to your PATH if the installer puts `langc` there.

## Run programs

```bash
langc run examples/hello.lp          # execute
langc check examples/input_demo.lp   # semantic check
langc build examples/hello.lp -o hello   # build runnable bundle
langc --emit ast examples/hello.lp   # debug AST as JSON
langc --version
```

## Quick Example

```lp
@ Greet the user by name.
function greet(name),
    print "Hello " with name with "!".
.

greet("World").
```

Output: `Hello World!`

## Project Structure

```
/langp
    /lexer         — Tokenization
    /parser        — Syntax analysis
    /ast           — Abstract syntax tree definitions
    /semantic      — Semantic analyzer
    /runtime       — Runtime values and errors
    /interpreter   — Tree-walking interpreter
    /langc         — Compiler CLI
    /scripts       — install.sh, install.ps1, build-fast.sh
    /examples      — Example programs
    /tests         — Integration & conformance tests
    /docs          — Language specification & guides
```

## Development Roadmap

| Phase | Deliverable                    | Status      |
|-------|--------------------------------|-------------|
| 1     | Complete language specification | **Complete** |
| 2     | Formal grammar (EBNF)          | **Complete** |
| 3     | Lexer                          | **Complete** |
| 4     | Parser + AST                   | **Complete** |
| 5     | AST generation (serde JSON)    | **Complete** |
| 6     | Semantic analyzer              | **Complete** |
| 7     | Interpreter                    | **Complete** |
| 8     | Build bundles (`langc build`)  | **Complete** |
| 9     | Runtime (values, builtins)     | **Complete** |
| 10    | Package manager                | Pending     |
| 11    | Navigator framework            | Pending     |
| 12    | AI framework                   | Pending     |
| 13    | Language Server                | Pending     |
| 14    | Lang Studio IDE                | Pending     |

## Faster compilation

Debug builds use optimized dependencies (`opt-level = 1/2`). For daily use:

| Command | When to use |
|---------|-------------|
| `./scripts/build-fast.sh` | Fast optimized `langc` (~10s first time, instant after) |
| `cargo test` | Run all tests (first compile slower, then incremental) |
| `cargo build --release -p langc` | Smallest/fastest native binary for distribution |

## Documentation

The authoritative language specification lives in [`docs/spec/`](docs/spec/).

- [Language Specification Index](docs/spec/README.md)
- [Full Specification (single document)](docs/spec/LANGP-SPEC.md)
- [Formal Grammar (EBNF)](docs/grammar/README.md)

## License

TBD.
