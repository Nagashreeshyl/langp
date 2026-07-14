#!/usr/bin/env sh
# Lang.P — one-line install (toolchain + IDE extension)
# Usage: curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
set -e

VERSION="${LANGP_VERSION:-latest}"
REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
CARGO_INSTALL="${LANGP_FROM_SOURCE:-0}"
SKIP_EXT="${LANGP_SKIP_EXTENSION:-0}"

echo "╔══════════════════════════════════════╗"
echo "║         Lang.P installer             ║"
echo "╚══════════════════════════════════════╝"
echo "  install dir: $INSTALL_DIR"
echo ""

mkdir -p "$INSTALL_DIR"

detect_platform() {
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  ARCH="$(uname -m)"
  case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
  esac
  case "$OS" in
    darwin) PLATFORM="apple-darwin" ;;
    linux) PLATFORM="unknown-linux-gnu" ;;
    *) echo "Unsupported OS: $OS" >&2; exit 1 ;;
  esac
  echo "${ARCH}-${PLATFORM}"
}

install_from_source() {
  SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
  ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
  echo "Building lang, langc, and lang-lsp from source..."
  if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust/cargo required. Install from https://rustup.rs" >&2
    exit 1
  fi
  (cd "$ROOT" && cargo build --profile release-fast -p lang -p langc -p langp-lsp)
  cp "$ROOT/target/release-fast/lang" "$INSTALL_DIR/lang"
  cp "$ROOT/target/release-fast/langc" "$INSTALL_DIR/langc"
  cp "$ROOT/target/release-fast/lang-lsp" "$INSTALL_DIR/lang-lsp"
}

download_binary() {
  name="$1"
  TRIPLE="$2"
  URL="https://github.com/${REPO}/releases/${VERSION}/download/${name}-${TRIPLE}"
  TMP="$(mktemp)"
  if curl -fsSL "$URL" -o "$TMP" 2>/dev/null; then
    chmod +x "$TMP"
    mv "$TMP" "$INSTALL_DIR/$name"
    return 0
  fi
  rm -f "$TMP"
  return 1
}

if [ "$CARGO_INSTALL" = "1" ] || ! command -v curl >/dev/null 2>&1; then
  install_from_source
else
  TRIPLE="$(detect_platform)"
  echo "Downloading binaries for $TRIPLE..."
  ok=1
  for bin in lang langc lang-lsp; do
    if download_binary "$bin" "$TRIPLE"; then
      echo "  ✓ $bin"
    else
      ok=0
      break
    fi
  done
  if [ "$ok" = "0" ]; then
    echo "Pre-built binaries not found; building from source..."
    install_from_source
  fi
fi

if [ "$SKIP_EXT" != "1" ]; then
  EXT_SCRIPT="$(mktemp)"
  if curl -fsSL "https://raw.githubusercontent.com/${REPO}/main/scripts/install-extension.sh" -o "$EXT_SCRIPT" 2>/dev/null; then
    sh "$EXT_SCRIPT" || true
    rm -f "$EXT_SCRIPT"
  else
    SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd)" || SCRIPT_DIR=""
    if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/install-extension.sh" ]; then
      sh "$SCRIPT_DIR/install-extension.sh" || true
    fi
  fi
fi

echo ""
echo "✓ lang      → $INSTALL_DIR/lang"
echo "✓ langc     → $INSTALL_DIR/langc"
echo "✓ lang-lsp  → $INSTALL_DIR/lang-lsp"
echo ""
if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
  echo "Add to PATH (add this to ~/.zshrc):"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
fi
echo "Run a program:"
echo "  lang run examples/hello.lp"
echo "  lang examples/hello.lp"
echo ""
echo "Reload Cursor/VS Code to activate the Lang.P extension."
