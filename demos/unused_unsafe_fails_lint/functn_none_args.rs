use prudent::prelude::unsafe_fn;

fn safe_fn_zero_args() {}

fn main() {
    unsafe_fn!(safe_fn_zero_args);
}
