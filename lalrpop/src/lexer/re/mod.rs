//! A parser and representation of regular expressions.

use regex_syntax::{self, ClassRange, Expr};
use std::char;
use std::fmt::{Debug, Formatter, Error};

#[cfg(test)]
mod test;

pub type Regex = Expr;
pub type RegexError = regex_syntax::Error;

/// Range of characters, inclusive. Note that this range may contain
/// some endpoints that are not valid unicode, hence we store u32.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test {
    pub start: u32,
    pub end: u32,
}

/// Convert a string literal into a parsed regular expression.
pub fn parse_literal(s: &str) -> Regex {
    match parse_regex(&regex_syntax::quote(s)) {
        Ok(v) => v,
        Err(_) => panic!("failed to parse literal regular expression")
    }
}

/// Parse a regular expression like `a+` etc.
pub fn parse_regex(s: &str) -> Result<Regex, RegexError> {
    Expr::parse(s)
}

impl Test {
    pub fn char(c: char) -> Test {
        let c = c as u32;
        Test { start: c, end: c + 1 }
    }

    pub fn inclusive_range(s: char, e: char) -> Test {
        Test { start: s as u32, end: e as u32 + 1 }
    }


    pub fn exclusive_range(s: char, e: char) -> Test {
        Test { start: s as u32, end: e as u32 }
    }

    pub fn is_char(self) -> bool {
        self.len() == 1
    }

    pub fn len(self) -> u32 {
        self.end - self.start
    }

    pub fn contains_u32(self, c: u32) -> bool {
        c >= self.start && c < self.end
    }

    pub fn contains_char(self, c: char) -> bool {
        self.contains_u32(c as u32)
    }

    pub fn intersects(self, r: Test) -> bool {
        !self.is_empty() && !r.is_empty() && (
            self.contains_u32(r.start) || r.contains_u32(self.start))
    }

    pub fn is_disjoint(self, r: Test) -> bool {
        !self.intersects(r)
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl From<ClassRange> for Test {
    fn from(range: ClassRange) -> Test {
        Test::inclusive_range(range.start, range.end)
    }
}

impl Debug for Test {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match (char::from_u32(self.start), char::from_u32(self.end)) {
            (Some(start), Some(end)) => {
                if self.is_char() {
                    if ".[]()?+*!".contains(start) {
                        write!(fmt, "\\{}", start)
                    } else {
                        write!(fmt, "{}", start)
                    }
                } else {
                    write!(fmt, "[{:?}..{:?}]", start, end)
                }
            }
            _ => {
                write!(fmt, "[{:?}..{:?}]", self.start, self.end)
            }
        }
    }
}

