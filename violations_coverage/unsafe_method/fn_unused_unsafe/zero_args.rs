use prudent::unsafe_method;

prudent::unsafe_method_macro_definition!();

#[allow(unused_macros)]
#[macro_export]
macro_rules! unsafe_expr {
    ($e:expr) => {({
        #[allow(unused_unsafe)]
        #[deny(unused_unsafe)]
        let result = unsafe { 2+3 };
        //let result = unsafe { $e };
        result
    })}
}

fn main() {
    //let _ = unsafe_expr!( 1+2 );

    println!( hi!() );
    //let _ = unsafe_method!("hi", len);
    let _ = unsafe_method!("hi", len);
}
