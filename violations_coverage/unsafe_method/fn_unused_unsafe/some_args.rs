use prudent::unsafe_method;

struct S;
impl S {
    fn use_it(&self, _: bool) {}
}

fn main() {
    let s = S;
    unsafe_method!(s =>. use_it => true);
}
