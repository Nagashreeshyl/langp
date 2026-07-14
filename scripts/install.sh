#!/usr/bin/env sh
# Lang.P installer — macOS and Linux
# Usage: curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
set -e

VERSION="${LANGP_VERSION:-latest}"
REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
CARGO_INSTALL="${LANGP_FROM_SOURCE:-0}"

echo "Lang.P installer"
echo "  install dir: $INSTALL_DIR"

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
    *) echo "Unsupported OS: $OS (build from source with: cargo build --release -p langc)" >&2; exit 1 ;;
  esac
  echo "${ARCH}-${PLATFORM}"
}

if [ "$CARGO_INSTALL" = "1" ] || ! command -v curl >/dev/null 2>&1; then
  echo "Building langc from source (release-fast)..."
  if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust/cargo required. Install from https://rustup.rs" >&2
    exit 1
  fi
  SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
  ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
  (cd "$ROOT" && cargo build --profile release-fast -p langc)
  cp "$ROOT/target/release-fast/langc" "$INSTALL_DIR/langc"
else
  TRIPLE="$(detect_platform)"
  URL="https://github.com/${REPO}/releases/${VERSION}/download/langc-${TRIPLE}"
  TMP="$(mktemp)"
  echo "Downloading langc for $TRIPLE..."
  if curl -fsSL "$URL" -o "$TMP" 2>/dev/null; then
    chmod +x "$TMP"
    mv "$TMP" "$INSTALL_DIR/langc"
  else
    echo "Pre-built binary not found; building from source..."
    SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
    ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
    (cd "$ROOT" && cargo build --profile release-fast -p langc)
    cp "$ROOT/target/release-fast/langc" "$INSTALL_DIR/langc"
  fi
fi

echo ""
echo "✓ langc installed to $INSTALL_DIR/langc"
echo ""
if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
  echo "Add to PATH:"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
fi
echo "Try:"
echo "  langc run examples/hello.lp"
echo "  langc --version"
