// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
pub const _: () = {};
