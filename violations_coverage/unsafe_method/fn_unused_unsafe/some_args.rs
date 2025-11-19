::prudent::load!("../../../../src/internal_front_end.rs");
use self::prudent::*;

struct S;
impl S {
    fn use_it(_: bool) {}
}

fn main() {
    let s = S;
    unsafe_method!(s, use_it, true);
}
