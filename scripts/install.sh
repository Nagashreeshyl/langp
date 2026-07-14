#!/usr/bin/env sh
# Lang.P — one-line install (toolchain + IDE extension)
# Usage: curl -fsSL https://raw.githubusercontent.com/Nagashreeshyl/langp/main/scripts/install.sh | sh
set -e

VERSION="${LANGP_VERSION:-latest}"
REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
INSTALL_DIR="${LANGP_INSTALL_DIR:-$HOME/.local/bin}"
CARGO_INSTALL="${LANGP_FROM_SOURCE:-0}"
SKIP_EXT="${LANGP_SKIP_EXTENSION:-0}"
CACHE_DIR="${LANGP_CACHE_DIR:-$HOME/.cache/langp-src}"

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

# When run via `curl | sh`, $0 is "sh" — detect repo root safely.
find_local_repo() {
  case "$0" in
    sh|bash|dash) return 1 ;;
  esac
  script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)" || return 1
  root="$(CDPATH= cd -- "$script_dir/.." && pwd)" || return 1
  if [ -f "$root/Cargo.toml" ] && [ -d "$root/langc" ]; then
    echo "$root"
    return 0
  fi
  return 1
}

resolve_release_tag() {
  tag="$VERSION"
  if [ "$tag" = "latest" ]; then
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
      | head -1)" || true
    if [ -z "$tag" ]; then
      tag="v0.1.2"
    fi
  fi
  echo "$tag"
}

install_from_source() {
  echo "Building lang, langc, and lang-lsp from source..."
  if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust/cargo required. Install from https://rustup.rs" >&2
    exit 1
  fi

  if root="$(find_local_repo)"; then
    echo "  Using local repo: $root"
  else
    echo "  Cloning from GitHub into $CACHE_DIR ..."
    if ! command -v git >/dev/null 2>&1; then
      echo "git required for source install. Install git or wait for release binaries." >&2
      exit 1
    fi
    if [ -d "$CACHE_DIR/.git" ]; then
      git -C "$CACHE_DIR" fetch --depth 1 origin main 2>/dev/null || true
      git -C "$CACHE_DIR" reset --hard origin/main 2>/dev/null || true
    else
      rm -rf "$CACHE_DIR"
      git clone --depth 1 "https://github.com/${REPO}.git" "$CACHE_DIR"
    fi
    root="$CACHE_DIR"
  fi

  (cd "$root" && cargo build --profile release-fast -p lang -p langc -p langp-lsp)
  cp "$root/target/release-fast/lang" "$INSTALL_DIR/lang"
  cp "$root/target/release-fast/langc" "$INSTALL_DIR/langc"
  cp "$root/target/release-fast/lang-lsp" "$INSTALL_DIR/lang-lsp"
}

download_binary() {
  name="$1"
  triple="$2"
  tag="$3"
  url="https://github.com/${REPO}/releases/download/${tag}/${name}-${triple}"
  tmp="$(mktemp)"
  if curl -fsSL "$url" -o "$tmp" 2>/dev/null; then
    chmod +x "$tmp"
    mv "$tmp" "$INSTALL_DIR/$name"
    return 0
  fi
  rm -f "$tmp"
  return 1
}

if [ "$CARGO_INSTALL" = "1" ] || ! command -v curl >/dev/null 2>&1; then
  install_from_source
else
  TRIPLE="$(detect_platform)"
  TAG="$(resolve_release_tag)"
  echo "Downloading binaries for $TRIPLE (release $TAG)..."
  ok=1
  for bin in lang langc lang-lsp; do
    if download_binary "$bin" "$TRIPLE" "$TAG"; then
      echo "  ✓ $bin"
    else
      ok=0
      break
    fi
  done
  if [ "$ok" = "0" ]; then
    echo "Pre-built binaries not found for $TAG; building from source..."
    install_from_source
  fi
fi

if [ "$SKIP_EXT" != "1" ]; then
  echo ""
  echo "Installing Lang.P IDE extensions (colors, autocomplete)..."
  EXT_SCRIPT="$(mktemp)"
  if curl -fsSL "https://raw.githubusercontent.com/${REPO}/main/scripts/install-extension.sh" -o "$EXT_SCRIPT" 2>/dev/null; then
    sh "$EXT_SCRIPT" || {
      echo "  ⚠ Extension install failed — run manually after clone:" >&2
      echo "    curl -fsSL https://raw.githubusercontent.com/${REPO}/main/scripts/install-extension.sh | sh" >&2
    }
    rm -f "$EXT_SCRIPT"
  elif root="$(find_local_repo)" && [ -f "$root/scripts/install-extension.sh" ]; then
    sh "$root/scripts/install-extension.sh" || true
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
echo ""
echo "Activate IDE colors:"
echo "  1. Fully QUIT Antigravity / Cursor / VS Code (Cmd+Q)"
echo "  2. Reopen and open a .lp file in the editor tab"
echo "  3. Bottom-right should say Lang.P (not Plain Text)"
echo "  If still Plain Text: Cmd+Shift+P → 'Lang.P: Set language mode'"
echo ""
echo "Or run from a cloned repo: ./scripts/fix-ide.sh"
