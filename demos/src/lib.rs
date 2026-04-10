#[cfg(not(feature = "lint_unused_unsafe"))]
mod sneaky_unsafe_stops_compilation;
#[cfg(feature = "lint_unused_unsafe")]
mod unused_unsafe_fails_lint;
