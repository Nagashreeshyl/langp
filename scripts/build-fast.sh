#!/usr/bin/env sh
# Fast local build — use this instead of plain `cargo build` during development.
set -e
cd "$(dirname "$0")/.."
cargo build --profile release-fast -p langc "$@"
echo ""
echo "✓ $(pwd)/target/release-fast/langc"
echo "  langc run examples/hello.lp"
