Duplication of all `compile_fail` doctests in `src/prelude.rs`, but here each doctest is in a
separate module. That gives correct line numbers if any of those doctests fail. That would be
difficult otherwise, since all those doctests are injected with `include_str`, which messes up the
line numbers. That is even more difficult for failure if `compile_fail` tests, since `compile_fail`
tests that fail (that is, ones that actually do compile) don't report any particular source code
snippet to search for.

Submodule names (and their filenames) are similar to respective file names under
`../violations_coverage/in_crate/src/bin/`.

Some of these doctests also validate the error numbers.

@TODO check if Markdown alias applies here on docs.rs and in VS Code/RustRover; otherwise add
Rustdoc book URL:

Error numbers are validated with `cargo +nightly test`, ([The Rustdoc book > Unstable features >
Error numbers for compile-fail
doctests](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests))
but NOT with
- `cargo +stable test` nor
- RUSTDOCFLAGS="..." cargo +nightly doc ...
