//! "backend" functionality (anything else than linted macros)
//! - helper macro(s); and
//! - any non-macro functionality.

/// For casting/ensuring that a user-provided function is unsafe. Used by [crate::unsafe_fn].
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
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg ),+ }, $crate::backend::expecting_unsafe_fn )
    };
    ( ~ { $arg_first:expr, $( $arg_rest:expr ),+ }, $( $path_part:tt )+ ) => {
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg_rest ),+ }, $( $path_part )+ ::arg )
    };
    ( ~ { $arg_last:expr }, $( $path_part:tt )+ ) => {
        $( $path_part )+ ::arg::fun
    };
}

/// NOT a part of public API. Pretend to get a mutable reference from a shared reference. For
/// internal/generated **compile-time** checks only.
#[doc(hidden)]
pub const fn shared_to_mut<T>(_: &T) -> &'static mut T {
    unreachable!()
}

/// This is an "early" type check for [crate::unsafe_val], so that the user knows to use
/// [crate::unsafe_val] with [core::marker::Copy] types only.
///
/// NOT a part of public API!
#[doc(hidden)]
pub const fn expect_copy_ptr<T: Copy>(_: *const T) {}
