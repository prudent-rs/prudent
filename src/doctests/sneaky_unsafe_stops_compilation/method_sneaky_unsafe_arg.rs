// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../../demos/src/sneaky_unsafe_stops_compilation/method_sneaky_unsafe_arg.rs")]
/// ```
pub const _: () = {};
