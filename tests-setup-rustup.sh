#!/bin/sh

# strict mode
set -euo pipefail

rustup install nightly --profile minimal
rustup +nightly component add clippy rustfmt miri
