use prudent::unsafe_fn;

unsafe fn add_three(left: u64, middle: u64, right: u64) -> u64 {
    left + middle + right
}

fn main() {
    unsafe_fn!(
        add_three,
        1,
        {
            let _ = core::str::from_utf8_unchecked(b"hi");
            2
        },
        3
    );
}
