//! Utilities for testing.

use std::fmt::Debug;
use util::tok::Tok;
use lalrpop_util::ParseError;

// a simple tokenizer
pub mod tok;

pub fn test<R:Debug+Eq,F>(parse_fn: F,
                          input: &str,
                          expected: R)
    where F: FnOnce(Vec<Tok>) -> Result<R,ParseError<(),Tok,()>>
{
    // create tokens
    let tokens = tok::tokenize(input);

    // filter to tokens
    let tokens = tokens.into_iter().map(|(_, tok, _)| tok).collect();

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(r == expected, "parsing {:?}, got {:#?}, expected {:#?}", input, r, expected);
}

pub fn test_loc<R:Debug+Eq,F>(parse_fn: F,
                              input: &str,
                              expected: R)
    where F: FnOnce(Vec<(usize, Tok, usize)>) -> Result<R, ParseError<usize, Tok, ()>>
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(r == expected, "parsing {:?}, got {:#?}, expected {:#?}", input, r, expected);
}
