# Contributing

## GIT branches

No merge pull requests. Instead, rebase on `main`.

## Both stable and nightly

`prudent` works 100% on `stable` Rust. However, doctests need both `stable` and `nightly` Rust
toolchains. That's because some `compile_fail` doctests have their error number verified (search for
`compile_fail,E` in [src/lib.rs](src/lib.rs)). That's a long term/permanently `nightly`-only
feature.

GitHub Actions also need `nightly`, since they use [MIRI](https://github.com/rust-lang/miri). See
also [.github/workflows/main.yml](.github/workflows/main.yml).

## Fast doctest checks

`cargo check` doesn't check doctests. For that, you need `nightly` Rust toolchain, and run:

`RUSTDOCFLAGS="-Z unstable-options --no-run" cargo +nightly test`

## File formatting

- Use `cargo fmt` for Rust source.
- Leave one empty line at the end of Rust, Markdown, Toml, Yaml and any other source/config files.

## Licenses

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be triple licensed as per
[LICENSE-APACHE](LICENSE-APACHE), [LICENSE-BSD](LICENSE-BSD) and  [LICENSE-MIT](LICENSE-MIT),
without any additional terms or conditions.
