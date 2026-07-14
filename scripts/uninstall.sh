#!/usr/bin/env sh
# Lang.P uninstall — one line:
#   curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/uninstall.sh | sh
set -e

INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${LANGP_EXT_VERSION:-0.2.4}"

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

for ext_root in \
  "$HOME/.antigravity-ide/extensions" \
  "$HOME/.antigravity/extensions" \
  "$HOME/.cursor/extensions" \
  "$HOME/.vscode/extensions" \
  "$HOME/Library/Application Support/Antigravity IDE/User/extensions" \
  "$HOME/Library/Application Support/Antigravity/User/extensions" \
  "$HOME/Library/Application Support/Cursor/User/extensions" \
  "$HOME/Library/Application Support/Code/User/extensions"; do
  [ -d "$ext_root" ] || continue
  for dir in "$ext_root"/[Nn]agashreeshyl.langp* "$ext_root"/[Nn]agashreeshyl.langp-grammar*; do
    [ -d "$dir" ] || continue
    rm -rf "$dir"
    echo "  ✓ removed extension $dir"
    removed=1
  done
  rm -f "$ext_root/extensions.json"
done

if [ "$removed" = "0" ]; then
  echo "  Nothing to remove — Lang.P may not be installed."
else
  echo ""
  echo "✓ Lang.P uninstalled."
  echo "  Fully quit and reopen your IDE to complete removal."
fi
