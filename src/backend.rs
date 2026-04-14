//! "backend" functionality (anything else than macros).

#[doc(hidden)]
#[inline]
pub const fn assert_version(expected_version: &'static str) {
    if !matches!(expected_version.as_bytes(),
        b"0.0.3") {
        panic!("prudent-rs/prudent is of different version than expected.");
    }
}

#[doc(hidden)]
const _ASSERT_VERSION_VERIFY: () = {
    assert_version(env!("CARGO_PKG_VERSION"));
};

unsafe fn _unsafe_generic_fun<R>() -> R {
    unreachable!()
}
fn _safe_generic_fun<R>() -> R {
    unreachable!()
}

unsafe fn _unsafe_fun_bool() -> bool {
    true
}
fn _safe_fun_bool() -> bool {
    true
}

fn _safe_zero_args() {}

/// See [ExpectedUnsafeFunctionButReceivedSafe::prudent_conflict_for_safe_function].
///
/// Internal - NOT a part of public API!
#[doc(hidden)]
pub trait ExpectedUnsafeFunctionButReceivedSafe {
    /// Triggers `multiple applicable items in scope` error if you invoke it on a function pointer
    /// that **is** safe. Used by [crate::prelude::unsafe_fn] and [crate::prelude::unsafe_method].
    ///
    /// Internal - NOT a part of public API!
    fn prudent_conflict_for_safe_function(&self) {}
}
impl<_O, F: Fn() -> _O> ExpectedUnsafeFunctionButReceivedSafe for F {}

/// See [FailsWithConflictForSafeFunction::prudent_conflict_for_safe_function].
///
/// Internal - NOT a part of public API!
#[doc(hidden)]
pub trait FailsWithConflictForSafeFunction {
    /// Triggers `multiple applicable items in scope` error if you invoke it on a function pointer
    /// that **is** safe. Used by [crate::prelude::unsafe_fn] and [crate::prelude::unsafe_method].
    ///
    /// Internal - NOT a part of public API!
    fn prudent_conflict_for_safe_function(&self) {}
}
impl<T> FailsWithConflictForSafeFunction for T {}

fn _try_unsafe_fn_zero_args() {
    (_safe_fun_bool as unsafe fn() -> bool).prudent_conflict_for_safe_function();

    // OK: Fails
    //
    // @TODO move to a compile-fail doctest
    //_safe_fun_bool.prudent_conflict_for_safe_function();

    // OK: Fails
    //
    // @TODO move to a compile-fail doctest
    //_safe_zero_args.prudent_conflict_for_safe_function();

    _unsafe_fun_bool.prudent_conflict_for_safe_function();
    (_unsafe_fun_bool as unsafe fn() -> bool).prudent_conflict_for_safe_function();

    {
        // BEST: even for generic functions
        let unsafe_generic_fun_cast_as_non_generic = _unsafe_generic_fun;
        unsafe_generic_fun_cast_as_non_generic.prudent_conflict_for_safe_function();

        // BUT: The following is needed to narrow down from generic to non-generic function:
        let _: bool = unsafe { unsafe_generic_fun_cast_as_non_generic() };
    }
    // BEST: Fails, as it should
    //
    // @TODO move to a compile-fail doctest
    /*{
        // BEST: even for generic functions
        let safe_generic_fun_cast_as_non_generic = _safe_generic_fun;
        safe_generic_fun_cast_as_non_generic.prudent_conflict_for_safe_function();

        // BUT: The following is needed to narrow down from generic to non-generic function:
        let _: bool = unsafe { safe_generic_fun_cast_as_non_generic() };
    }*/
}

// NOT possible:
//
// pub fn expect_unsafe_fn<F: unsafe Fn<()>>(_: F) {}

/// For casting/ensuring that a user-provided function is unsafe. Used by
/// [crate::prelude::unsafe_fn] (and, when applicable, by [crate::prelude::unsafe_method]).
///
/// Internal - NOT a part of public API!
#[doc(hidden)]
#[allow(clippy::module_inception)]
pub mod expecting_unsafe_fn {
    /// For casting/ensuring that a user-provided function is unsafe. Used by `unsafe_fn`.
    pub unsafe fn fun<R>() -> R {
        unreachable!()
    }
    /// Function with one argument.
    pub mod arg {
        /// Used by [crate::prelude::unsafe_fn].
        ///
        /// @TODO check link generated for the above in docs.rs
        ///
        /// @TODO here and above: Try replace generics
        /// - `<A1, A2...>`` and `_:A1`` with `_: impl Sized``,
        /// - `<R>` with `impl Sized`
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
