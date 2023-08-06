fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .process_current_dir()
        .unwrap();
}
