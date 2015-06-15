#![recursion_limit="256"]

#[macro_use]
extern crate rusty_peg;

extern crate regex;

mod grammar;
mod intern;
mod normalize;
mod parser;

fn main() {
    println!("Hello, world!");
}
