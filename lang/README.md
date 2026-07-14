# lang

User-facing CLI for Lang.P — run programs with one simple command.

## Usage

```bash
lang run examples/hello.lp    # run a program
lang examples/hello.lp        # same thing (shorthand)
lang check examples/hello.lp  # check for errors
lang --version
```

## Install

One line (installs `lang`, `langc`, `lang-lsp`, and the Cursor/VS Code extension):

```bash
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
```

## Package manager (future)

Long term, `lang` will also handle project init, dependencies, and publishing. For now it focuses on running `.lp` files.
