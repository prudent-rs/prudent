**Not** compilable (except for [verify_error_messages/src/lib.rs](verify_error_messages/src/lib.rs)).

This requires an operating system and a filesystem that support symlinks.

# Error code validation
Some errors (that is, all errors that we check, other than lint violations), are checked for
compilation error codes. See [prudent's README.md](../README.md) for expected compilation error
codes. The error codes are validated by [GitHub Actions](../.github/workflows/main.yml), see
[results](https://github.com/prudent-rs/prudent/actions). Error code validation requires `nightly`
Rust toolchain.

# Error output validation
Lint violations don't have a special error code. So we validate the error message in
[verify_error_messages](verify_error_messages) with
[dtolnay/trybuild](https://github.com/dtolnay/trybuild/)
[crates.io/crates/trybuild](https://crates.io/crates/trybuild).

See also documentation in [prudent's src/lib.rs](../src/lib.rs).
