//! "Unlinted" functionality (anything else than linted macros)
//! - macros that don't need `warn/deny/forbid/allow/expect` lint rules
//! - non-macro functionality.

/// Implementation may change to support a range of versions etc.
///
/// NOT a part of public API - internal.
#[doc(hidden)]
pub const fn verify_linted_version(linted_version: &'static str) {
    // https://github.com/rust-lang/rust/issues/143874 we can't (yet) use == on &str constants, not
    // even matches! macro.
    //
    //assert!( linted_version==env!("CARGO_PKG_VERSION") );
    //
    //assert!( matches!(linted_version, env!("CARGO_PKG_VERSION")) );
    let linted_version = linted_version.as_bytes();
    //
    //assert!( matches!(linted_version, env!("CARGO_PKG_VERSION").as_bytes()) );
    //
    //Can't yet:
    //
    //assert!( linted_version == env!("CARGO_PKG_VERSION").as_bytes() );
    //
    // Can't yet:
    //
    // assert!( matches!(&linted_version[0..5], b"0.0.3") );
    //
    // Can now:
    //
    //assert!( matches!(linted_version, b"0.0.3-beta") );
    assert!(matches!(linted_version, [b'0', b'.', b'0', b'.', b'3', ..]));
}

/// For casting/ensuring that a user-provided function is unsafe. Used by `unsafe_fn`.
///
/// Internal - NOT a part of public API.
#[doc(hidden)]
#[allow(clippy::module_inception)]
pub mod expecting_unsafe_fn {
    /// For casting/ensuring that a user-provided function is unsafe. Used by `unsafe_fn`.
    pub unsafe fn fun<R>() -> R {
        unreachable!()
    }
    /// Function with one argument.
    pub mod arg {
        /// Used by `unsafe_fn`.
        pub unsafe fn fun<A1, R>(_: A1) -> R {
            unreachable!()
        }

        /// Two arguments.
        #[allow(clippy::module_inception)]
        pub mod arg {
            #[allow(clippy::module_inception)]
            /// Used by `unsafe_fn`.
            pub unsafe fn fun<A1, A2, R>(_: A1, _: A2) -> R {
                unreachable!()
            }

            /// Three arguments.
            #[allow(clippy::module_inception)]
            pub mod arg {
                #[allow(clippy::module_inception)]
                /// Used by `unsafe_fn`.
                pub unsafe fn fun<A1, A2, A3, R>(_: A1, _: A2, _: A3) -> R {
                    unreachable!()
                }

                /// Four arguments.
                #[allow(clippy::module_inception)]
                pub mod arg {
                    #[allow(clippy::module_inception)]
                    /// Used by `unsafe_fn`.
                    pub unsafe fn fun<A1, A2, A3, A4, R>(_: A1, _: A2, _: A3, _: A4) -> R {
                        unreachable!()
                    }

                    /// Five arguments.
                    #[allow(clippy::module_inception)]
                    pub mod arg {
                        /// Used by `unsafe_fn`.
                        pub unsafe fn fun<A1, A2, A3, A4, A5, R>(
                            _: A1,
                            _: A2,
                            _: A3,
                            _: A4,
                            _: A5,
                        ) -> R {
                            unreachable!()
                        }

                        /// Six arguments.
                        #[allow(clippy::module_inception)]
                        pub mod arg {
                            /// Used by `unsafe_fn`.
                            pub unsafe fn fun<A1, A2, A3, A4, A5, A6, R>(
                                _: A1,
                                _: A2,
                                _: A3,
                                _: A4,
                                _: A5,
                                _: A6,
                            ) -> R {
                                unreachable!()
                            }

                            /// Seven arguments.
                            #[allow(clippy::module_inception)]
                            pub mod arg {
                                /// Used by `unsafe_fn`.
                                pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, R>(
                                    _: A1,
                                    _: A2,
                                    _: A3,
                                    _: A4,
                                    _: A5,
                                    _: A6,
                                    _: A7,
                                ) -> R {
                                    unreachable!()
                                }

                                /// Eight arguments.
                                #[allow(clippy::module_inception)]
                                pub mod arg {
                                    /// Used by `unsafe_fn`.
                                    #[allow(clippy::too_many_arguments)]
                                    pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, A8, R>(
                                        _: A1,
                                        _: A2,
                                        _: A3,
                                        _: A4,
                                        _: A5,
                                        _: A6,
                                        _: A7,
                                        _: A8,
                                    ) -> R {
                                        unreachable!()
                                    }

                                    /// Nine arguments.
                                    #[allow(clippy::module_inception)]
                                    pub mod arg {
                                        /// Used by `unsafe_fn`.
                                        #[allow(clippy::too_many_arguments)]
                                        pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, A8, A9, R>(
                                            _: A1,
                                            _: A2,
                                            _: A3,
                                            _: A4,
                                            _: A5,
                                            _: A6,
                                            _: A7,
                                            _: A8,
                                            _: A9,
                                        ) -> R {
                                            unreachable!()
                                        }

                                        /// Ten arguments.
                                        #[allow(clippy::module_inception)]
                                        pub mod arg {
                                            /// Used by `unsafe_fn`.
                                            #[allow(clippy::too_many_arguments)]
                                            pub unsafe fn fun<
                                                A1,
                                                A2,
                                                A3,
                                                A4,
                                                A5,
                                                A6,
                                                A7,
                                                A8,
                                                A9,
                                                A10,
                                                R,
                                            >(
                                                _: A1,
                                                _: A2,
                                                _: A3,
                                                _: A4,
                                                _: A5,
                                                _: A6,
                                                _: A7,
                                                _: A8,
                                                _: A9,
                                                _: A10,
                                            ) -> R {
                                                unreachable!()
                                            }

                                            /// Eleven arguments.
                                            #[allow(clippy::module_inception)]
                                            pub mod arg {
                                                /// Used by `unsafe_fn`.
                                                #[allow(clippy::too_many_arguments)]
                                                pub unsafe fn fun<
                                                    A1,
                                                    A2,
                                                    A3,
                                                    A4,
                                                    A5,
                                                    A6,
                                                    A7,
                                                    A8,
                                                    A9,
                                                    A10,
                                                    A11,
                                                    R,
                                                >(
                                                    _: A1,
                                                    _: A2,
                                                    _: A3,
                                                    _: A4,
                                                    _: A5,
                                                    _: A6,
                                                    _: A7,
                                                    _: A8,
                                                    _: A9,
                                                    _: A10,
                                                    _: A11,
                                                ) -> R
                                                {
                                                    unreachable!()
                                                }
                                                /// Twelve arguments.
                                                #[allow(clippy::module_inception)]
                                                pub mod arg {
                                                    /// Used by `unsafe_fn`.
                                                    #[allow(clippy::too_many_arguments)]
                                                    pub unsafe fn fun<
                                                        A1,
                                                        A2,
                                                        A3,
                                                        A4,
                                                        A5,
                                                        A6,
                                                        A7,
                                                        A8,
                                                        A9,
                                                        A10,
                                                        A11,
                                                        A12,
                                                        R,
                                                    >(
                                                        _: A1,
                                                        _: A2,
                                                        _: A3,
                                                        _: A4,
                                                        _: A5,
                                                        _: A6,
                                                        _: A7,
                                                        _: A8,
                                                        _: A9,
                                                        _: A10,
                                                        _: A11,
                                                        _: A12,
                                                    ) -> R
                                                    {
                                                        unreachable!()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate path to `fun` under [expecting_unsafe_fn::arg], or [expecting_unsafe_fn::arg::arg], or
/// [expecting_unsafe_fn::arg::arg::arg] etc, as appropriate for given number of argument(s).
///
/// Internal - NOT a part of public API.
#[macro_export]
#[doc(hidden)]
macro_rules! expecting_unsafe_fn_path {
    ( $( $arg:expr ),+ ) => {
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg ),+ }, $crate::unlinted::expecting_unsafe_fn )
    };
    ( ~ { $arg_first:expr, $( $arg_rest:expr ),+ }, $( $path_part:tt )+ ) => {
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg_rest ),+ }, $( $path_part )+ ::arg )
    };
    ( ~ { $arg_last:expr }, $( $path_part:tt )+ ) => {
        $( $path_part )+ ::arg::fun
    };
}

/// NOT a part of public API. Ensure that maximum one of `~allow_unsafe` or `~expect_unsafe` is
/// passed to [unsafe_method].
#[doc(hidden)]
#[macro_export]
macro_rules! allow_unsafe_expect_unsafe_is_correct {
    (
        ~allow_unsafe  $( { $_allow_unsafe_empty_indicator:tt  } )?
        ~expect_unsafe $( { $_expect_unsafe_empty_indicator:tt } )?
    ) => {
        compile_error!(
            "Do not use *both* ~allow_unsafe and ~expect_unsafe with unsafe_method macro."
        );
    };
    (
        ~allow_unsafe  $( { $_allow_unsafe_empty_indicator:tt  } )?
    ) => {};
    (
        ~expect_unsafe  $( { $_expect_unsafe_empty_indicator:tt  } )?
    ) => {};
    () => {};
}

/// NOT a part of public API. Pretend to get a mutable reference from a shared reference. For
/// internal/generated **compile-time** checks only.
#[doc(hidden)]
pub const fn shared_to_mut<T>(_: &T) -> &'static mut T {
    unreachable!()
}

/// This is an "early" type check for [unsafe_val], so that the user knows to use [unsafe_val] with
/// [core::marker::Copy] types only.
///
/// NOT a part of public API!
#[doc(hidden)]
pub const fn expect_copy_ptr<T: Copy>(_: *const T) {}
