::prudent::load!(any: "internal_front_end.rs");
use self::prudent::*;

fn main() {
    #[allow(unused_unsafe)]
    // str::len is actually not unsafe, but that doesn't matter for this example
    let _ = unsafe_method!(core::str::from_utf8_unchecked(b"hello"), len);
}
