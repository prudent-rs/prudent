use prudent::unsafe_fn;

struct S;
impl S {
    fn use_it(_: bool) {}
}

fn main() {
    let s = S;
    unsafe_method!(s, use_it, true);
}
