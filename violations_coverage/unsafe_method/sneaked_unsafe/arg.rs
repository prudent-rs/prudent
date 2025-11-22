::prudent::load!("internal_front_end.rs");
use self::prudent::*;

fn main() {
    let _ = unsafe_method!(1u8, unchecked_add, 0u8.unchecked_add(0));
}
