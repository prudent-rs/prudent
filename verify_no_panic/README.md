# verify_panic_never

## Goal

This verifies that **positive** uses of `prudent` (loaded from
[`../positive_tests/`](../positive_tests/)) never use Rust's `panic!`. See
[panic_never](https://docs.rs/panic-never/latest/panic_never/).

## Current support: Linux

### On Mac OS or Windows

- see linker settings at <https://dev.to/ender_minyard/rust-nostd-template-23j0>
- get the correct `[target.'cfg(...)']` or `[target.triple]` condition
- see
  - <https://doc.rust-lang.org/beta/cargo/reference/config.html> and/or
  - <https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#platform-specific-dependencies>
    and/or
  - output of `rustc --print=cfg`
- add a new `[target...]` with `rustflags` to [`.cargo/config.toml`](.cargo/config.toml)
- create a pull request in this project's repo
