use prudent::unsafe_method;

fn main() {
    #[allow(unused_unsafe)]
    let _ = unsafe_method!(core::str::from_utf8_unchecked(b"hi"), len);
}
