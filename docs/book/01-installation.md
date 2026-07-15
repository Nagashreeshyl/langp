# Installation

## Introduction

Before you write programs, you need the **Lang.P toolchain** on your computer. The toolchain includes:

| Tool | Purpose |
|------|---------|
| `lang` | Run and check programs; manage packages |
| `langc` | Compiler CLI (parse, check, build, debug AST) |
| `lang-lsp` | Language server for IDE features |
| VS Code / Cursor extension | Syntax highlighting, diagnostics, autocomplete |

**Why installation matters:** Without `lang`, your `.lp` files are just text. After installation, the computer can execute them.

**When to install:** Once per machine (or per development environment). Re-run the installer when upgrading to a new Lang.P release.

---

## Syntax

Installation is done from the terminal (command line), not in Lang.P code.

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.ps1 | iex
```

After install, reload your IDE so the extension activates.

---

## Examples

### Simple — verify install

```bash
lang --version
langc --version
```

Expected output includes `0.2.0` (or your installed version).

### Intermediate — run the hello example from a cloned repo

```bash
git clone https://github.com/Nagashreeshyl/langp.git
cd langp
./scripts/build-fast.sh
./target/release-fast/lang run examples/hello.lp
```

### Advanced — build from source (developers)

```bash
git clone https://github.com/Nagashreeshyl/langp.git
cd langp
cargo build --profile release-fast -p lang -p langc -p langp-lsp
export PATH="$PWD/target/release-fast:$PATH"
lang run examples/hello.lp
```

### Check your IDE

1. Open VS Code or Cursor.
2. Open any `.lp` file (for example `examples/hello.lp`).
3. Confirm syntax highlighting and that `lang check` runs on save (if configured).

Run project health check:

```bash
lang doctor
```

---

## Common mistakes

**Mistake:** Running `lang` before reloading the terminal or IDE after install.

**Why:** The shell may not see the new `PATH` entry.

**Fix:** Close and reopen the terminal, or run `source ~/.bashrc` / `source ~/.zshrc`.

---

**Mistake:** Using an old `lang` binary from a previous install.

**Why:** Multiple copies can exist (`~/.local/bin/lang`, project `target/release-fast/lang`).

**Fix:**

```bash
which lang
lang --version
```

Use the path that reports the version you expect.

---

## Best practices

- Pin your Lang.P version when sharing projects (`langp.toml` + `langp.lock`).
- Run `lang doctor` when something "used to work" after an upgrade.
- Prefer the official install script for releases; build from source for contributing.

---

## Exercises

### Beginner

1. Install Lang.P and print `lang --version`.
2. Run `lang run examples/hello.lp` from a cloned repository.
3. Run `lang check examples/hello.lp` and read the success message.
4. Open `hello.lp` in your IDE and identify three `@` comments.
5. Run `lang doctor` and list what it checks.

### Intermediate

1. Build Lang.P from source with `cargo build -p lang`.
2. Compare output of `target/debug/lang --version` vs system `lang --version`.
3. Install the VS Code extension from `editors/vscode-langp/` if not bundled.
4. Configure `.vscode/settings.json` to run `lang check` on save.
5. Explain the difference between `lang` and `langc`.

### Advanced

1. Read `scripts/install.sh` and list every binary it installs.
2. Set up `CARGO_TARGET_DIR=target` when using `build-fast.sh` (see script comments).
3. Document your install steps for teammates on macOS, Linux, and Windows.

---

## Summary

You installed the Lang.P toolchain: `lang`, `langc`, `lang-lsp`, and the IDE extension. You verified versions and ran your first checked program.

**Previous:** [00 — Preface](00-preface.md) · **Next:** [02 — Your First Program](02-your-first-program.md)

**See also:** [28 — Package Manager](28-package-manager.md), [30 — Debugging](30-debugging.md)
