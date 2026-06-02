//! "backend" functionality (anything else than macros).

#[doc(hidden)]
#[inline]
pub const fn assert_version(expected_version: &'static str) {
    if !matches!(expected_version.as_bytes(), b"0.0.4") {
        panic!("prudent-rs/prudent is of different version than expected.");
    }
}

#[doc(hidden)]
const _: () = {
    assert_version(env!("CARGO_PKG_VERSION"));
};

/// Pretend to get an (owned) instance from/based on a shared reference. For internal/generated
/// **compile-time** checks only.
///
/// Internal - NOT a part of public API!
#[doc(hidden)]
pub const fn shared_to_owned<T>(_: &T) -> T {
    unreachable!()
}

/// This is an "early" type check for [crate::prelude::unsafe_val], so that the user knows to use
/// [crate::prelude::unsafe_val] with [core::marker::Copy] types only.
///
/// Internal - NOT a part of public API!
#[doc(hidden)]
pub const fn expect_copy_ptr<T: Copy>(_: *const T) {}
