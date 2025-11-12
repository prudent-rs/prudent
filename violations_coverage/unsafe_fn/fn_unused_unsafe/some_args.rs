use prudent::unsafe_fn;

fn safe_fn_one_arg(_: bool) {}

fn main() {
    unsafe_fn!(safe_fn_one_arg, true);
}
