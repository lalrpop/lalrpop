// Need this for rusty_peg
#![recursion_limit="256"]

// I hate this lint.
#![allow(unused_parens)]

// The builtin tests don't cover the CLI and so forth, and it's just
// too darn annoying to try and make them do so.
#![cfg_attr(test, allow(dead_code))]

extern crate diff;
extern crate lalrpop_intern as intern;
extern crate lalrpop_util;
extern crate petgraph;
extern crate regex;
extern crate itertools;
extern crate unicode_xid;

#[cfg(test)]
extern crate rand;

// rust exports a macro that others use, so hoist it early.
#[macro_use]
mod rust;

mod build;
mod grammar;
mod lexer;
mod lr1;
mod normalize;
mod parser;
mod kernel_set;
mod tok;
mod util;

#[cfg(test)] mod generate;
#[cfg(test)] mod test_util;

pub use build::process_root;
pub use build::process_root_unconditionally;
