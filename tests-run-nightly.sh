#!/bin/sh

# strict mode
set -euo pipefail

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo NIGHTLY Rust TOOLCHAIN

echo
echo "CLIPPY"
cargo +nightly clippy -- -D warnings
echo
echo "CLIPPY (feature lint_unused_unsafe)"
cargo +nightly clippy --features lint_unused_unsafe -- -D warnings

echo
echo "FMT"
cargo +nightly fmt --check

echo
echo "FMT: negative_tests/verify_error_messages/"
cd negative_tests/verify_error_messages/
cargo +nightly fmt --check
cd - >/dev/null

echo
echo "FMT:negative_tests/sneaky_unsafe_stops_compilation/"
cd negative_tests/sneaky_unsafe_stops_compilation/
cargo +nightly fmt --check
cd - >/dev/null

echo
echo "FMT: negative_tests/unused_unsafe_fails_lint/"
cd negative_tests/unused_unsafe_fails_lint/
cargo +nightly fmt --check
cd - >/dev/null

echo
echo "DOC"
RUSTDOCFLAGS="--forbid rustdoc::invalid_codeblock_attributes \
  --forbid rustdoc::missing_doc_code_examples \
  -Zcrate-attr=feature(rustdoc_missing_doc_code_examples)" \
  cargo +nightly doc --no-deps --quiet

echo
echo "CARGO TEST (debug)"
# We need "cargo +nightly test" to validate error numbers. To locate them, search for
# "compile_fail,E" in src/lib.rs.
cargo +nightly test
echo
echo "CARGO TEST (release)"
cargo +nightly test --release

# Features "lint_unused_unsafe" and "lint_unused_unsafe_all" can't be tested/built, but only
#   checked; and not in release, but only in debug
echo
echo "CARGO CHECK --TESTS (debug, feature lint_unused_unsafe)"
cargo +nightly check --tests --features lint_unused_unsafe
echo
echo "CARGO CHECK --TESTS (debug, feature lint_unused_unsafe_all)"
cargo +nightly check --tests --features lint_unused_unsafe_all

echo
echo "CARGO CHECK --TESTS (negative_tests/verify_error_messages, debug, feature unused_lint)"
cd negative_tests/verify_error_messages
cargo +nightly check --tests --features lint_unused_unsafe
echo
echo "CARGO CHECK --TESTS (negative_tests/verify_error_messages, debug, feature unused_lint_all)"
cargo +nightly check --tests --features lint_unused_unsafe_all
cd - >/dev/null

echo
echo "CARGO TEST (MIRI)"
cargo +nightly miri test
echo
echo "CARGO TEST (MIRI, feature unused_lint)"
cargo +nightly miri test --features lint_unused_unsafe
echo
echo "CARGO TEST (MIRI, feature unused_lint_all)"
cargo +nightly miri test --features lint_unused_unsafe_all
