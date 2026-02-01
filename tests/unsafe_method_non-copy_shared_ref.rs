// RUSTFLAGS="-Znext-solver" cargo +nightly test --features assert_unsafe_methods --test unsafe_method_non-copy_shared_ref

#![cfg_attr(feature = "assert_unsafe_methods", feature(type_alias_impl_trait))]

use prudent::unsafe_method;
struct SNonCopy {}
impl SNonCopy {
    unsafe fn unsafe_method_no_args(&self) {}
    unsafe fn unsafe_method_one_arg(&self, _: bool) {}
    unsafe fn unsafe_method_two_args(&self, _: bool, _: bool) {}
}

#[test]
fn run_test() {
    let s = SNonCopy {};
    unsafe_method!(s =>. unsafe_method_no_args);
    unsafe_method!(s =>. unsafe_method_one_arg => true);
    unsafe_method!(s =>. unsafe_method_two_args => true, false);
}
