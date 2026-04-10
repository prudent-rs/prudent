/// ```compile_fail,E0308
#[doc = include_str!("../../../demos/unused_unsafe_fails_lint/src/bin/functn_none_args.rs")]
/// ```
pub const _: () = {};
