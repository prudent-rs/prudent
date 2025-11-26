We keep the compilation-failing `.rs` (and their respective `.stderr` files) in directory (actually,
a symlink) called [`violations`](violations). This directory/symlink must NOT be called "tests"!

Otherwise, if it were "tests" AND if it contained any .rs files being checked by
[dtolnay/trybuild](https://github.com/dtolnay/trybuild), those files would ALSO be loaded by
standard `cargo test` mechanism - as if they were integration tests. That would cause many errors
like "the name `internal_prudent_...` is defined multiple times", because in that (symlinked)
directory we also have (a symlink to) `prudent`'s file `linted.rs`:
[`violations/linted.rs`](violations/linted.rs).
