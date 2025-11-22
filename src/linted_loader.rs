// - Linted ("frontend"_ macros can (and do) invoke unlinted ("backend") macros, but not the other
//   way. That's because unlinted macros don't know the crate name where linted macros were loaded.
// - linted macros can be recursive (between themselves, but not through unlinted ("backend")
//   macros")
/// Invoke from the top level of your crate, and only once (once per crate).
#[macro_export]
macro_rules! load {
    // No lints.
    //
    // re-export all (both `linted` and `unlinted`) from `prudent` crate under module `prudent`.
    () => {
        $crate::load!( prudent );
    };
    // No lints.
    //
    // re-export all (both `linted` and `unlinted`) from `prudent` crate, under given module name.
    ( $module_name:ident ) => {
        /// prudent exported in a module
        mod $module_name {
            $crate::reexport_macros!( ::prudent::linted );
            $crate::reexport_non_macros!( ::prudent::linted );
        }
    };// -----------

    // load `linted` from the given file; re-export `unlinted` from `prudent` crate; blend them
    // under a module with the given name. If `$module_name` is not given, it's `prudent`.
    //
    // Since `any` is **not** specified, this applies to `$[cfg(test)` only.
    ( $prudent_linted_first:literal
      $( $prudent_linted_second:literal )?
      $( -> $module_name:ident )?
    ) => {
        $crate::load!( ~
            (test) :
            $prudent_linted_first
            $( $prudent_linted_second )?
            $( -> $module_name )?
        );
    };
    // `any`` means any build/`$cfg`/profile/target...
    ( any :
      $prudent_linted_first:literal
      $( $prudent_linted_second:literal )?
      $( -> $module_name:ident )?
     ) => {
        // @TODO test:
        $crate::load!( ~
            (test,not(test)) :
             $prudent_linted_first
             $( $prudent_linted_second )?
             $( -> $module_name )?
        );
    };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_linted_first:literal
      $( $prudent_linted_second:literal )?
    ) => {
        $crate::load!( ~
            ( $( $cfg_filter )* ) :
            $prudent_linted_first
            $( $prudent_linted_second )?
            -> prudent
        );
      };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_linted_same:literal
      -> $module_name:ident
    ) => {
        $crate::load!( ~
            ( $( $cfg_filter )* ) :
            $prudent_linted_same
            $prudent_linted_same
            -> $module_name
        );
    };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_linted_first:literal
      $prudent_linted_second:literal
      -> $module_name:ident
    ) => {
        #[cfg(not(windows))]
        $crate::load!( ~~
            ( $( $cfg_filter )* ) :
            $prudent_linted_first
            -> $module_name
        );

        #[cfg(windows)]
        $crate::load!( ~~
            ( $( $cfg_filter )* ) :
            $prudent_linted_second
            -> $module_name
        );
    };
    ( ~~
      ( $( $cfg_filter:tt )* ) :
      $prudent_linted:literal
      -> $module_name:ident
    ) => {
        // TODO obsolete docs:
        //
        // We do NOT load the file into a separate submodule, like:
        // ```
        // #[cfg(any( $( $cfg_filter )* ))]
        // #[allow(unused)]
        // #[path = $prudent_linted]
        // mod internal_prudent_linted_loaded_or_aliased;
        // ``
        // Why? because https://github.com/rust-lang/rust/issues/52234 makes it very difficult (or
        // impossible?) to re-export such macros. But using
        // `#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]` didn't help, so
        // there may be other restrictions, too.
        // Life is too short. So we ARE using ::core::include!(...);
        /*#[cfg(any( $( $cfg_filter )* ))]
        #[allow(unused)]
        mod internal_prudent_linted_loaded_or_aliased {
            ::core::include!( $prudent_linted);
        }*/

        #[cfg(any( $( $cfg_filter )* ))]
        #[allow(unused)]
        #[path = $prudent_linted]
        pub mod internal_prudent_linted_loaded_or_aliased;

        //compile_error!("WOULD RE-EXPORT INSTEAD OF LOAD FRONTEND");

        #[cfg(not(any( $( $cfg_filter )* )))]
        #[allow(unused)]
        pub use ::prudent::linted as internal_prudent_linted_loaded_or_aliased;

        mod $module_name {
            /*$crate::load_module_content!(
                ( $( $cfg_filter )* )
                : $prudent_linted
            );*/

            // @TODO
            //
            // #[cfg(not(any( $( $cfg_filter )* )))]
            //
            // $crate::reexport_macros!( ::prudent );
            //
            // #[cfg(any( $( $cfg_filter )* ))]
            //
            // $crate::reexport_macros!( crate );

            //#[cfg(any( $( $cfg_filter )* ))]
            ////#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
            //#[allow(unused)]
            //use crate::*;

            //#[cfg(not(any( $( $cfg_filter )* )))]
            //#[allow(unused)]
            //use ::prudent::linted::*;

            //$crate::reexport_macros!();
            //#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

            //pub use crate::internal_prudent_unsafe_fn as unsafe_fn;

            //pub use super::internal_prudent_linted_loaded_or_aliased::internal_prudent_unsafe_fn_ as unsafe_fn;
            //
            // OR:

            $crate::reexport_macros!(super::internal_prudent_linted_loaded_or_aliased);
            //pub use super::internal_prudent_linted_loaded_or_aliased::internal_prudent_unsafe_fn as unsafe_fn;

            $crate::reexport_non_macros!( super::internal_prudent_linted_loaded_or_aliased );
        }
        const _VERIFY_VERSION: () = {
            ::prudent::unlinted::verify_linted_version( $module_name::PRUDENT_INTERNAL_LINTED_VERSION );
        };
    }
}

/// Re-export macros from the given `linted`_module/path.
///
/// NOT a part of public API - internal.
#[doc(hidden)]
#[macro_export]
macro_rules! reexport_macros {
    ($path:path) => {
        #[allow(unused)]
        pub use $path::{
            // Without re-exporting all internal macros, we hit
            // https://github.com/rust-lang/rust/issues/52234. The consumer crate would have to
            // `#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]`, which is NOT
            // future-compatible!
            internal_prudent_unsafe_fn as unsafe_fn,
            internal_prudent_unsafe_fn_internal_build_tuple_tree as unsafe_fn_internal_build_tuple_tree,
            internal_prudent_unsafe_fn_internal_build_accessors_and_call as unsafe_fn_internal_build_accessors_and_call,
            internal_prudent_unsafe_fn_internal_access_tuple_tree_field as unsafe_fn_internal_access_tuple_tree_field,
            internal_prudent_unsafe_method as unsafe_method,
            internal_prudent_unsafe_method_internal_check_args_etc as unsafe_method_internal_check_args_etc,
            internal_prudent_unsafe_method_internal_build_accessors_check_args_call as unsafe_method_internal_build_accessors_check_args_call,
            internal_prudent_unsafe_static_set as unsafe_static_set,
            internal_prudent_unsafe_ref as unsafe_ref,
            internal_prudent_unsafe_mut as unsafe_mut,
            internal_prudent_unsafe_val as unsafe_val,
            internal_prudent_unsafe_set as unsafe_set
        };
    };
}

/// Re-export
/// 1. non-macros from the given `linted`_module/path. And
/// 2. anything needed from `unlinted`.
///
/// NOT a part of public API - internal.
#[doc(hidden)]
#[macro_export]
// `cargo fmt` removes `{` and `}` around PRUDENT_INTERNAL_LINTED_VERSION, which fails to compile. See
// https://github.com/rust-lang/rustfmt/issues/6047.
#[rustfmt::skip]
macro_rules! reexport_non_macros {
    ($linted_path:path) => {
        #[allow(unused)]
        pub use $linted_path::{PRUDENT_INTERNAL_LINTED_VERSION};

        #[allow(unused)]
        pub use ::prudent::unlinted::*;
    };
}
