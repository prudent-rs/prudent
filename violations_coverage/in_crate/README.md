Compilation fails - as intended.

This crate exists to run
- `cargo fmt`, so that symlinked-files get formatted.
- `cargo expand`, for example: `cargo expand --bin unsafe_fn_some_args_arg`

This needs a filesystem that supports symlinks.
