/// ```compile_fail,E0133
#[doc = include_str!("../../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
pub const _: () = {};
