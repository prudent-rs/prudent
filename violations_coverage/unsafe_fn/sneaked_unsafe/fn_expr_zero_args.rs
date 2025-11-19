::prudent::load!("../../../../src/internal_front_end.rs");
use self::prudent::*;

unsafe fn fn_itself() {}

unsafe fn get_fn_itself() -> unsafe fn() {
    fn_itself
}

fn main() {
    unsafe_fn!(get_fn_itself());
}
