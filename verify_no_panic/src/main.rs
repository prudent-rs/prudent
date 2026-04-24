#[cfg(debug_assertions)]
compile_error!("Build in release profile only.");

fn divide() {
     //core::hint::black_box(1 / core::hint::black_box(1));
     core::hint::black_box( 1/1 );
}

fn slice_access() {
    const A: [bool; 2] = [true, false];
    let s = core::hint::black_box( &A[..] );
    core::hint::black_box( s[0] );
}

//#[::no_panic::no_panic]
fn main() {
    divide();
    slice_access();
}