//! "Frontend" = user-facing macros. Suggested to be blank imported by the user.

#[cfg(not(feature = "lint_unused_unsafe"))]
use prudent_macros_enforce as prudent_macros;
#[cfg(feature = "lint_unused_unsafe")]
use prudent_macros_lint as prudent_macros;

/// Invoke an `unsafe` function, but isolate `unsafe {...}` only for the function invocation itself.
/// - If `$fn` (the function itself) is NOT given as an identifier/qualified path, but it's given as
///   an expression, then this expression is treated as if evaluated **outside** `unsafe {...}`.
/// - Any arguments passed in as expressions are treated as if evaluated **outside** `unsafe {...}`.
///
/// There is **no** extra enclosing pair of parenthesis `(...)` around the list of arguments (if
/// any). If there was such a pair, it could be confused for a tuple. It would also be less readable
/// when some parameters were tuples/complex expressions.
///
/// This does NOT accept closures, since closures cannot be `unsafe`.
///
/// # Possible violations
/// - No arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
/// - Some arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
/// OK with stable
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/functn_sneaky_unsafe_fn_expr_none_args.rs")]
/// ```
///
/// ## Some arguments
/// The given expression (which evaluates to the function to be called) is `unsafe.`
// OK with stable
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/functn_sneaky_unsafe_fn_expr_some_args.rs")]
/// ```
///
/// A passed parameter (expression that evaluates to a value passed to the target `unsafe` function as an argument) itself is `unsafe.`
/// // OK with stable
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/functn_sneaky_unsafe_arg.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Zero args.
///
/// @TODO this should fail, but it does NOT
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```ignore")]
#[doc = include_str!("../demos/unused_unsafe_fails_lint/src/bin/functn_none_args.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Some args.
///
/// OK on stable
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```ignore")]
#[doc = include_str!("../demos/unused_unsafe_fails_lint/src/bin/functn_some_args.rs")]
/// ```
/// @TODO consider:
/// ```test_harness
/// // test_harness -as per https://github.com/rust-lang/rust/issues/148942#issuecomment-3565011334
/// #[cfg(not(test))]
/// compile_error!("NOT DOCTEST!");
/// ```
/// Use the result of `unsafe_fn!` immediately as an array/slice:
/// ```test_harness
/// //TODO? failing??
/// use prudent::prelude::unsafe_fn;
/// const unsafe fn return_array() -> [bool; 1] { [true] }
///
/// const _: bool = unsafe_fn!( return_array)[0];
/// ```
/// Use the result of `unsafe_fn!` immediately as a mutable array/slice (assign/modify its slot(s)):
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// use prudent::prelude::*;
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
/// use prudent::prelude::*;
/// unsafe fn return_same_mut_ref<T>(mref: &mut T) -> &mut T {
///    mref
/// }
///
/// fn main() {
///     let mut marray = [true];
///     unsafe_fn!( return_same_mut_ref => &mut marray )[0] = true;
/// }
/// ```
pub use prudent_macros::unsafe_fn;

/// Invoke an `unsafe` method. For methods that have a receiver parameter (`&self`, `&mut self`,
/// `self`). For associated functions (implemented for a type but with no receiver) use `unsafe_fn`,
/// and pass the qualified name of the associated function to it.
///
/// Like [unsafe_fn], but
/// - This accepts a receiver `&self`, `&mut self` and `self`. TODO Box/Rc/Arc, dyn?
/// - This treats `self` as if it were evaluated **outside** the `unsafe {...}` block.
/// - $fn can **NOT** be an expression or a qualified path (which doesn't work in standard methods
///   calls anyways), but only an identifier.
///
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/method_sneaky_unsafe_arg.rs")]
/// ```
///
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/method_sneaky_unsafe_self_none_args.rs")]
/// ```
///
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```compile_fail")]
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```ignore")]
#[doc = include_str!("../demos/sneaky_unsafe_stops_compilation/src/bin/functn_sneaky_unsafe_fn_expr_some_args.rs")]
/// ```
// TODO refactor for new checks - CURRENTLY as a NON-DOC comment!!
//
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```ignore")]
//#[doc = include_str!("../demos/unused_unsafe_fails_lint/src/bin/method_none_args.rs")]
// ```
//
//#[allow(clippy::useless_attribute)]
//#[allow(clippy::needless_doctest_main)]
// OK with stable
//
#[cfg_attr(feature = "lint_unused_unsafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "lint_unused_unsafe"), doc = "```ignore")]
//#[doc = include_str!("../demos/unused_unsafe_fails_lint/src/bin/method_some_args.rs")]
// ```
pub use prudent_macros::unsafe_method;

/// Set a value of a `static mut` variable or its (sub...-)field, but isolate `unsafe {...}` only to
/// that assignment.
///
/// To minimize unintended `unsafe`, calculate any indexes etc. beforehand, store them in local
/// variables and pass them in.
///
/// We do **not** have a similar macro to get a value of a `static mut`. For that, simply enclose it
/// in `unsafe{...}`. TODO reconsider.
///
/// NOT for `static` variables (or their fields/components) of `union` types.
///
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// //use prudent::prelude::*;
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
pub use prudent_macros::unsafe_static_set;

/// Deref a pointer (either `const` or `mut`) and yield a read-only reference.
///
/// If `$type` is given, it's expected to be the referenced type (NOT the given pointer, NOT a
/// reference based on the given pointer), and the given pointer is cast to `* const $type`. `$type`
/// may start with `dyn`. `$type` may be a slice `[...]`.
pub use prudent_macros::unsafe_ref;

/// Deref a `mut` pointer and yield a `mut` reference.
///
/// Like in [unsafe_ref]: If `$type` is given, it's expected to be the referenced
/// type (NOT the given pointer, NOT the target reference type) and the given pointer is cast to `*
/// const $type`. `$type` may start with `dyn`. `$type` may be a slice `[...]`.
pub use prudent_macros::unsafe_mut;

/// Get a (copy of) value from where the pointer points. For [core::marker::Copy] types only.
pub use prudent_macros::unsafe_val;

/// Assign the given value to the location given in the pointer.
///
/// Needed, because we can't isolate:
///
/// `unsafe { *ptr } = value;`
///
/// Also, we can't have a macro invocation on the left side (target) of an assignment operator `=`,
/// so nothing like:
///
/// `unsafe_set!( pt ) = false;`
pub use prudent_macros::unsafe_set;
