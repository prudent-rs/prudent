#!/bin/sh

# strict mode
set -euo pipefail

rustup component add rustfmt
rustup install nightly --profile minimal
rustup +nightly component add clippy miri
