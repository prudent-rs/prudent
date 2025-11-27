::prudent::load!(any: "linted.rs");
use crate::prudent::*;

struct S;

const S_INSTANCE: S = S;

impl S {
    pub unsafe fn method(&self, _: bool) {}
}

fn main() {
    let _ = unsafe_method!(~expect_unsafe S_INSTANCE =>@ method => true);
}
