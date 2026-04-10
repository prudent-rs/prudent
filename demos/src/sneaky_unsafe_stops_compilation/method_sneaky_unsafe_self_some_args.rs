use prudent::prelude::unsafe_method;

fn main() {
    let _ = unsafe_method!(0u8.unchecked_add(0) =>. unchecked_sub => 0);
}
