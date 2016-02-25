//! Utilities for testing.

use diff;
use lalrpop_util::ParseError;
use std::fmt::{Debug, Formatter, Error};
use util::tok::Tok;

// a simple tokenizer
pub mod tok;

pub fn test<R: Debug + Eq, F>(parse_fn: F, input: &str, expected: R)
    where F: FnOnce(Vec<Tok>) -> Result<R, ParseError<(), Tok, ()>>
{
    // create tokens
    let tokens = tok::tokenize(input);

    // filter to tokens
    let tokens = tokens.into_iter().map(|(_, tok, _)| tok).collect();

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(r == expected,
            "parsing {:?}, got {:#?}, expected {:#?}",
            input,
            r,
            expected);
}

pub fn test_loc<R: Debug + Eq, F>(parse_fn: F, input: &str, expected: R)
    where F: FnOnce(Vec<(usize, Tok, usize)>) -> Result<R, ParseError<usize, Tok, ()>>
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(r == expected,
            "parsing {:?}, got {:#?}, expected {:#?}",
            input,
            r,
            expected);
}

pub fn test_err_gen<R, F>(parse_fn: F, input: &str) -> R
    where F: FnOnce(Vec<(usize, Tok, usize)>) -> R
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse, expecting input to be totally consumed
    parse_fn(tokens)
}

struct ExpectedDebug<'a>(&'a str);

impl<'a> Debug for ExpectedDebug<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.0)
    }
}

pub fn expect_debug<D: Debug>(actual: D, expected: &str) {
    compare(ExpectedDebug(&format!("{:#?}", actual)),
            ExpectedDebug(expected))
}

pub fn compare<D: Debug, E: Debug>(actual: D, expected: E) {
    let actual_s = format!("{:?}", actual);
    let expected_s = format!("{:?}", expected);
    if actual_s != expected_s {
        let actual_s = format!("{:#?}", actual);
        let expected_s = format!("{:#?}", expected);

        for diff in diff::lines(&actual_s, &expected_s) {
            match diff {
                diff::Result::Right(r) => println!("- {}", r),
                diff::Result::Left(l) => println!("+ {}", l),
                diff::Result::Both(l, _) => println!("  {}", l),
            }
        }

        assert!(false);
    }
}
