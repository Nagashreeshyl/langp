#!/usr/bin/env sh
# Copy Lang.P extensions into every VS Code-based IDE (no CLI — avoids Cursor segfault).
set -e

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
VERSION="${LANGP_EXT_VERSION:-0.2.8}"
GRAMMAR_ID="Nagashreeshyl.langp-grammar-${VERSION}"
SERVICES_ID="Nagashreeshyl.langp-${VERSION}"
GRAMMAR_SRC="$ROOT/editors/langp-grammar"
SERVICES_SRC="$ROOT/editors/vscode-langp"

EXT_ROOTS="
$HOME/.antigravity-ide/extensions
$HOME/.antigravity/extensions
$HOME/.cursor/extensions
$HOME/.vscode/extensions
$HOME/.windsurf/extensions
$HOME/.codeium/windsurf/extensions
"

remove_old_langp_extensions() {
  printf '%s\n' $EXT_ROOTS | while IFS= read -r ext_root; do
    [ -n "$ext_root" ] || continue
    parent="$(dirname "$ext_root")"
    [ -d "$parent" ] || [ -d "$ext_root" ] || continue
    for dir in "$ext_root"/[Nn]agashreeshyl.langp* "$ext_root"/[Nn]agashreeshyl.langp-grammar*; do
      [ -d "$dir" ] || continue
      case "$dir" in
        *"$GRAMMAR_ID"*|*"$SERVICES_ID"*) continue ;;
      esac
      rm -rf "$dir"
      echo "  removed old: $dir"
    done
    rm -f "$ext_root/extensions.json"
  done
}

copy_extension() {
  src="$1"
  id="$2"
  label="$3"
  if [ ! -f "$src/package.json" ]; then
    echo "  ✗ missing $src/package.json" >&2
    return 1
  fi
  printf '%s\n' $EXT_ROOTS | while IFS= read -r ext_root; do
    [ -n "$ext_root" ] || continue
    parent="$(dirname "$ext_root")"
    [ -d "$parent" ] || [ -d "$ext_root" ] || continue
    mkdir -p "$ext_root"
    dest="$ext_root/$id"
    rm -rf "$dest"
    cp -R "$src" "$dest"
    rm -f "$ext_root/extensions.json"
    echo "  ✓ $label → $dest"
  done
}

build_services() {
  if [ ! -f "$SERVICES_SRC/out/extension.js" ]; then
    if command -v npm >/dev/null 2>&1; then
      echo "Building Lang.P services extension..."
      (cd "$SERVICES_SRC" && npm install --silent && npm run compile)
    else
      echo "  ⚠ npm not found — services extension may be stale" >&2
    fi
  fi
}

install_workspace_extensions() {
  ws="$ROOT/.vscode/extensions"
  mkdir -p "$ws"
  rm -rf "$ws/$GRAMMAR_ID" "$ws/$SERVICES_ID"
  cp -R "$GRAMMAR_SRC" "$ws/$GRAMMAR_ID"
  cp -R "$SERVICES_SRC" "$ws/$SERVICES_ID"
  rm -rf "$ws/$SERVICES_ID/node_modules/.cache" 2>/dev/null || true
  echo "  ✓ workspace extensions → $ws"
}

echo "=== Lang.P IDE extensions v${VERSION} ==="
echo ""

build_services
remove_old_langp_extensions
copy_extension "$GRAMMAR_SRC" "$GRAMMAR_ID" "grammar (colors, icon, indent)"
copy_extension "$SERVICES_SRC" "$SERVICES_ID" "services (errors, autocomplete, LSP)"
install_workspace_extensions

echo ""
echo "=== DONE ==="
echo ""
echo "Next steps:"
echo "  1. Fully QUIT Antigravity / Cursor / VS Code (Cmd+Q)"
echo "  2. Reopen your IDE"
echo "  3. Open a .lp file in the EDITOR tab (not the AI chat panel)"
echo "  4. Bottom-right must say: Lang.P  (if Plain Text → Cmd+Shift+P → 'Lang.P: Set language mode')"
echo ""
echo "Optional: Command Palette → 'File Icon Theme' → 'Lang.P File Icons'"
echo ""
