#!/usr/bin/sh

rustup component add clippy rustfmt
rustup install nightly --profile minimal
rustup +nightly component add miri
