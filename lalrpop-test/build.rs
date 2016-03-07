extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .force_build(true)
        .process_current_dir()
        .unwrap();
}
