extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .force_build(true)
        .unit_test()
        .process_current_dir()
        .unwrap();
}
