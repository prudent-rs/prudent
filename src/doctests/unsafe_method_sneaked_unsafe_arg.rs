// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
pub const _: () = {};
