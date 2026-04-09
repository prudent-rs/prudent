/// ```compile_fail,E0308
#[doc = include_str!("../../../unused_unsafe_fails_lint/functn_some_args.rs")]
/// ```
pub const _: () = {};
