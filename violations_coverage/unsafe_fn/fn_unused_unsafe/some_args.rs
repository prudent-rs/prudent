::prudent::load!(any: "frontend_linted.rs");
use crate::prudent::*;

fn safe_fn_one_arg(_: bool) {}

fn main() {
    unsafe_fn!(safe_fn_one_arg=> true);
}
