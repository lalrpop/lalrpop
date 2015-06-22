#![crate_name="sub"]

extern crate util;

mod sub;

fn main() {
    util::main(sub::parse_S);
}
