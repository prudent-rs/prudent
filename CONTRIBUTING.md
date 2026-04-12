# Contributing

## GIT branches

No merge pull requests. Instead, rebase on `main`.

## Both stable and nightly

`prudent` works 100% on `stable` Rust. However, doctests also need the `nightly` Rust toolchain.
That's because some `compile_fail` doctests have their error number verified (search for
`compile_fail,E` in [src/lib.rs](src/lib.rs)). That's a long term/permanently `nightly`-only
feature.

GitHub Actions also need `nightly`, since they use [MIRI](https://github.com/rust-lang/miri). See
also [.github/workflows/main.yml](.github/workflows/main.yml).

### cargo clean on nightly

When developing with `1.96.0-nightly` (April 2026), [tests-run-nightly.sh](./tests-run-nightly.sh)
sometimes didn't generate expected `unused_unsafe` lint errors. Running `cargo clean` helps.

## Fast doctest checks

`cargo check` doesn't check doctests. For that, you need `nightly` Rust toolchain, and run:

`RUSTDOCFLAGS="-Z unstable-options --no-run" cargo +nightly test`

## docs.rs pre-check

1. Install [dtolnay/cargo-docs-rs](https://github.com/dtolnay/cargo-docs-rs):
   `cargo +nightly install cargo-docs-rs`
2. `cargo +nightly docs-rs`

## File formatting

- Use `cargo fmt` for Rust source.
- Leave one empty line at the end of Rust, Markdown, Toml, Yaml and any other source/config files.

## Licenses

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be triple licensed as per
[LICENSE-APACHE](LICENSE-APACHE), [LICENSE-BSD](LICENSE-BSD) and [LICENSE-MIT](LICENSE-MIT),
without any additional terms or conditions.
