#[cfg(any(debug_assertions, not(_overflow_checks)))]
compile_error!("cargo run --profile custom");

const ARBITRARY: u8 = 42;

fn main() {
    //this UB™, we need to "touch" trivial before doing assume_init
    let trivial: u8 = safe_uninit::safe_uninit();
    if (trivial == ARBITRARY) == (trivial != ARBITRARY) {
        println!("observed contradiction!");
    } else {
        unreachable!("llvm changed its mind");
    }
}
