fn main() {
    if std::panic::catch_unwind(|| {
        #[allow(arithmetic_overflow)]
        let _ = 255_u8 + 1;
    })
    .is_err()
    {
        println!("cargo:rustc-cfg=_overflow_checks");
    }
}
