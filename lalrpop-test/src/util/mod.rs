//! Utilities for testing.

use crate::util::tok::Tok;
use diff;
use lalrpop_util::ParseError;
use std::fmt::{Debug, Error, Formatter};

// a simple tokenizer
pub mod tok;

pub fn test<'input, R: Debug + Eq, F>(parse_fn: F, input: &'input str, expected: R)
where
    F: FnOnce(Vec<Tok<'input>>) -> Result<R, ParseError<(), Tok<'input>, &'static str>>,
{
    // create tokens
    let tokens = tok::tokenize(input);

    // filter to tokens
    let tokens = tokens.into_iter().map(|(_, tok, _)| tok).collect();

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(
        r == expected,
        "parsing {:?}, got {:#?}, expected {:#?}",
        input,
        r,
        expected
    );
}

pub fn test_loc<R: Debug + Eq, F>(parse_fn: F, input: &str, expected: R)
where
    F: FnOnce(Vec<(usize, Tok, usize)>) -> Result<R, ParseError<usize, Tok, &'static str>>,
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse, expecting input to be totally consumed
    let r = parse_fn(tokens).unwrap();

    // expect output to be correct
    assert!(
        r == expected,
        "parsing {:?}, got {:#?}, expected {:#?}",
        input,
        r,
        expected
    );
}

pub fn test_err_gen<'input, R, F>(parse_fn: F, input: &'input str) -> R
where
    F: FnOnce(Vec<(usize, Tok<'input>, usize)>) -> R,
{
    // create tokens
    let tokens = tok::tokenize(input);

    // parse, expecting input to be totally consumed
    parse_fn(tokens)
}

struct ExpectedDebug<'a>(&'a str);

impl<'a> Debug for ExpectedDebug<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        // Ignore trailing commas in multiline Debug representation.
        // Needed to work around rust-lang/rust#59076.
        let s = self.0.replace(",\n", "\n");
        write!(fmt, "{}", s)
    }
}

pub fn expect_debug<D: Debug>(actual: D, expected: &str) {
    compare(
        ExpectedDebug(&format!("{:#?}", actual)),
        ExpectedDebug(expected),
    )
}

pub fn compare<D: Debug, E: Debug>(actual: D, expected: E) {
    let actual_s = format!("{:?}", actual);
    let expected_s = format!("{:?}", expected);
    if actual_s != expected_s {
        let actual_s = format!("{:#?}", actual);
        let expected_s = format!("{:#?}", expected);

        compare_str(&actual_s, &expected_s, "");
    }
}

pub fn compare_str(actual: &str, expected: &str, msg: &str) {
    if actual != expected {
        let lines = diff::lines(actual, expected);
        for diff in lines.iter().take(100) {
            match diff {
                diff::Result::Right(r) => println!("- {}", r),
                diff::Result::Left(l) => println!("+ {}", l),
                diff::Result::Both(l, _) if lines.len() < 100 => println!("  {}", l),
                _ => (),
            }
        }

        if lines.len() >= 100 {
            println!("... more");
        }
        panic!("{}", msg);
    }
}
