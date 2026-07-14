#!/usr/bin/env sh
# Fix Lang.P IDE — colors, error squiggles, extension (one command)
# Usage: ./scripts/fix-ide.sh
set -e

REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
VERSION="${LANGP_EXT_VERSION:-0.1.3}"
INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
CURSOR="/Applications/Cursor.app/Contents/Resources/app/bin/cursor"

echo "Lang.P IDE fix"
echo ""

# 1. Ensure lang-lsp is installed
if [ ! -x "$INSTALL_DIR/lang-lsp" ]; then
  echo "Installing lang-lsp..."
  sh "$(dirname "$0")/install.sh"
else
  echo "✓ lang-lsp found at $INSTALL_DIR/lang-lsp"
fi

# 2. Download VSIX (Cursor cannot install from URL directly)
VSIX="/tmp/langp-langp-${VERSION}.vsix"
TAG="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)"
[ -n "$TAG" ] || TAG="v0.1.2"

echo "Downloading extension ${VERSION}..."
if [ -f "$(dirname "$0")/../editors/vscode-langp/langp-langp-${VERSION}.vsix" ]; then
  cp "$(dirname "$0")/../editors/vscode-langp/langp-langp-${VERSION}.vsix" "$VSIX"
else
  curl -fsSL "https://github.com/${REPO}/releases/download/${TAG}/langp-langp-${VERSION}.vsix" -o "$VSIX" \
    || curl -fsSL "https://github.com/${REPO}/releases/download/v0.1.2/langp-langp-0.1.2.vsix" -o "$VSIX"
fi

# 3. Install extension via local VSIX path (NOT URL)
if [ -x "$CURSOR" ]; then
  "$CURSOR" --install-extension "$VSIX" --force
  echo "✓ Extension installed via Cursor"
else
  sh "$(dirname "$0")/install-extension.sh"
fi

# 4. Write workspace settings with absolute lang-lsp path
LSP="$INSTALL_DIR/lang-lsp"
SETTINGS="$(dirname "$0")/../.vscode/settings.json"
mkdir -p "$(dirname "$SETTINGS")"
cat > "$SETTINGS" <<EOF
{
  "files.associations": { "*.lp": "langp" },
  "[langp]": {
    "editor.tabSize": 4,
    "editor.insertSpaces": true,
    "editor.autoIndent": "full",
    "editor.renderValidationDecorations": "on",
    "editor.quickSuggestions": { "other": "on", "comments": "off", "strings": "on" }
  },
  "langp.languageServerPath": "$LSP",
  "langp.enableLanguageServer": true
}
EOF
echo "✓ Workspace settings updated"

echo ""
echo "Done! Now:"
echo "  1. Press Cmd+Q to fully quit Cursor"
echo "  2. Reopen this project"
echo "  3. Open any .lp file — you should see colors + red/yellow error lines"
echo ""
echo "Test error squiggles: remove a '.' from a line and save."
