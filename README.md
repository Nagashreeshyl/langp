# Lang.P

**Version 0.2.0** · [Status](STATUS.md) · [Changelog](CHANGELOG.md) · [Roadmap](ROADMAP.md)

**Lang.P** (spoken name: **Lang**) is a readable programming language — approachable for beginners, structured for production. v0.2 adds collections, objects, modules, filesystem stdlib, and a package manager foundation.

| Component       | Name        |
|-----------------|-------------|
| File extension  | `.lp`       |
| Runner          | `lang`      |
| Compiler CLI    | `langc`     |
| Package manager | `lang` (built-in) |
| Language Server | `lang-lsp`  |
| IDE             | VS Code / Cursor extension |

## Install (one line)

Installs `lang`, `langc`, `lang-lsp`, and the Cursor/VS Code extension:

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex
```

Reload your IDE, then:

```bash
lang run examples/hello.lp
lang init my-project
lang install filesystem
```

## What's new in v0.2

- **Collections** — List, Dict, Set, Tuple with methods and generics
- **Objects (beta)** — `type`, methods, `init`, `extends`
- **Modules (beta)** — `use filesystem.`, stdlib modules
- **Package manager (beta)** — `langp.toml`, `lang install`, lock file
- **Release docs** — STATUS, limitations, roadmap, contributing guide

See [CHANGELOG.md](CHANGELOG.md) for full details.

## Learn

| Guide | Description |
|-------|-------------|
| **[Lang.P Manual](docs/manual/README.md)** | 25-chapter guide |
| **[How to Code](docs/guides/HOW-TO-CODE.md)** | Beginner tutorial |
| **[Language Reference](docs/guides/LANGUAGE-REFERENCE.md)** | What works in v0.2 |
| **[STATUS.md](STATUS.md)** | Feature stability matrix |

## Run programs

```bash
lang run examples/hello.lp
lang check examples/hello.lp
lang init                          # new project
lang build                         # check project entry
langc --emit ast examples/hello.lp # debug AST
```

## Quick example

```lp
@ Greet the user by name.
function greet(name),
    print "Hello " with name with "!".
..

greet("World").
```

## Project structure

```
/langp
    /lexer /parser /ast /semantic   Compiler front-end
    /runtime /interpreter            Execution
    /langc /lang /langpm /lang-lsp   Tooling
    /editors                         VS Code extension
    /examples                        Runnable programs
    /tests                           Conformance tests
    /docs                            Spec, manual, guides
```

See [TECH-STACK.md](docs/TECH-STACK.md) and [ARCHITECTURE-v0.2.md](docs/ARCHITECTURE-v0.2.md).

## Development

```bash
./scripts/build-fast.sh
cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md) and [STYLE_GUIDE.md](STYLE_GUIDE.md).

## Documentation index

- [Grammar Freeze v1.0](docs/spec/GRAMMAR-FREEZE-v1.md) — official syntax
- [Specification](docs/spec/README.md)
- [Known Limitations](KNOWN_LIMITATIONS.md)
- [Security](SECURITY.md)

## License

[MIT](LICENSE)
