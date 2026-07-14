# Lang.P for VS Code / Cursor

Official editor extension — syntax highlighting, file icon, snippets, autocomplete, diagnostics, hover, and go-to-definition.

## Features

| Feature | Status |
|---------|--------|
| `.lp` file association | ✅ |
| Language icon in explorer | ✅ |
| Syntax highlighting | ✅ |
| Snippets (`function`, `if`, `for`, …) | ✅ |
| Auto-indent on `,` / dedent on `..` | ✅ |
| Autocomplete (keywords + symbols) | ✅ via LSP |
| Error squiggles | ✅ via LSP |
| Hover docs | ✅ via LSP |
| Go to definition | ✅ via LSP |
| Outline / document symbols | ✅ via LSP |

## Prerequisites

Install the Lang.P toolchain (includes `lang-lsp`):

```bash
curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
```

Ensure `lang-lsp` is on your PATH:

```bash
lang-lsp --help  # or which lang-lsp
```

## Install in Cursor / VS Code

### Option A — From source (recommended for now)

```bash
cd editors/vscode-langp
npm install
npm run compile
npm run package
```

Then in Cursor/VS Code:

1. Open Command Palette → **Extensions: Install from VSIX…**
2. Select `langp-langp-0.1.0.vsix`

Or:

```bash
code --install-extension langp-langp-0.1.0.vsix
cursor --install-extension langp-langp-0.1.0.vsix
```

### Option B — Open workspace folder

Open any `.lp` file — Cursor/VS Code will prompt to install recommended extensions once published to the marketplace.

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `langp.languageServerPath` | `lang-lsp` | Path to language server binary |
| `langp.enableLanguageServer` | `true` | Toggle LSP features |

## Manual LSP setup (without extension)

Add to `.vscode/settings.json`:

```json
{
  "files.associations": { "*.lp": "langp" },
  "[langp]": { "editor.tabSize": 4, "editor.insertSpaces": true }
}
```

And use a generic LSP client extension pointed at `lang-lsp`.

## Publish to Marketplace

```bash
npm install -g @vscode/vsce
cd editors/vscode-langp
vsce publish
```

Publisher: `Nagashreeshyl` (requires VS Code Marketplace account).
