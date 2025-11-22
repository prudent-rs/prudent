#[doc(hidden)]
#[allow(unused)]
pub const INTERNAL_LINTED_VERSION: &str = "0.0.3-beta";

// Implementation notes ARE a part of the documentation:
// - Otherwise it's a pain to edit them/render them in VS Code. Yes, that matters.
// - Users deserve documentation of how a macro works, because
//   - macros are much more difficult to read than Rust non-macro code, and
//   - macros inject code, so they are not as sandboxed/isolated as non-macro code.
//

/// Invoke an `unsafe` function, but isolate `unsafe {...}` only for the function invocation itself.
/// - If `$fn`, that is, the function itself, is NOT given as an identifier/qualified path, but it's
///   given as an expression, then this expression is treated as if evaluated **outside** `unsafe
///   {...}`.
/// - Any arguments passed in as expressions are treated as if evaluated **outside** `unsafe {...}`.
///
/// There is **no** extra enclosing pair of parenthesis `(...)` around the list of arguments (if
/// any). If there was such a pair, it could be confused for a tuple. It would also be less readable
/// when some parameters were tuples/complex expressions.
///
/// This does NOT accept closures, since, closures cannot be `unsafe`.
///
/// # Possible violations
/// - Zero arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
/// - Some arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
#[macro_export]
macro_rules! internal_prudent_unsafe_fn {
    ( $fn:expr $(, $arg:expr)+ ) => {
        // Enclosed in (...) and NOT in {...}. Why? Because the later does NOT work if the result is
        // an array/slice and then it's indexed with array access suffix [usize_idx].
        (
            // Enclosed in a block, so that
            // 1. the result can be used as a value in an outer expression, and
            // 2. local variables don't conflict with the outer scope
            {
                // Ensure that $fn (the expression itself, one that yields a function to call) and
                // any arguments (expressions that yield values passed to the function to call)
                // don't include any unnecessary `unsafe{...}` block(s):
                #[deny(unused_unsafe)]
                // Ensure that $fn (the expression itself) and any arguments (expressions) don't
                // include any unsafe code/calls/casts on their  own without their own `unsafe{...}`
                // block(s):
                let (tuple_tree, fun) = (unsafe_fn_internal_build_tuple_tree!{ $($arg),+ }, $fn);

                if false {
                    // Ensure that $fn is not safe, but `unsafe`. Using
                    // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                    let _ = if false {
                        ::prudent::expecting_unsafe_fn_path!( $( $arg ),+ )
                    } else {
                        fun
                    };
                    unreachable!()
                }
                unsafe_fn_internal_build_accessors_and_call! {
                    fun,
                    tuple_tree,
                    ( $( $arg ),* ),
                    (0)
                }
            }
        )
    };
    ($fn:expr) => {
        ({
            // Ensure that $fn (the expression itself, one that yields a function to call) doesn't
            // include an unnecessary `unsafe{...}` block:
            #[deny(unused_unsafe)]
            // Ensure that $fn (the expression itself) doesn't include any unsafe code/calls/casts
            // on its own without its own `unsafe{...}` block(s):
            let fun = $fn;
            if false {
                // Ensure that $fn is not safe, but `unsafe`. Using
                // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                let _ = if false {
                    ::prudent::unlinted::expecting_unsafe_fn::fun
                } else {
                    fun
                };
                unreachable!()
            }
            // `#[deny(unused_unsafe)]` does NOT work here. Why? Because when we assigned `let fun =
            // $fn` above, that then happily coerces/infers to an unsafe function, even though it's
            // safe. That's why we have `expecting_unsafe_fn` module.
            #[allow(unsafe_code)]
            let result = unsafe {
                fun()
            };
            result
        })
    };
}

// Such aliases are needed: https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997
//
// pub use internal_prudent_unsafe_fn as internal_prudent_unsafe_fn_;
//
// OR:
pub use internal_prudent_unsafe_fn;

/// INTERNAL. Do NOT use directly - subject to change.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_prudent_unsafe_fn_internal_build_tuple_tree {
    // Construct the tuple_tree. Recursive:
    ( $first:expr, $($rest:expr),+ ) => {
        (
            $first, unsafe_fn_internal_build_tuple_tree!{ $($rest),+ }
        )
    };
    ( $last:expr) => {
        ($last,)
    };
}
pub use internal_prudent_unsafe_fn_internal_build_tuple_tree;

/// INTERNAL. Do NOT use directly - subject to change.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_prudent_unsafe_fn_internal_build_accessors_and_call {
    // Access tuple_tree parts and get ready to call the function:
    ( $fn:expr, $tuple_tree:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        unsafe_fn_internal_build_accessors_and_call!{
            $fn, $tuple_tree, ( $($other_arg),+ ),
            // Insert a new accessor to the front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    // All accessors are ready, so call the function:
    ( $fn:expr, $tuple_tree:ident,
      ( $_last_or_only_arg:expr ),
      $( ( $($accessor_part:tt),+
         )
      ),*
    ) => {
        #[allow(unsafe_code)]
        #[deny(unused_unsafe)]
        unsafe {
            $fn( $(
                    unsafe_fn_internal_access_tuple_tree_field!{ $tuple_tree, $($accessor_part),+ }
                ),*
            )
        }
    };
}
pub use internal_prudent_unsafe_fn_internal_build_accessors_and_call;

#[doc(hidden)]
#[macro_export]
/// INTERNAL. Do NOT use directly - subject to change.
///
/// Expand an accessor group/list to access a field in the tuple_tree.
macro_rules! internal_prudent_unsafe_fn_internal_access_tuple_tree_field {
    ( $tuple_tree:ident, $($accessor_part:tt),* ) => {
        $tuple_tree $(. $accessor_part )*
    };
}
pub use internal_prudent_unsafe_fn_internal_access_tuple_tree_field;
//-------------

/// Invoke an `unsafe` method. For methods that have a receiver parameter (`&self`, `&mut self`,
/// `self`). For associated functions (implemented for a type but with no receiver) use `unsafe_fn`,
/// and pass the qualified name of the associated function to it.
///
/// Like [internal_prudent_unsafe_fn], but
/// - This accepts a receiver `&self`, `&mut self` and `self`. TODO Box/Rc/Arc, dyn?
/// - This treats `self` as if it were evaluated **outside** the `unsafe {...}` block.
/// - $fn can **NOT** be an expression or a qualified path (which doesn't work in standard methods
///   calls anyways), but only an identifier.
///
/// Do NOT use parameters/input parts matched by
/// - `$expect_unsafe_empty_indicator` or
/// - `$allow_unsafe_empty_indicator`
///
/// as they are internal.
#[macro_export]
macro_rules! internal_prudent_unsafe_method {
    (
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $self:expr, $fn:ident $(, $arg:expr )*
     ) => {
        // See unsafe_fn for why here we enclose in (...) and not in {...}.
        (
            if false {
                ::prudent::allow_unsafe_expect_unsafe_is_correct!{
                    $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
                    $( ~expect_unsafe $( { $expect_unsafe_empty_indicator } )? )?
                }
                if false {
                    // This block "makes" owned_receiver, an instance/owned value of the same type
                    // as $self. (Of course, the instance is invalid - this is for compile-time
                    // checks only, hence `if false {...}`.)
                    //
                    // Then we simulate invocation of the given method inside `unsafe {...}``, BUT
                    // without evaluating the given $self expression inside that same `unsafe
                    // {...}`` block, so that we isolate/catch any `unsafe`` code in $self.
                    //
                    // We **cannot** just move/take/assign $self by value, in case it's a non-Copy
                    // **static** variable. See also comments in
                    // unsafe_method_internal_build_accessors_check_args_call.
                    let rref = {
                        #[rustfmt::skip]
                        #[deny(unused_unsafe)]
                        // @TODO simplify once https://github.com/rust-lang/rust/issues/15701
                        // (attributes on expressions)
                        #[deny(unsafe_code)]
                        $(
                            $( { $allow_unsafe_empty_indicator } )?
                            #[allow(unsafe_code)]
                        )?
                        $(
                            $( { $expect_unsafe_empty_indicator } )?
                            #[expect(unsafe_code)]
                        )?
                        let rref = &( $self );
                        rref
                    };
                    //
                    let mref = ::prudent::unlinted::shared_to_mut(rref);
                    let mut owned_receiver = ::core::mem::replace(mref, unsafe{ ::core::mem::zeroed() });
                    // Detect code where unsafe_fn! or unsafe_method! is not needed at all. That is,
                    // where a function/method used to be `unsafe`, but it stopped being so.
                    #[deny(unused_unsafe)]
                    let _ = unsafe { owned_receiver. $fn( $( $arg ),* ) };
                } else {
                    // @TODO eliminate
                    $(
                        #[deny(unused_unsafe)]
                        let _ = $arg;
                    )*
                }
                unreachable!()
            } else {
                //compile_error!("TODO move to unsafe_method_internal_check_args_etc");
                unsafe_method_internal_check_args_etc!(
                    $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
                    $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
                    $self, $fn $(, $arg )*
                )
                /*#[allow(unsafe_code)]
                // Notify if $self includes `unsafe {...}`, but no ~allow_unsafe or ~expect_unsafe:
                #[deny(unused_unsafe)]
                $(
                    $( { $allow_unsafe_empty_indicator } )?
                    #[allow(unused_unsafe)]
                )?
                $(
                    $( { $expect_unsafe_empty_indicator } )?
                    #[expect(unused_unsafe)]
                )?
                unsafe { $self. $fn ( $( $arg ),* ) }*/
            }
        )
    }
}
pub use internal_prudent_unsafe_method;

#[doc(hidden)]
#[macro_export]
macro_rules! internal_prudent_unsafe_method_internal_check_args_etc {
    (
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $self:expr, $fn:ident $(, $arg:expr )+
     ) => {({
                #[deny(unused_unsafe)]
                let tuple_tree =
                    unsafe_fn_internal_build_tuple_tree!{ $($arg),+ };

                unsafe_method_internal_build_accessors_check_args_call! {
                    $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
                    $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
                    $self,
                    $fn,
                    tuple_tree,
                    ( $( $arg ),* ),
                    (0)
                }
    })};
    (
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $self:expr, $fn:ident
     ) => {({
                #[allow(unsafe_code)]
                // Notify if $self includes `unsafe {...}`, but no ~allow_unsafe or ~expect_unsafe:
                //
                //#[deny(unused_unsafe)]
                #[deny(unused_unsafe)]
                $(
                    $( { $allow_unsafe_empty_indicator } )?
                    #[allow(unused_unsafe)]
                )?
                $(
                    $( { $expect_unsafe_empty_indicator } )?
                    #[expect(unused_unsafe)]
                )?
                #[deny(unused_unsafe)]
                let result = unsafe { $self. $fn () };
                result
    })};
}
pub use internal_prudent_unsafe_method_internal_check_args_etc;

#[doc(hidden)]
#[macro_export]
macro_rules! internal_prudent_unsafe_method_internal_build_accessors_check_args_call {
    // Access tuple_tree parts and get ready to call the method:
    (
     $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
     $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
     $self:expr, $fn:ident, $tuple_tree:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        unsafe_method_internal_build_accessors_check_args_call!{
            $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
            $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
            $self, $fn, $tuple_tree, ( $($other_arg),+ ),
            // Insert a new accessor to the front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    // All accessors are ready. Call the function:
    (
     $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
     $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
     $self:expr, $fn:ident, $tuple_tree:ident,
      ( $_last_or_only_arg:expr ),
      $( ( $($accessor_part:tt),+
         )
      ),*
    ) => {({
        #[allow(unsafe_code)]
        #[deny(unused_unsafe)]
        $(
            $( { $allow_unsafe_empty_indicator } )?
            #[allow(unused_unsafe)]
        )?
        $(
            $( { $expect_unsafe_empty_indicator } )?
            #[expect(unused_unsafe)]
        )?
        let result = unsafe {
            // Unlike arguments, we can NOT store result of $self expression in a variable, because
            // it would be moved, but a method with receiver by reference `&self` or `&mut self`
            // does NOT move the instance it's called on. And if Self were `Copy`, then the
            // reference would not point to the original instance!
            $self. $fn( $(
                    unsafe_fn_internal_access_tuple_tree_field!{ $tuple_tree, $($accessor_part),+ }
                ),*
            )
        };
        result
    })};
}
pub use internal_prudent_unsafe_method_internal_build_accessors_check_args_call;
//-------------

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
#[macro_export]
macro_rules! internal_prudent_unsafe_static_set {
    ($static:path, $val:expr) => {{
        if false {
            let _ = $val;
            unreachable!()
        } else {
            #[allow(unsafe_code)]
            unsafe {
                $static = $val;
            }
        }
    }};
    // @TODO implement + rename, so it's for union fields, too:
    //
    // @TODO similar to read union fields
    ($static:ident { $( $suffix:tt )* } $val:expr) => {{}};
    ($static:path { $( $suffix:tt )* } $val:expr) => {{
        if false {
            let mptr = &raw mut $static;
            let mref = unsafe { &mut *mptr };
            unreachable!()
        } else {
        }
    }};
}
pub use internal_prudent_unsafe_static_set;

/// Deref a pointer (either `const` or `mut`) and yield a read-only reference.
///
/// If `$type` is given, it's expected to be the referenced type (NOT the given pointer, NOT a
/// reference based on the given pointer), and the given pointer is cast to `* const $type`. `$type`
/// may start with `dyn`. `$type` may be a slice `[...]`.
#[macro_export]
macro_rules! internal_prudent_unsafe_ref {
    ($ptr:expr) => {{
        let ptr: *const _ = $ptr;
        unsafe { &*ptr }
    }};
    ($ptr:expr, $lifetime:lifetime) => {{
        let ptr: *const _ = $ptr;
        unsafe { &*ptr as &$lifetime _ }
    }};
    ($ptr:expr, $type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $type;
        unsafe { &*ptr }
    }};
    ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $ptr_type;
        unsafe { &*ptr as &$lifetime _ }
    }};
}
pub use internal_prudent_unsafe_ref;

/// Deref a `mut` pointer and yield a `mut` reference.
///
/// Like for [internal_prudent_unsafe_ref]: If `$type` is given, it's expected to be the referenced
/// type (NOT the given pointer, NOT the target reference type) and the given pointer is cast to `*
/// const $type`. `$type` may start with `dyn`. `$type` may be a slice `[...]`.
#[macro_export]
macro_rules! internal_prudent_unsafe_mut {
    ($ptr:expr) => {{
        let ptr: *mut _ = $ptr;
        unsafe { &mut *ptr }
    }};
    ($ptr:expr, $lifetime:lifetime) => {{
        let ptr: *mut _ = $ptr;
        unsafe { &mut *ptr as &$lifetime mut _}
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *mut $ptr_type;
        unsafe { &mut *ptr}
    }};
    ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {{
        let ptr = $ptr;
        let ptr = ptr as *mut $ptr_type;
        unsafe { &mut *ptr as &$lifetime mut _}
    }};
}
pub use internal_prudent_unsafe_mut;

/// Get a (copy of) value from where the pointer points. For [core::marker::Copy] types only.
#[macro_export]
macro_rules! internal_prudent_unsafe_val {
    ($ptr:expr) => {{
        let ptr: *const _ = $ptr;
        ::prudent::unlinted::expect_copy_ptr(ptr);
        unsafe { *ptr }
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $ptr_type;
        ::prudent::unlinted::expect_copy_ptr(ptr);
        unsafe { *ptr }
    }};
}
pub use internal_prudent_unsafe_val;

/*
-nightly version only
https://doc.rust-lang.org/std/keyword.use.html#ergonomic-clones
https://doc.rust-lang.org/std/clone/trait.UseCloned.html


#[macro_export]
macro_rules! unsafe_use {
    ($ptr:expr) => {{
        let ptr = $ptr;
        unsafe { ( *ptr ).use }
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr as $ptr_type;
        unsafe { ( *ptr ).use }
    }};
}*/

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
#[macro_export]
macro_rules! internal_prudent_unsafe_set {
    ($ptr:expr, $value:expr) => {{
        if false {
            let _: *mut _ = $ptr;
            let _ = $value;
            unreachable!()
        } else {
            #[allow(unsafe_code)]
            unsafe {
                *$ptr = $value;
            }
        }
    }};
}
pub use internal_prudent_unsafe_set;
