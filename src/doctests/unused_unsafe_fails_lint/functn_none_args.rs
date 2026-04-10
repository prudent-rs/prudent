/// ```compile_fail,E0308
#[doc = include_str!("../../../demos/src/unused_unsafe_fails_lint/functn_none_args.rs")]
/// ```
pub const _: () = {};
