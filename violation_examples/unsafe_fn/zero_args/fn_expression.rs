use prudent::unsafe_fn;

unsafe fn fn_itself() {}

unsafe fn get_fn_itself() -> unsafe fn() {
    fn_itself
}

fn main() {
    unsafe_fn!( get_fn_itself() );
}
