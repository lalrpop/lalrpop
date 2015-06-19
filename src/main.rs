#![recursion_limit="256"]

#[macro_use]
extern crate rusty_peg;
extern crate diff;

extern crate regex;

mod grammar;
mod intern;
mod lr1;
mod normalize;
mod parser;
mod util;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
