//! We can NOT wilcard `t.compile_fail("../in_crate/src/bin/*");`` because each file calls
//! `::prudent::load!(...)` and then there would be conflicts of multiple definitions of the same
//! macros.

#[test]
fn unsafe_method_fn_unused_unsafe_zero_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("violations/unsafe_method-fn_unused_unsafe-zero_args.rs");
}

#[test]
fn unsafe_method_fn_unused_unsafe_some_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("violations/unsafe_method-fn_unused_unsafe-some_args.rs");
}
//---------
