/// Doctests to verify that violations of prudent stop compilation.
#[cfg(all(doctest, not(feature = "lint_unused_unsafe")))]
pub mod sneaky_unsafe_stops_compilation;

/// Doctests to verify that using prudent unnecessarily fails "unused_unsafe" lint.
#[cfg(all(doctest, feature = "lint_unused_unsafe"))]
pub mod unused_unsafe_fails_lint;
