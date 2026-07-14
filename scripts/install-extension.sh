#!/usr/bin/env sh
# Install Lang.P extensions into Cursor, Antigravity, VS Code (no CLI required).
set -e

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
export LANGP_EXT_VERSION="${LANGP_EXT_VERSION:-0.2.2}"

if [ -d "$ROOT/editors/langp-grammar" ]; then
  sh "$ROOT/scripts/install-ide-extensions.sh"
  exit 0
fi

# Fallback when run from curl|sh without full repo — download release VSIX
REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
VERSION="${LANGP_EXT_VERSION}"
RELEASE_TAG="${LANGP_VERSION:-latest}"

if [ "$RELEASE_TAG" = "latest" ]; then
  RELEASE_TAG="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
    | head -1)" || true
  [ -n "$RELEASE_TAG" ] || RELEASE_TAG="v0.2.1"
fi

VSIX_URL="https://github.com/${REPO}/releases/download/${RELEASE_TAG}/langp-${VERSION}.vsix"
VSIX_TMP="$(mktemp).vsix"

echo "Installing Lang.P editor extension from release..."
if ! curl -fsSL "$VSIX_URL" -o "$VSIX_TMP" 2>/dev/null; then
  echo "  ⚠ Could not download $VSIX_URL"
  exit 0
fi

tmp="$(mktemp -d)"
unzip -q "$VSIX_TMP" -d "$tmp"
EXT_ID="Nagashreeshyl.langp-${VERSION}"

for ext_root in \
  "$HOME/.cursor/extensions" \
  "$HOME/.vscode/extensions" \
  "$HOME/.antigravity/extensions" \
  "$HOME/.antigravity-ide/extensions"; do
  parent="$(dirname "$ext_root")"
  if [ -d "$parent" ] || [ "$ext_root" = "$HOME/.cursor/extensions" ]; then
    mkdir -p "$ext_root"
    rm -rf "$ext_root/$EXT_ID"
    cp -R "$tmp/extension" "$ext_root/$EXT_ID"
    rm -f "$ext_root/extensions.json"
    echo "  ✓ → $ext_root/$EXT_ID"
  fi
done

rm -rf "$tmp" "$VSIX_TMP"
echo "  ✓ extension installed — restart your IDE"
