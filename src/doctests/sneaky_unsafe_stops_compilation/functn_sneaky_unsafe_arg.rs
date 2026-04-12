/// ```compile_fail,E0133
#[doc = include_str!("../../../negative_tests/sneaky_unsafe_stops_compilation/src/bin/functn_sneaky_unsafe_arg.rs")]
/// ```
pub const _: () = {};
