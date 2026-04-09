// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../demos/sneaky_unsafe_stops_compilation/method_sneaky_unsafe_self_none_args.rs")]
/// ```
pub const _: () = {};
