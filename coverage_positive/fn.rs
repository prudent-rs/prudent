use prudent::unsafe_fn;

const unsafe fn unsafe_fn_no_args() {}
const unsafe fn unsafe_fn_one_arg(b: bool) -> bool { b }
const unsafe fn unsafe_fn_two_args(_: bool, u: u8) -> u8 { u }

const _: () = unsafe_fn!(unsafe_fn_no_args);
const _: bool = unsafe_fn!(unsafe_fn_one_arg=> true);
const _: u8 = unsafe_fn!(unsafe_fn_two_args=> true, 0);
fn main() {}
