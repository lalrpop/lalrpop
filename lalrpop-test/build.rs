fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .force_build(true)
        .unit_test()
        .log_debug()
        .process_dir("src")
        .unwrap();

    lalrpop::Configuration::new()
        .emit_comments(false)
        .force_build(true)
        .process_dir("benches")
        .unwrap()
}
