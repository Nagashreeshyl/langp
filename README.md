# Lang.P

**Lang.P** (spoken name: **Lang**) is a production-oriented programming language designed to be the world's most readable programming language — powerful enough for desktop apps, browsers, AI agents, APIs, games, and systems software, yet approachable enough that a beginner can understand most code after seeing it once.

| Component       | Name        |
|-----------------|-------------|
| File extension  | `.lp`       |
| Compiler        | `langc`     |
| Package manager | `lang`      |
| IDE             | Lang Studio |
| Language Server | Lang LSP    |

## Install (one line)

Installs `lang`, `langc`, `lang-lsp`, and the Cursor/VS Code extension automatically:

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex
```

Then **reload Cursor/VS Code** and run:

```bash
lang run examples/hello.lp
lang examples/hello.lp          # shorthand
```

## Uninstall (one line)

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.ps1 | iex
```

## Learn to code

**Start here:** [How to Code in Lang.P](docs/guides/HOW-TO-CODE.md) — complete beginner guide with loops, functions, conditionals, and Python comparisons.

## IDE support (Cursor / VS Code)

The install script sets up everything automatically — no `cursor` CLI needed:

- `.lp` file icon in the explorer
- Syntax highlighting and snippets
- **Auto-indent** after `,` and **de-indent** on `..`
- Autocomplete, error squiggles, hover, go-to-definition (via `lang-lsp`)

If you cloned the repo locally instead:

```bash
./scripts/install.sh
```

## Run programs

```bash
lang run examples/hello.lp       # run (recommended)
lang examples/hello.lp           # same thing
lang check examples/hello.lp     # check for errors
langc --emit ast examples/hello.lp   # compiler/debug tools
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
    /lang-lsp      — Language Server (IDE autocomplete, diagnostics)
    /editors       — VS Code / Cursor extension
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
| 13    | Language Server                | **Complete** |
| 14    | Lang Studio IDE                | Partial (VS Code/Cursor extension) |

## Faster compilation

Debug builds use optimized dependencies (`opt-level = 1/2`). For daily use:

| Command | When to use |
|---------|-------------|
| `./scripts/build-fast.sh` | Fast optimized `langc` (~10s first time, instant after) |
| `cargo test` | Run all tests (first compile slower, then incremental) |
| `cargo build --release -p langc` | Smallest/fastest native binary for distribution |

## Documentation

- **[How to Code in Lang.P](docs/guides/HOW-TO-CODE.md)** — beginner guide (start here)
- [Language Specification Index](docs/spec/README.md)
- [Full Specification (single document)](docs/spec/LANGP-SPEC.md)
- [Formal Grammar (EBNF)](docs/grammar/README.md)

## License

TBD.
