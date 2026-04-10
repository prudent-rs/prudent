#![forbid(unused_unsafe)]
use prudent::prelude::unsafe_method;

fn main() {
    let _ = unsafe_method!("hi" =>. len);
}
