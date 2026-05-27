#!/bin/sh

# strict mode
set -euo pipefail

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo DEFAULT Rust TOOLCHAIN

echo
echo "DOC"
cargo doc --no-deps --quiet

echo
echo "CARGO TEST (debug)"
cargo test
echo
echo "CARGO TEST (release)"
cargo test --release

echo
echo "CARGO CHECK --TESTS (debug, feature lint_unused_unsafe)"
cargo check --tests --features lint_unused_unsafe
# - feature "lint_unused_unsafe" can't be tested/built, only checked; and not in release, only in
#   debug

echo
echo "CARGO CHECK --TESTS (negative_tests/verify_error_messages, debug, feature unused_lint)"
cd negative_tests/verify_error_messages
cargo check --tests --features lint_unused_unsafe
cd - >/dev/null
