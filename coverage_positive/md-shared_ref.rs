use crate::prudent::unsafe_method;
// Works for Copy types
const _: u8 = unsafe_method!(                        1u8                    =>@ unchecked_add => 0);
const _: u8 = unsafe_method!(~allow_unsafe           1u8                    =>@ unchecked_add => 0);
const _: u8 = unsafe_method!(~allow_unsafe  unsafe { 1u8.unchecked_add(2) } =>@ unchecked_add => 0);
const _: u8 = unsafe_method!(~expect_unsafe unsafe { 1u8.unchecked_add(2) } =>@ unchecked_add => 0);
  
fn main() {}

