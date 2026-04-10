// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../../demos/sneaky_unsafe_stops_compilation/src/bin/method_sneaky_unsafe_arg.rs")]
/// ```
pub const _: () = {};
