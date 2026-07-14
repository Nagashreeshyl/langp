#!/usr/bin/env sh
# Install Lang.P VS Code / Cursor extension (no `cursor` CLI required)
set -e

REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
VERSION="${LANGP_EXT_VERSION:-0.1.3}"
EXT_ID="Nagashreeshyl.langp-langp-${VERSION}"

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
    rm -rf "$ext_root/$EXT_ID" "$ext_root/nagashreeshyl.langp-langp-${VERSION}"
    cp -R "$src" "$ext_root/$EXT_ID"
    echo "  ✓ extension → $ext_root/$EXT_ID"
  done
}

install_from_vsix() {
  vsix="$1"
  tmp="$(mktemp -d)"
  unzip -q "$vsix" -d "$tmp"
  for ext_root in "$HOME/.cursor/extensions" "$HOME/.vscode/extensions"; do
    mkdir -p "$ext_root"
    rm -rf "$ext_root/$EXT_ID" "$ext_root/nagashreeshyl.langp-langp-${VERSION}"
    cp -R "$tmp/extension" "$ext_root/$EXT_ID"
    echo "  ✓ extension → $ext_root/$EXT_ID"
  done
  rm -rf "$tmp"
}

resolve_release_tag() {
  tag="${LANGP_VERSION:-latest}"
  if [ "$tag" = "latest" ]; then
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
      | head -1)" || true
    [ -n "$tag" ] || tag="v0.1.2"
  fi
  echo "$tag"
}

echo "Installing Lang.P editor extension..."

RELEASE_TAG="$(resolve_release_tag)"
VSIX_URL="https://github.com/${REPO}/releases/download/${RELEASE_TAG}/langp-langp-${VERSION}.vsix"
VSIX_TMP="$(mktemp).vsix"

if CLI="$(find_editor_cli)"; then
  if curl -fsSL "$VSIX_URL" -o "$VSIX_TMP" 2>/dev/null; then
    if "$CLI" --install-extension "$VSIX_TMP" --force 2>/dev/null; then
      rm -f "$VSIX_TMP"
      echo "  ✓ installed via $CLI"
      exit 0
    fi
  fi
fi

if curl -fsSL "$VSIX_URL" -o "$VSIX_TMP" 2>/dev/null; then
  install_from_vsix "$VSIX_TMP"
  rm -f "$VSIX_TMP"
  echo "  ✓ extension installed (no CLI needed)"
  exit 0
fi
rm -f "$VSIX_TMP"

echo "  ⚠ Could not download VSIX from $VSIX_URL"
exit 0
