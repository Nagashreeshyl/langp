#!/usr/bin/env sh
# Fast local build — use this instead of plain `cargo build` during development.
set -e
cd "$(dirname "$0")/.."
cargo build --profile release-fast -p lang -p langc -p langp-lsp "$@"
echo ""
echo "✓ $(pwd)/target/release-fast/lang"
echo "✓ $(pwd)/target/release-fast/langc"
echo "✓ $(pwd)/target/release-fast/lang-lsp"
echo "  lang run examples/hello.lp"
