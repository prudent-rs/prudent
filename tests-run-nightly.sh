#!/bin/sh

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo NIGHTLY
echo "CLIPPY (nightly)"
cargo clippy

echo
echo "FMT (nightly)"
cargo +nightly fmt --check

cd demos

echo
echo "FMT: demos (nightly)"
cargo +nightly fmt --check
cd - >/dev/null

echo
echo "DOC (nightly)"
RUSTDOCFLAGS="--forbid rustdoc::invalid_codeblock_attributes \
  --forbid rustdoc::missing_doc_code_examples \
  -Zcrate-attr=feature(rustdoc_missing_doc_code_examples)" \
  cargo +nightly doc --no-deps --quiet

echo
echo "CARGO TEST (debug, nightly)"
# We need "cargo +nightly test" to validate error numbers. To locate them, search for
# "compile_fail,E" in src/lib.rs.
cargo +nightly test

echo
echo "CARGO TEST (debug, nightly, lint_unused_unsafe)"
cargo +nightly test --features lint_unused_unsafe

echo
echo "CARGO TEST (release, nightly)"
cargo +nightly test --release

echo
echo "CARGO TEST (MIRI, nightly)"
cargo +nightly miri test

cd violations_coverage/verify_error_messages

echo
echo "CARGO TEST (verify_error_messages with trybuild, nightly)"
cargo +nightly test

echo
echo "CARGO FMT (verify_error_messages with trybuild, nightly)"
cargo +nightly fmt --check
cd - >/dev/null
