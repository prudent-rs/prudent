use prudent::unsafe_fn;

unsafe fn fn_itself(b: bool) -> bool {
    b
}

unsafe fn get_fn_itself() -> unsafe fn(b: bool) -> bool {
    fn_itself
}

fn main() {
    unsafe_fn!(get_fn_itself() => true);
}
