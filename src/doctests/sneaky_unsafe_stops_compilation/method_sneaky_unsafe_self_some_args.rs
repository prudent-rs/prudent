// OK with stable
/// ```compile_fail,E0133
#[doc = include_str!("../../../negative_tests/sneaky_unsafe_stops_compilation/src/bin/method_sneaky_unsafe_self_some_args.rs")]
/// ```
pub const _: () = {};
