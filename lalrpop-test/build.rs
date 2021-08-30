fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .force_build(true)
        .unit_test()
        .log_debug()
        .process_current_dir()
        .unwrap();
}
