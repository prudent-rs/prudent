::prudent::load!(any: "linted.rs");
use self::prudent::*;

fn safe_fn_zero_args() {}

fn main() {
    unsafe_fn!(safe_fn_zero_args);
}
