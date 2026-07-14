# Chapter 21 — Tooling

## 21.1 Overview

Lang.P tooling is a first-class part of the ecosystem. The IDE teaches the language; tools make development productive from day one.

| Tool | Binary | Purpose |
|------|--------|---------|
| Compiler | `langc` | Compile `.lp` to native code or bytecode |
| Package Manager | `lang` | Dependencies, build, test, publish |
| Language Server | `lang-lsp` | LSP for IDE integration |
| IDE | Lang Studio | Official integrated development environment |
| Formatter | `langfmt` | Code formatting (part of lang-lsp) |
| REPL | `lang-repl` | Interactive interpreter |
| Debugger | `lang-debug` | Debug adapter protocol |

## 21.2 Compiler (`langc`)

### 21.2.1 Usage

```bash
langc main.lp                    @ Compile to executable
langc main.lp -o myapp           @ Specify output name
langc main.lp --mode interpret   @ Run via interpreter
langc main.lp --mode compile     @ Compile to native (default)
langc main.lp --mode bytecode    @ Compile to bytecode
langc main.lp --mode check       @ Type-check only
langc main.lp --emit ast         @ Dump AST
langc main.lp --emit ir          @ Dump IR
langc main.lp --emit tokens      @ Dump tokens
```

### 21.2.2 Flags

| Flag | Description |
|------|-------------|
| `-o <file>` | Output file path |
| `--mode <mode>` | `compile`, `interpret`, `bytecode`, `check` |
| `--emit <target>` | `ast`, `ir`, `tokens`, `llvm` |
| `-O <level>` | Optimization level (0-3) |
| `-g` | Debug symbols |
| `--warn <level>` | Warning level: `off`, `default`, `pedantic` |
| `--target <triple>` | Target platform (cross-compilation) |
| `--stdlib <path>` | Custom stdlib path |
| `--version` | Print version |
| `--help` | Print help |

### 21.2.3 Error Output

```
error[E0301]: type mismatch
  --> src/main.lp:5:5
   |
 5 |     result: String = 42.
   |                      ^^ expected String, found Int
   |
  = help: use to_string(42) to convert

error: compilation failed with 1 error
```

Colors enabled by default; `--color=never` to disable.

## 21.3 Language Server (`lang-lsp`)

### 21.3.1 Capabilities

| Feature | LSP Method | Status |
|---------|-----------|--------|
| Diagnostics | `textDocument/publishDiagnostics` | v0.1 |
| Autocomplete | `textDocument/completion` | v0.1 |
| Hover documentation | `textDocument/hover` | v0.1 |
| Go to definition | `textDocument/definition` | v0.1 |
| Find references | `textDocument/references` | v0.1 |
| Rename symbol | `textDocument/rename` | v0.1 |
| Formatting | `textDocument/formatting` | v0.1 |
| Semantic highlighting | `textDocument/semanticTokens` | v0.1 |
| Signature help | `textDocument/signatureHelp` | v0.1 |
| Code actions | `textDocument/codeAction` | v0.1 |
| Inlay hints | `textDocument/inlayHint` | v0.2 |
| Folding ranges | `textDocument/foldingRange` | v0.2 |
| Document symbols | `textDocument/documentSymbol` | v0.1 |

### 21.3.2 Input Type Quick-Fix

When the compiler emits warning `W0101` (input type could be more specific), the language server MUST offer a code action to convert generic `input` to typed `input`:

**Before:**

```lp
age = input "Age : ".
print age + 1.
```

**Diagnostic:**

```
warning[W0101]: input type could be more specific
  --> main.lp:1:7
   |
 1 | age = input "Age : ".
   |       ^^^^^^^^^^^^^^ the value "age" appears to be used as a number
   |
  = help: consider using: age = input number "Age : ".
```

**Quick-fix action:** `Convert to typed input (number)`

**After:**

```lp
age = input number "Age : ".
print age + 1.
```

The quick-fix MUST:

1. Insert the correct input type keyword (`text`, `number`, `decimal`, `boolean`, `password`, `file`, `folder`, `date`, or `color`) based on inferred usage.
2. Preserve the prompt string exactly.
3. Suppress `W0101` after application.

Lang Studio displays this as a lightbulb action and inline "Fix in Lang Studio" link on the warning underline.

Additional input-related code actions (v0.2):

| Action | Description |
|--------|-------------|
| `Add explicit type annotation` | Add `: Int` etc. instead of input type keyword |
| `Wrap in try/catch for InputCancelledError` | For picker-based input expressions |

### 21.3.3 Configuration

```json
{
    "langp.lsp.path": "/usr/local/bin/lang-lsp",
    "langp.lsp.trace": "off",
    "langp.format.indentSize": 4,
    "langp.format.maxLineLength": 100,
    "langp.inlayHints.enabled": true,
    "langp.diagnostics.enabled": true
}
```

## 21.4 Lang Studio (IDE)

### 21.4.1 Features

| Feature | Description |
|---------|-------------|
| **Auto indentation** | Typing `,` indents; typing `..` dedents |
| **Syntax highlighting** | Full semantic highlighting via LSP |
| **Autocomplete** | Context-aware suggestions with documentation |
| **Hover documentation** | Type info, docs, and examples on hover |
| **Go to definition** | Jump to symbol definition |
| **Rename symbol** | Refactor names across project |
| **Formatting** | Format on save, format selection |
| **Debugger** | Breakpoints, step, watch, call stack |
| **Profiler** | CPU and memory profiling |
| **Package manager** | GUI for `lang add/remove/update` |
| **AI Assistant** | Built-in AI chat and code generation |
| **Visual Browser Designer** | Drag-and-drop browser UI builder |
| **Live Preview** | Run and preview in IDE |
| **Integrated terminal** | Built-in terminal |
| **Integrated documentation** | Browse stdlib docs in IDE |
| **Project templates** | One-click project creation |
| **Comment toggle** | Show/hide `@` comments in generated code |
| **Error explanations** | AI-powered error fix suggestions |

### 21.4.2 Auto Indentation Behavior

When the user types `,` at the end of a block header:

```
if age >= 18,█
```

The IDE automatically transforms to:

```
if age >= 18,
    █
```

When the user types `..`:

```
if age >= 18,
    print "Adult".
    ..█
```

The IDE dedents:

```
if age >= 18,
    print "Adult".
..█
```

### 21.4.3 Visual Browser Designer

A drag-and-drop interface for building Navigator browser UIs:

- Component palette (toolbar, tabs, sidebar, address bar)
- Property inspector for selected components
- Live preview of browser chrome
- Generates commented Lang.P source code
- Theme editor with color picker

### 21.4.4 AI Integration

Lang Studio's AI assistant can:

- Explain selected code in plain language
- Generate code from `@` comments
- Fix compiler errors
- Suggest completions
- Generate tests
- Answer Lang.P language questions

## 21.5 Formatter (`langfmt`)

Enforces consistent style:

```bash
langfmt main.lp              @ Format file in place
langfmt --check main.lp      @ Check formatting without changes
langfmt --diff main.lp       @ Show formatting diff
```

Formatting rules:

- 4-space indentation
- One statement per line
- Blank line between top-level declarations
- No trailing whitespace
- Maximum line length: 100 characters (soft wrap)
- Block opener `,` at end of header line
- Block closer `..` at parent indentation level

## 21.6 REPL (`lang-repl`)

Interactive interpreter:

```bash
lang-repl
```

```
Lang.P v0.1.0
>>> name = "Naga".
>>> print "Hello " with name.
Hello Naga
>>> function greet(n), print "Hi " with n. .
>>> greet("World").
Hi World
>>> :type name
String
>>> :help
>>> :exit
```

REPL commands:

| Command | Description |
|---------|-------------|
| `:type <expr>` | Show type of expression |
| `:ast <expr>` | Show AST |
| `:help` | Show help |
| `:load <file>` | Load and execute file |
| `:reset` | Reset environment |
| `:exit` | Exit REPL |

## 21.7 Debugger (`lang-debug`)

Debug Adapter Protocol (DAP) compatible:

```bash
lang-debug main.lp
```

Features:

- Breakpoints (line, conditional, logpoint)
- Step over, step into, step out, continue
- Variable inspection and modification
- Watch expressions
- Call stack navigation
- Exception breakpoints

## 21.8 Profiler

```bash
langc main.lp --profile cpu.
langc main.lp --profile memory.
```

Output formats: text, flamegraph (HTML), JSON.

## 21.9 Editor Extensions

Official extensions for:

| Editor | Extension |
|--------|-----------|
| Lang Studio | Built-in |
| VS Code | `langp-langp` |
| Cursor | `langp-langp` |
| Neovim | `langp.nvim` |
| JetBrains | `langp-plugin` |

All extensions use `lang-lsp` for language features.

## 21.10 Documentation Generator

```bash
lang doc                     @ Generate docs for current project
lang doc --output docs/      @ Specify output directory
```

Generates HTML documentation from `@` doc comments and type signatures.

Doc comment syntax:

```lp
@ Creates a new user with the given name and age.
@ Returns a User object ready for use.
@
@ Example:
@   user = create_user("Naga", 25).
@   print user.name.
function create_user(name: String, age: Int) -> User,
    @ ...
.
```

## 21.11 Continuous Integration

Recommended CI pipeline:

```yaml
@ .github/workflows/ci.yml (conceptual)
steps:
  - lang install
  - lang build
  - lang test
  - langfmt --check .
  - langc main.lp --mode check
```

## 21.12 Toolchain Installation

```bash
@ Install entire toolchain
curl -fsSL https://langp.dev/install | sh

@ Or via package manager
brew install langp          @ macOS
scoop install langp         @ Windows
apt install langp           @ Debian/Ubuntu
```

Installed tools: `langc`, `lang`, `lang-lsp`, `langfmt`, `lang-repl`, `lang-debug`.

Lang Studio is installed separately (desktop application).
