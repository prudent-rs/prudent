use prudent::unsafe_method;

pub fn invoke() {
    let _ = unsafe_method!(1u8, unchecked_add, 0);
}
