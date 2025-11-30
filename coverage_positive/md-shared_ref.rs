use prudent::unsafe_method;
// Works for Copy types
const _: u8 = unsafe_method!( 1u8 =>@ unchecked_add => 0 );
  
fn main() {}

