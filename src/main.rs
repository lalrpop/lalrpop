// Need this for rusty_peg
#![recursion_limit="256"]

// I hate this lint.
#![allow(unused_parens)]

#[macro_use]
extern crate rusty_peg;
extern crate diff;
extern crate rand;
extern crate regex;

// rust exports a macro that others use, so hoist it early.
#[macro_use]
mod rust;

mod generate;
mod grammar;
mod intern;
mod lr1;
mod normalize;
mod parser;
mod util;

#[cfg(test)]
mod test_util;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
