//! Utilities for testing.

use std::fmt::Debug;
use util::tok::Tok;

// a simple tokenizer
pub mod tok;

pub fn test<R:Debug+Eq,F>(parse_fn: F,
                          input: &str,
                          expected: R)
    where F: FnOnce(Vec<Tok>) -> Result<(Option<Tok>,R),Option<Tok>>
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse
    let (lookahead, r) = parse_fn(tokens).unwrap();

    // expect input to be completely consumed
    assert!(lookahead.is_none(), "input not completely consumed");

    // expect output to be correct
    assert!(r == expected, "parsing {:?}, got {:#?}, expected {:#?}", input, r, expected);
}
