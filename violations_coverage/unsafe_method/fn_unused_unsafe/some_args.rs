::prudent::load!(any: "frontend_linted.rs");
use crate::prudent::*;

struct S;
impl S {
    fn use_it(&self, _: bool) {}
}

fn main() {
    let s = S;
    unsafe_method!(s =>@ use_it => true);
}
