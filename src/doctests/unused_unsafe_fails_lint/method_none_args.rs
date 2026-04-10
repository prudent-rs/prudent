// TODO refactor for new checks - CURRENTLY as a NON-DOC comment!!
//
// @TODO compilation error code
//
/// ```compile_fail
#[doc = include_str!("../../../demos/unused_unsafe_fails_lint/method_none_args.rs")]
/// ```
pub const _: () = {};
