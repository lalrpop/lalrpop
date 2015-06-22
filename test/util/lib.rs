//! Utilities for testing.

#![crate_type="rlib"]
#![crate_name="util"]

use std::env;

use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
use tok::Tok;

// a simple tokenizer
pub mod tok;

// a generic main fn suitable for being executed from the makefile
pub fn main(parse_fn: fn(Vec<Tok>) -> Result<(Option<Tok>,()),Option<Tok>>) {
    let mut args = env::args().skip(1);

    // read the input file
    let mut input = String::new();
    match args.next() {
        Some(input_path) => {
            let mut f = File::open(&input_path).unwrap();
            f.read_to_string(&mut input).unwrap();
        }
        None => {
            let mut s = io::stdin();
            s.read_to_string(&mut input).unwrap();
        }
    }

    // create tokens
    let tokens = tok::tokenize(&input);

    // parse
    let (lookahead, ()) = parse_fn(tokens).unwrap();

    // expect input to be completely consumed
    if lookahead.is_some() {
        println!("input not completely consumed");
        exit(1);
    }

    println!("input ok");
}
