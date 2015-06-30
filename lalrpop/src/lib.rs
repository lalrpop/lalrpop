// Need this for rusty_peg
#![recursion_limit="256"]

// I hate this lint.
#![allow(unused_parens)]

// The builtin tests don't cover the CLI and so forth, and it's just
// too darn annoying to try and make them do so.
#![cfg_attr(test, allow(dead_code))]

#[macro_use]
extern crate rusty_peg;
extern crate diff;
extern crate rand;
extern crate regex;
extern crate itertools;

// rust exports a macro that others use, so hoist it early.
#[macro_use]
mod rust;

mod build;
mod cli;

mod grammar;
mod intern;
mod lr1;
mod normalize;
mod parser;
mod kernel_set;
mod token;
mod util;

#[cfg(test)] mod generate;
#[cfg(test)] mod test_util;

pub use build::process_root;
