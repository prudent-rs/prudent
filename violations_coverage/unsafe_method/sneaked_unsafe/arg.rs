::prudent::load!(any: "frontend_linted.rs");
use crate::prudent::*;

fn main() {
    let _ = unsafe_method!(1u8 =>@ unchecked_add => 0u8.unchecked_add(0));
}
