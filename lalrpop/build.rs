extern crate lalrpop_snap;

fn main() {
    lalrpop_snap::Configuration::new()
        .generate_in_source_tree()
        .process()
        .unwrap();
}
