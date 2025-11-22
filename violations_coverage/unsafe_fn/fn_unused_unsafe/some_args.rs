::prudent::load!(any: "../../../../src/internal_front_end.rs");
use self::prudent::*;

fn safe_fn_one_arg(_: bool) {}

fn main() {
    unsafe_fn!(safe_fn_one_arg, true);
}
