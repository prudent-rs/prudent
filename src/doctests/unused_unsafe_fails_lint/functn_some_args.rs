/// ```compile_fail,E0308
#[doc = include_str!("../../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
pub const _: () = {};
