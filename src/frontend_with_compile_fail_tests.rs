//! Frontend macros.
//!
//! Implementation notes ARE a part of the documentation:
//! - Users deserve documentation of **how** a macro works, because
//!   - macros are much more difficult to read than Rust non-macro code, and
//!   - macros inject code, so they are not as sandboxed/isolated as non-macro code.
//! - Otherwise it's a pain to edit them/render them in VS Code. Yes, that matters.

/// ```compile_fail
/// // @TODO Docs: at your crate's top level, use either self::prudent, or crate:;prudent (but NOT
/// // just prudent, which will fail, fortunately).
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
///
/// ## Some arguments
/// The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
///
/// A passed parameter (expression that evaluates to a value passed to the target `unsafe` function as an argument) itself is `unsafe.`
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Zero args.
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Some args.
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
/// test cfg test:
/// ```test_harness
/// // test_harness -as per https://github.com/rust-lang/rust/issues/148942#issuecomment-3565011334
/// #[cfg(not(test))]
/// compile_error!("NOT DOCTEST!");
/// ```
/// Use the result of `unsafe_fn!` immediately as an array/slice:
/// ```test_harness
/// //TODO? failing??
/// use prudent::*;
/// const unsafe fn return_array() -> [bool; 1] { [true] }
///
/// const _: bool = unsafe_fn!( return_array)[0];
/// ```
/// Use the result of `unsafe_fn!` immediately as a mutable array/slice (assign/modify its slot(s)):
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// use prudent::*;
/// fn _test_unsafe_fn_returning_mut_ref() {
///     // NOT running under MIRI, because of an intentional leak.
///     if !cfg!(miri) {
///         unsafe fn return_mut_ref_array() -> &'static mut [bool; 1] {
///             let boxed = Box::new([true]);
///              Box::leak(boxed)
///         }
///
///     unsafe_fn!( return_mut_ref_array)[0] = true;
///     }
/// }
/// fn main() {}
/// ```
/// The same, but the function takes an argument (and no leak):
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// use prudent::*;
/// unsafe fn return_same_mut_ref<T>(mref: &mut T) -> &mut T {
///    mref
/// }
///
/// fn main() {
///     let mut marray = [true];
///     unsafe_fn!( return_same_mut_ref => &mut marray )[0] = true;
/// }
/// ```
pub const TODOinternal_prudent_unsafe_fn: () = {};

// Same `compile_fail` tests as listed above for `unsafe_fn`, but here we validate the error
// numbers.
//
// Error numbers are validated with `cargo +nightly test`, ([The Rustdoc book > Unstable features >
// Error numbers for compile-fail
// doctests](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests))
// but NOT with
// - `cargo +stable test` nor
// - RUSTDOCFLAGS="..." cargo +nightly doc ...
//
// Even though the following constant is "pub", it will **not** be a part of the public API, neither
// a part of the documentation - it's used for doctest only.
/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0308
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0308
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};
//----------------------

/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
// TODO refactor for new checks:
// ```compile_fail
//#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/zero_args.rs")]
// ```
//
//#[allow(clippy::useless_attribute)]
//#[allow(clippy::needless_doctest_main)]
// ```compile_fail
//#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/some_args.rs")]
// ```
pub const TODO_internal_prudent_unsafe_method: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};
//----------------------

//#[allow(clippy::useless_attribute)]
//#[allow(clippy::needless_doctest_main)]

/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// //use prudent::*;
/// fn main() {
/// {
///     static mut S: (bool,) = (true,);
///
///     let mptr = &raw mut S;
///     unsafe { *mptr = (false,); }
///
///     let _mref = unsafe {&mut *mptr};
///     
///     // The following IS accepted:
///     //
///     //{unsafe {&mut *mptr}}.0 = true;
///     //
///     // BUT, because the outer curly brackets {...} are **refused** just left of
///     // [index_here] when indexing arrays (see below), we use oval parenthesis (...)
///     // which work for both: the tuple access .usize_literal and for array access
///     // [usize_expression].
/// }
/// {
///     static mut ARR: [bool; 1] = [true];
///     let mptr = &raw mut ARR;
///     unsafe { *mptr = [false]; }
///
///     let _mref = unsafe {&mut *mptr};
///     *_mref = [false];
///     _mref[ 0 ] = true;
///     
///     // Read access OK:
///     let _b: bool = { unsafe {&mut *mptr} }[ 0 ];
///     // Mut access - bad: The following refused:
///     //
///     //{ unsafe {&mut *mptr} }[ 0 ] = true;
///     //
///     // Have to use oval parenthesis:
///     ( unsafe {&mut *mptr} )[ 0 ] = true;
/// }
/// }
/// ```
pub const TODO_internal_prudent_unsafe_static_set: () = {};
//----------------------
