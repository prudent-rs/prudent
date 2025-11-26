::prudent::load!(any: "linted.rs");
use crate::prudent::*;

struct S;

impl S {
    unsafe fn method(&self) {}
}

unsafe fn new_receiver() -> S {
    unreachable!();
}

fn main() {
    let _ = unsafe_method!(new_receiver(), method);
}
