#!/usr/bin/sh

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo "Clippy"
cargo clippy
echo
echo fmt
cargo fmt --check

echo
echo "fmt: violations_coverage/in_crate"
cd violations_coverage/in_crate
cargo fmt --check
cd -

echo
echo "Doc (stable)"
cargo doc --no-deps --quiet

echo
echo "Doc (nightly)"
RUSTDOCFLAGS="--forbid rustdoc::invalid_codeblock_attributes \
  --forbid rustdoc::missing_doc_code_examples \
  -Zcrate-attr=feature(rustdoc_missing_doc_code_examples)" \
  cargo +nightly doc --no-deps --quiet

echo
echo "Tests (debug, stable)"
cargo test

echo
echo "Tests (debug, nightly)"
# We need "cargo +nightly test" to validate error numbers. To locate them, search for
# "compile_fail,E" in src/lib.rs.
cargo +nightly test

echo
echo "Tests (release, stable)"
cargo test --release

echo
echo "Tests (debug, MIRI)"
cargo +nightly miri test

echo
echo "Tests (verify_error_messages=trybuild)"
cd violations_coverage/verify_error_messages
cargo test

echo
echo "fmt (verify_error_messages=trybuild)"
cargo fmt --check
