use prudent::unsafe_fn;

unsafe fn add_four(left: u64, middle_left:u64, middle_right:u64, right: u64) -> u64 {
    left + middle_left + middle_right + right
}

fn main() {
    unsafe_fn!(add_four, 1, 2, 3, 4);
}

