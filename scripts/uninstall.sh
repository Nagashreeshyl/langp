#!/usr/bin/env sh
# Lang.P uninstall — one line:
#   curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.sh | sh
set -e

INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
EXT_VERSION="${LANGP_EXT_VERSION:-0.1.2}"
EXT_ID="Nagashreeshyl.langp-langp-${EXT_VERSION}"

echo "Lang.P uninstaller"
echo ""

removed=0

for bin in lang langc lang-lsp; do
  if [ -f "$INSTALL_DIR/$bin" ]; then
    rm -f "$INSTALL_DIR/$bin"
    echo "  ✓ removed $INSTALL_DIR/$bin"
    removed=1
  fi
done

for ext_root in "$HOME/.cursor/extensions" "$HOME/.vscode/extensions"; do
  for dir in "$ext_root/$EXT_ID" "$ext_root/nagashreeshyl.langp-langp-0.1.0" "$ext_root/nagashreeshyl.langp-langp-0.1.1"; do
    if [ -d "$dir" ]; then
      rm -rf "$dir"
      echo "  ✓ removed extension $dir"
      removed=1
    fi
  done
done

if [ "$removed" = "0" ]; then
  echo "  Nothing to remove — Lang.P may not be installed."
else
  echo ""
  echo "✓ Lang.P uninstalled."
  echo "  Reload Cursor/VS Code to complete removal."
fi
