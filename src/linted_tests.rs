// This separate module exists to workaround the issue of no lint control in cross-crate macro_rules
// https://github.com/rust-lang/rust/issues/110613). Without this separate file we got (when
// "linted" used to be called "front_end"):
/*
---- src/internal_front_end.rs - internal_front_end::internal_prudent_unsafe_fn (line 117) stdout ----
error: circular modules: src/internal_front_end.rs -> src/internal_front_end.rs
   --> src/internal_front_end.rs:121:1
    |
121 | ::prudent::load!(any: "internal_front_end.rs");
*/

const _VERIFY_MODULE_PATH: () = {
    let path = core::module_path!().as_bytes();
    if !matches!(path, b"prudent::linted_tests") {
        panic!("Do NOT load linted_tests.rs in other crates. It's internal in prudent only.");
    }
};

/// # unsafe_fn
///
/// Zero arguments. The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
/// // @TODO Docs: at your crate's top level, use either self::prudent, or crate:;prudent (but NOT
/// // just prudent, which will fail, fortunately).
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
/// Some arguments. The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
/// A passed parameter (expression that evaluates to a value passed to the target `unsafe` function as an argument) itself is `unsafe.`
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
/// The target function is safe, hence no need for `unsafe_fn`. Zero args.
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
/// The target function is safe, hence no need for `unsafe_fn`. Some args.
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
/// test cfg test:
/// ```test_harness
/// #[cfg(not(test))]
/// compile_error!("NOT DOCTEST!");
/// ```
/// Use the result of `unsafe_fn!` immediately as an array/slice:
/// ```test_harness
/// //TODO failing
/// //# ::prudent::load!("linted.rs");
/// //::prudent::load!(any: "linted.rs");
/// ::prudent::load!(any: "linted.rs");
/// // ::prudent::load!( "linted.rs");
/// # use self::prudent::*;
/// const unsafe fn return_array() -> [bool; 1] { [true] }
///
/// const _: bool = unsafe_fn!( return_array)[0];
/// ```
/// Use the result of `unsafe_fn!` immediately as a mutable array/slice (assign/modify its slot(s)):
/// ```
/// ::prudent::load!(any: "linted.rs");
/// use self::prudent::*;
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
/// ::prudent::load!(any: "linted.rs");
/// use crate::prudent::*;
/// unsafe fn return_same_mut_ref<T>(mref: &mut T) -> &mut T {
///    mref
/// }
///
/// fn main() {
///     let mut marray = [true];
///     unsafe_fn!( return_same_mut_ref, &mut marray )[0] = true;
/// }
/// ```
/// TODO docs about tuple tree
pub fn internal_prudent_unsafe_fn() {}

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
///  #![allow(clippy::needless_doctest_main)]
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
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
///
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
///
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
///
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/zero_args.rs")]
/// ```
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/some_args.rs")]
/// ```
pub fn internal_prudent_unsafe_method() {}

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail
/// compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};
//----------------------

/// ```
/// ::prudent::load!(any: "linted.rs");
/// use self::prudent::*;
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
pub fn internal_prudent_unsafe_static_set() {}

//----------------------
//----------------------
//----------------------
//----------------------
//----------------------
//----------------------
//----------------------
