::prudent::load!(any: "linted.rs");
use self::prudent::*;

fn main() {
    let _ = unsafe_method!("hi", len);
}
