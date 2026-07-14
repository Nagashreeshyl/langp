#!/usr/bin/env sh
# Install Lang.P VS Code / Cursor extension (no `cursor` CLI required)
set -e

REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
VERSION="${LANGP_EXT_VERSION:-0.1.2}"
EXT_ID="Nagashreeshyl.langp-langp-${VERSION}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
VSIX_LOCAL="$ROOT/editors/vscode-langp/langp-langp-${VERSION}.vsix"

find_editor_cli() {
  for bin in \
    "$HOME/.cursor/bin/cursor" \
    "/Applications/Cursor.app/Contents/Resources/app/bin/cursor" \
    "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code" \
    "$(command -v cursor 2>/dev/null || true)" \
    "$(command -v code 2>/dev/null || true)"; do
    [ -n "$bin" ] && [ -x "$bin" ] && { echo "$bin"; return 0; }
  done
  return 1
}

install_unpacked() {
  src="$1"
  for ext_root in "$HOME/.cursor/extensions" "$HOME/.vscode/extensions"; do
    mkdir -p "$ext_root"
    rm -rf "$ext_root/$EXT_ID"
    cp -R "$src" "$ext_root/$EXT_ID"
    echo "  ✓ extension → $ext_root/$EXT_ID"
  done
}

install_from_vsix() {
  vsix="$1"
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT INT TERM
  unzip -q "$vsix" -d "$tmp"
  for ext_root in "$HOME/.cursor/extensions" "$HOME/.vscode/extensions"; do
    mkdir -p "$ext_root"
    rm -rf "$ext_root/$EXT_ID"
    mv "$tmp/extension" "$ext_root/$EXT_ID"
    echo "  ✓ extension → $ext_root/$EXT_ID"
  done
}

echo "Installing Lang.P editor extension..."

if CLI="$(find_editor_cli)"; then
  if [ -f "$VSIX_LOCAL" ]; then
    if "$CLI" --install-extension "$VSIX_LOCAL" --force 2>/dev/null; then
      echo "  ✓ installed via $CLI"
      exit 0
    fi
  fi
  VSIX_TMP="$(mktemp).vsix"
  if curl -fsSL "https://github.com/${REPO}/releases/latest/download/langp-langp-${VERSION}.vsix" -o "$VSIX_TMP" 2>/dev/null; then
    if "$CLI" --install-extension "$VSIX_TMP" --force 2>/dev/null; then
      rm -f "$VSIX_TMP"
      echo "  ✓ installed via $CLI (downloaded VSIX)"
      exit 0
    fi
  fi
  rm -f "$VSIX_TMP"
fi

if [ -f "$VSIX_LOCAL" ]; then
  install_from_vsix "$VSIX_LOCAL"
  exit 0
fi

EXT_SRC="$ROOT/editors/vscode-langp"
if [ -f "$EXT_SRC/package.json" ] && [ -f "$EXT_SRC/out/extension.js" ]; then
  install_unpacked "$EXT_SRC"
  exit 0
fi

if [ -f "$EXT_SRC/package.json" ] && command -v npm >/dev/null 2>&1; then
  echo "  Building extension..."
  (cd "$EXT_SRC" && npm install --silent && npm run compile --silent)
  install_unpacked "$EXT_SRC"
  exit 0
fi

echo "  ⚠ Could not install extension automatically."
echo "    Reload Cursor/VS Code after manual install from editors/vscode-langp/"
exit 0
