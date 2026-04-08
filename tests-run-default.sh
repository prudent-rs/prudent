#!/usr/bin/sh

# - KEEP this in sync with .github/workflows/main.yml
# - BUT, this needs to undo any directory change (`cd`) done in any of the GitHub Actions step

echo DEFAULT
echo
echo "DOC (default)"
cargo doc --no-deps --quiet

echo
echo "CARGO TEST (debug, default)"
cargo test

echo
echo "CARGO TEST (release, default)"
cargo test --release
