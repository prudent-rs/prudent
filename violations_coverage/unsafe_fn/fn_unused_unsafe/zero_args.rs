::prudent::load!(any: "frontend_linted.rs");
use crate::prudent::*;

fn safe_fn_zero_args() {}

fn main() {
    unsafe_fn!(safe_fn_zero_args);
}
