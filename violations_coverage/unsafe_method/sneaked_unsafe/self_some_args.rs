use prudent::unsafe_method;

fn main() {
    #[allow(unused_unsafe)]
    let _ = unsafe_method!(0u8.unchecked_add(0), unchecked_add, 0);
}
