#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
// Need this for rusty_peg
#![recursion_limit = "256"]
// I hate this lint.
#![allow(unused_parens)]
// The builtin tests don't cover the CLI and so forth, and it's just
// too darn annoying to try and make them do so.
//
// ε shows up in lalrpop/src/lr1/example/test.rs
#![cfg_attr(test, allow(dead_code, mixed_script_confusables))]
#![warn(rust_2018_idioms)]
#![deny(clippy::exit)]
#![warn(clippy::cargo)]
// This is implied by clippy::cargo, but the version overlap may happen deep in
// our dependency tree and there's little we can do about it.
#![allow(clippy::multiple_crate_versions)]

// hoist the modules that define macros up earlier
#[macro_use]
mod rust;
#[macro_use]
mod log;

mod api;
mod build;
mod collections;
mod file_text;
mod grammar;
mod kernel_set;
mod lexer;
mod lr1;
mod message;
mod normalize;
mod parser;
mod session;
mod tls;
mod tok;
mod util;

#[cfg(test)]
mod generate;
#[cfg(test)]
mod test_util;

pub use crate::api::process_root;
#[allow(deprecated)]
pub use crate::api::process_root_unconditionally;
pub use crate::api::process_src;
pub use crate::api::Configuration;
use ascii_canvas::style;
