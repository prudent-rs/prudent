use prudent::unsafe_method;

fn main() {
    let _ = unsafe_method!(1u8, unchecked_add, 0u8.unchecked_add(0));
}
