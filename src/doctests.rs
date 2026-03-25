//! Duplication of all `compile_fail` doctests in <prelude.rs>, but here each doctest is in aS
//! separate module. That gives correct line numbers if any of those doctests fail. That would be
//! difficult otherwise, since all those doctests are injected with `include_str`, which messes up
//! the line numbers. That is even more difficult for failure if `compile_fail` tests, since
//! `compile_fail` that fail (that is, ones that do compile) don't report any particular source code
//! snippet to search for.
//!
//! Submodule names (and their filenames) are similar to respective file names under
//! [`../violations_coverage/in_crate/src/bin/`](../violations_coverage/in_crate/src/bin/).
//!
//! Some of these doctests also validate the error numbers.
//!
//! @TODO check if Markdown alias applies here; otherwise add Rustdoc book URL:
//!
//! Error numbers are validated with `cargo +nightly test`, ([The Rustdoc book > Unstable features >
//! Error numbers for compile-fail
// doctests](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests))
//! but NOT with
//! - `cargo +stable test` nor
//! - RUSTDOCFLAGS="..." cargo +nightly doc ...

pub mod unsafe_fn_fn_unused_unsafe_some_args;
pub mod unsafe_fn_fn_unused_unsafe_zero_args;
pub mod unsafe_fn_sneaked_unsafe_arg;
pub mod unsafe_fn_sneaked_unsafe_fn_expr_some_args;
pub mod unsafe_fn_sneaked_unsafe_fn_expr_zero_args;
pub mod unsafe_method_fn_unused_unsafe_some_args;
pub mod unsafe_method_fn_unused_unsafe_zero_args;
pub mod unsafe_method_sneaked_unsafe_arg;
pub mod unsafe_method_sneaked_unsafe_self_some_args;
pub mod unsafe_method_sneaked_unsafe_self_zero_args;
