fn main() {
    // https://github.com/rust-lang/cargo/issues/4789
    // println!("cargo::rustc-cfg=test");
    // std::env::set_var("CARGO_CFG_TEST", "true");
    lalrpop::process_src().unwrap();
}
