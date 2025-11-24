::prudent::load!(any: "linted.rs");
use crate::prudent::*;

fn main() {
    let _ = unsafe_method!("hi", len);
}
