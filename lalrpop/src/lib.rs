// Need this for rusty_peg
#![recursion_limit="256"]

// I hate this lint.
#![allow(unused_parens)]

// The builtin tests don't cover the CLI and so forth, and it's just
// too darn annoying to try and make them do so.
#![cfg_attr(test, allow(dead_code))]

extern crate bit_set;
#[macro_use] extern crate bitflags;
extern crate diff;
extern crate lalrpop_intern as intern;
extern crate lalrpop_util;
extern crate petgraph;
extern crate regex;
extern crate itertools;
extern crate term;
extern crate time;
extern crate unicode_xid;

#[cfg(test)]
extern crate rand;

// hoist the modules that define macros up earlier
#[macro_use]
mod rust;
#[macro_use]
mod log;

mod ascii_canvas;
mod build;
mod file_text;
mod grammar;
mod lexer;
mod lr1;
mod message;
mod normalize;
mod parser;
mod kernel_set;
mod session;
pub mod style;
mod tls;
mod tok;
mod util;

#[cfg(test)] mod generate;
#[cfg(test)] mod test_util;

pub use build::process_root;
pub use build::process_root_unconditionally;
pub use build::process_file;
pub use log::Level;
pub use log::Log;
pub use session::Session;
