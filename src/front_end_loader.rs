// - "frontend" macros can (and do) invoke "backend" macros, but not the other way. That's because
//   "backend" macros don't know the crate name where "frontend" macros were loaded.
// - "frontend" macros can be recursive (between themselves, but not through "backend" macros")
/// Invoke from the top level of your crate, and only once (once per crate).
#[macro_export]
macro_rules! load {
    // No lints.
    //
    // re-export all (both "backend" and "frontend") from `prudent` crate under module `prudent`.
    () => {
        $crate::load!( prudent );
    };
    // No lints.
    //
    // re-export all (both "backend" and "frontend") from `prudent` crate, under given module name.
    ( $module_name:ident ) => {
        /// prudent exported in a module
        mod $module_name {
            use ::prudent::internal_front_end::*;
            $crate::reexport_macros!();
            $crate::reexport_non_macros!( ::prudent::internal_front_end );
        }
    };// -----------

    // load "frontend" from the given file; re-export "backend" from `prudent` crate; blend them
    // under a module with the given name. If `$module_name` is not given, it's `prudent`.
    //
    // Since `any` is **not** specified, this applies to `$[cfg(doctest)]` and `$[cfg(test)` only.
    // - under `$[cfg(doctest)]` the exported module is `pub` (because doctests are in separate
    //   crate(s))
    // - under `$[cfg(test)]` the exported module is private
    ( $prudent_front_end_first:literal
      $( $prudent_front_end_second:literal )?
      $( -> $module_name:ident )?
    ) => {
        $crate::load!( ~
            (doctest,test) :
            $prudent_front_end_first
            $( $prudent_front_end_second )?
            $( -> $module_name )?
        );
    };
    // `any`` doesn't mean any of `doctest` and/or `test`, but it means any
    // build/`$cfg`/profile/target...
    ( any :
      $prudent_front_end_first:literal
      $( $prudent_front_end_second:literal )?
      $( -> $module_name:ident )?
     ) => {
        // @TODO test:
        $crate::load!( ~
            (test,not(test)) :
             $prudent_front_end_first
             $( $prudent_front_end_second )?
             $( -> $module_name )?
        );
    };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_front_end_first:literal
      $( $prudent_front_end_second:literal )?
    ) => {
        $crate::load!( ~
            ( $( $cfg_filter )* ) :
            $prudent_front_end_first
            $( $prudent_front_end_second )?
            -> prudent
        );
      };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_front_end_same:literal
      -> $module_name:ident
    ) => {
        $crate::load!( ~
            ( $( $cfg_filter )* ) :
            $prudent_front_end_same
            $prudent_front_end_same
            -> $module_name
        );
    };
    ( ~
      ( $( $cfg_filter:tt )* ) :
      $prudent_front_end_first:literal
      $prudent_front_end_second:literal
      -> $module_name:ident
    ) => {
        #[cfg(not(windows))]
        $crate::load!( ~~
            ( $( $cfg_filter )* ) :
            $prudent_front_end_first
            -> $module_name
        );

        #[cfg(windows)]
        $crate::load!( ~~
            ( $( $cfg_filter )* ) :
            $prudent_front_end_second
            -> $module_name
        );
    };
    ( ~~
      ( $( $cfg_filter:tt )* ) :
      $prudent_front_end:literal
      -> $module_name:ident
    ) => {
        // We do NOT load the file into a separate submodule, like:
        // ```
        // #[cfg(any( $( $cfg_filter )* ))]
        // #[allow(unused)]
        // #[path = $prudent_front_end]
        // mod internal_prudent_front_end_loaded_or_aliased;
        // ``
        // Why? because https://github.com/rust-lang/rust/issues/52234 makes it very difficult (or
        // impossible?) to re-export such macros. But using
        // `#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]` didn't help, so
        // there may be other restrictions, too.
        // Life is too short. So we ARE using ::core::include!(...);
        /*#[cfg(any( $( $cfg_filter )* ))]
        #[allow(unused)]
        mod internal_prudent_front_end_loaded_or_aliased {
            ::core::include!( $prudent_front_end);
        }*/

        #[cfg(any( $( $cfg_filter )* ))]
        #[allow(unused)]
        #[path = $prudent_front_end]
        pub mod internal_prudent_front_end_loaded_or_aliased;

        //compile_error!("WOULD RE-EXPORT INSTEAD OF LOAD FRONTEND");

        #[cfg(not(any( $( $cfg_filter )* )))]
        #[allow(unused)]
        pub use ::prudent::internal_front_end as internal_prudent_front_end_loaded_or_aliased;

        // For non-doctest build, the module is private. No need to re-export as public (instead,
        // the clients can load and import this crate themselves).
        #[cfg(not(any(doctest, doc)))]
        mod $module_name {
            /*$crate::load_module_content!(
                ( $( $cfg_filter )* )
                : $prudent_front_end
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

            #[cfg(any( $( $cfg_filter )* ))]
            //#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
            #[allow(unused)]
            use crate::*;

            #[cfg(not(any( $( $cfg_filter )* )))]
            #[allow(unused)]
            use ::prudent::internal_front_end::*;

            //$crate::reexport_macros!();
            //#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

            //pub use crate::internal_prudent_unsafe_fn as unsafe_fn;

            //pub use super::internal_prudent_front_end_loaded_or_aliased::internal_prudent_unsafe_fn_ as unsafe_fn;
            //
            // OR:
            pub use super::internal_prudent_front_end_loaded_or_aliased::internal_prudent_unsafe_fn as unsafe_fn;

            $crate::reexport_non_macros!( super::internal_prudent_front_end_loaded_or_aliased );
        }
        // - For doctests, the module is public, because doctests are build in a separate crate.
        // - For documentation, even though the consumer does NOT access the front end as
        //   `::prudent::prudent::*`, but instead, as `crate::prudent::*` or
        //   `crate::module_name_given::*`.
        #[cfg(any(doctest, doc))]
        pub mod $module_name {
            /*$crate::load_module_content!(
                ( $( $cfg_filter )* )
                : $prudent_front_end
            );*/

            #[cfg(any( $( $cfg_filter )* ))]
            //#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
            #[allow(unused)]
            use crate::*;

            #[cfg(not(any( $( $cfg_filter )* )))]
            #[allow(unused)]
            use ::prudent::internal_front_end::*;

            //$crate::reexport_macros!();
            //#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

            //pub use crate::internal_prudent_unsafe_fn as unsafe_fn;

            //pub use super::internal_prudent_front_end_loaded_or_aliased::internal_prudent_unsafe_fn_ as unsafe_fn;
            //
            // OR:
            pub use super::internal_prudent_front_end_loaded_or_aliased::internal_prudent_unsafe_fn as unsafe_fn;

            $crate::reexport_non_macros!( super::internal_prudent_front_end_loaded_or_aliased );
        }

        /*const _VERIFY_VERSION: () = {
            ::prudent::back_end::verify_front_end_version( $module_name::INTERNAL_FRONT_END_VERSION );
        };*/
    }
}

/// Shared functionality between
/// - public module (for `$[cfg(doctest)]`) - public, because doctests are build in a separate
///   crate; and
/// - private module (for $[cfg(test)]` or "any" builds).
///
/// NOT a part of public API - internal.
#[doc(hidden)]
#[macro_export]
macro_rules! load_module_content {
    (
        ( $( $cfg_filter:tt )* ) :
        $prudent_front_end:literal
    ) => {
        // @TODO const time validation
        //
        // #[cfg(any( $( $cfg_filter )* ))]
        //
        //include_str! -> substring: first line (a comment with version)

        // `prudent`'s "frontend" macros, loaded into the user's crate
    };
}

/// Re-export `front_end_loaded_or_aliased` and `::prudent::back_end`.
///
/// NOT a part of public API - internal.
#[doc(hidden)]
#[macro_export]
macro_rules! reexport_macros {
    () => {
        //pub use internal_prudent_unsafe_fn as unsafe_fn;
        #[allow(unused)]
        pub use {
        //pub use crate::{
            //internal_prudent_unsafe_fn as unsafe_fn,
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

#[doc(hidden)]
#[macro_export]
macro_rules! reexport_non_macros {
    ($front_end_path:path) => {
        #[allow(unused)]
        pub use $front_end_path::{INTERNAL_FRONT_END_VERSION};

        #[allow(unused)]
        pub use ::prudent::back_end::*;
    };
}
