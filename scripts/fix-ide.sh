#!/usr/bin/env sh
# Fix Lang.P in Cursor, Antigravity, VS Code — no broken `cursor` CLI.
set -e

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
LANG="$HOME/.local/bin/lang"
LSP="$HOME/.local/bin/lang-lsp"

echo "=== Lang.P IDE Fix ==="
echo ""

if [ ! -x "$LANG" ] || [ ! -x "$LSP" ]; then
  echo "Installing lang toolchain..."
  sh "$ROOT/scripts/install.sh" || true
fi

export LANGP_EXT_VERSION="${LANGP_EXT_VERSION:-0.2.8}"
sh "$ROOT/scripts/install-ide-extensions.sh"
