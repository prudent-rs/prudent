::prudent::load!(any: "linted.rs");
use crate::prudent::*;

fn main() {
    #[allow(unused_unsafe)]
    let _ = unsafe_method!(0u8.unchecked_add(0), unchecked_add, 0);
}
