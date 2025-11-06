use prudent::unsafe_method;

pub fn invoke() {
    let _ = unsafe_method!(1u8, unchecked_add, 0);
}

/*
pub fn invoke() {
    let _ = {
        if false {
            let (tuple_tree, receiver) = ((0,), 1u8);
            #[allow(unsafe_code)]
            unsafe {
                receiver.unchecked_add(tuple_tree.0)
            }
            ::core::panicking::panic("internal error: entered unreachable code")
        } else {
            1u8.unchecked_add(0)
        }
    };
}
*/