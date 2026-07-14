#!/usr/bin/env sh
# Install Lang.P extensions into Cursor, Antigravity, VS Code (no CLI required).
set -e

REPO="${LANGP_REPO:-Nagashreeshyl/langp}"
CACHE_DIR="${LANGP_CACHE_DIR:-$HOME/.cache/langp-src}"
export LANGP_EXT_VERSION="${LANGP_EXT_VERSION:-0.2.4}"

find_local_repo() {
  case "$0" in
    sh|bash|dash) return 1 ;;
  esac
  script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)" || return 1
  root="$(CDPATH= cd -- "$script_dir/.." && pwd)" || return 1
  if [ -f "$root/Cargo.toml" ] && [ -d "$root/editors/langp-grammar" ]; then
    echo "$root"
    return 0
  fi
  return 1
}

ensure_repo() {
  if root="$(find_local_repo)"; then
    echo "$root"
    return 0
  fi

  echo "Fetching Lang.P editor extensions from GitHub..."
  if ! command -v git >/dev/null 2>&1; then
    echo "  ✗ git is required to install IDE extensions." >&2
    echo "    Install git, or clone https://github.com/${REPO} and run:" >&2
    echo "    ./scripts/fix-ide.sh" >&2
    return 1
  fi

  if [ -d "$CACHE_DIR/.git" ]; then
    git -C "$CACHE_DIR" fetch --depth 1 origin main 2>/dev/null || true
    git -C "$CACHE_DIR" reset --hard origin/main 2>/dev/null || true
  else
    rm -rf "$CACHE_DIR"
    git clone --depth 1 "https://github.com/${REPO}.git" "$CACHE_DIR"
  fi

  if [ ! -d "$CACHE_DIR/editors/langp-grammar" ]; then
    echo "  ✗ editors/langp-grammar missing in cloned repo" >&2
    return 1
  fi

  echo "$CACHE_DIR"
}

root="$(ensure_repo)" || exit 1
sh "$root/scripts/install-ide-extensions.sh"
