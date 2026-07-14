#!/usr/bin/env sh
# Fix Lang.P IDE — colors, error squiggles, auto-indent (run from project root)
set -e

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
CURSOR="/Applications/Cursor.app/Contents/Resources/app/bin/cursor"
EXT_SRC="$ROOT/editors/vscode-langp"
VERSION="0.2.0"
VSIX="/tmp/langp-${VERSION}.vsix"
LSP="$HOME/.local/bin/lang-lsp"
LANG="$HOME/.local/bin/lang"

echo "=== Lang.P IDE Fix v${VERSION} ==="
echo ""

# Build extension
if command -v npm >/dev/null 2>&1 && [ ! -f "$EXT_SRC/langp-${VERSION}.vsix" ]; then
  echo "Building extension..."
  (cd "$EXT_SRC" && npm run compile && npm run package) || true
fi

VSIX="$EXT_SRC/langp-${VERSION}.vsix"
if [ ! -f "$VSIX" ]; then
  VSIX=$(ls "$EXT_SRC"/*.vsix 2>/dev/null | tail -1)
fi
cp "$VSIX" "/tmp/langp-${VERSION}.vsix" 2>/dev/null || true
VSIX="/tmp/langp-${VERSION}.vsix"

# Install to Cursor global extensions
if [ -x "$CURSOR" ] && [ -f "$VSIX" ]; then
  echo "Installing extension..."
  "$CURSOR" --install-extension "$VSIX" --force
fi

# Also copy unpacked into workspace (loads with this project)
WS_EXT="$ROOT/.vscode/extensions/Nagashreeshyl.langp-${VERSION}"
echo "Installing workspace extension to $WS_EXT ..."
rm -rf "$WS_EXT"
mkdir -p "$WS_EXT"
# Copy built extension files (exclude dev junk)
for item in package.json language-configuration.json icons snippets syntaxes out node_modules; do
  if [ -e "$EXT_SRC/$item" ]; then
    cp -R "$EXT_SRC/$item" "$WS_EXT/"
  fi
done

# Ensure lang tools exist
if [ ! -x "$LANG" ] || [ ! -x "$LSP" ]; then
  echo "Installing lang toolchain..."
  sh "$ROOT/scripts/install.sh"
fi

echo ""
echo "=== DONE ==="
echo ""
echo "IMPORTANT — do these 3 steps:"
echo "  1. Cmd+Q  (fully QUIT Cursor — not just close window)"
echo "  2. Reopen Cursor"
echo "  3. Open examples/hello.lp in the EDITOR tab (not the chat panel)"
echo ""
echo "Bottom-right of editor must say: Lang.P"
echo "If it says Plain Text, click it → choose Lang.P"
echo ""
echo "Test errors: delete a '.' from a line — red squiggle should appear."
