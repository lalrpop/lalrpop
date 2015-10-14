//! A parser and representation of regular expressions.

use std::fmt::{Debug, Display, Formatter, Error};
use std::str::CharIndices;

#[cfg(test)]
mod test;

pub struct Regex {
    pub alternatives: Vec<Alternative>
}

pub struct Alternative {
    pub elems: Vec<Elem>
}

pub enum Elem {
    Any,
    Test(Test),
    Group(Regex),
    NotGroup(Regex),
    Repeat(RepeatOp, Box<Elem>),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Test {
    Char(char), // some specific character
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RepeatOp {
    Plus,     // R+     1 .. infinity
    Question, // R?     0 .. 1
    Star,     // R*     0 .. infinity
}

#[derive(Debug)]
pub struct RegexError {
    pub message: String,
    pub offset: usize,
}

macro_rules! return_err {
    ($offset: expr, $($args:expr),+) => {
        return Err(RegexError {
            message: format!($($args),+),
            offset: $offset
        });
    }
}

/// Convert a string literal into a parsed regular expression.
pub fn parse_literal(s: &str) -> Regex {
    Regex {
        alternatives: vec![
            Alternative {
                elems: s.chars().map(|c| Elem::Test(Test::Char(c))).collect()
            }]
    }
}

/// Parse a regular expression like `a+` etc.
pub fn parse_regex(s: &str) -> Result<Regex, RegexError> {
    let mut parser = RegexParser::new(s.char_indices(), s.len());
    let regex = try!(parser.regex());

    // we should have reached the end of the string:
    match parser.lookahead {
        Some((index, c)) => return_err!(index, "unexpected '{}'", c),
        None => Ok(regex),
    }
}

struct RegexParser<'str> {
    lookahead: Option<(usize, char)>,
    length: usize, // length of string in total
    chars: CharIndices<'str>,
}

// little recursive descent parser
impl<'str> RegexParser<'str> {
    fn new(mut chars: CharIndices<'str>, length: usize) -> RegexParser<'str> {
        let lookahead = chars.next();
        RegexParser { lookahead: lookahead, chars: chars, length: length }
    }

    fn bump(&mut self) {
        self.lookahead = self.chars.next();
    }

    fn expect(&mut self, c: char) -> Result<(), RegexError> {
        match self.lookahead {
            Some((_, d)) if c == d => Ok(self.bump()),
            Some((index, d)) => return_err!(index, "expected '{}', but found '{}'", c, d),
            _ => return_err!(self.length, "expected '{}', but regex ended", c)
        }
    }

    fn regex(&mut self) -> Result<Regex, RegexError> {
        let mut alternatives = vec![];

        loop {
            alternatives.push(try!(self.alternative()));

            match self.lookahead {
                Some((_, '|')) => { continue; }
                _ => { break; }
            }
        }

        Ok(Regex { alternatives: alternatives })
    }

    fn alternative(&mut self) -> Result<Alternative, RegexError> {
        let mut elems = vec![];
        while let Some((index, c)) = self.lookahead {
            match c {
                '\\' => { elems.push(try!(self.escape(index))); }
                '*'  => { try!(self.repeat(&mut elems, index, RepeatOp::Star)); }
                '+'  => { try!(self.repeat(&mut elems, index, RepeatOp::Plus)); }
                '?'  => { try!(self.repeat(&mut elems, index, RepeatOp::Question)); }
                '('  => { elems.push(try!(self.group())); }
                ')'  => { break; }
                '['  => { elems.push(try!(self.range(index))); }
                ']'  => { break; }
                '|'  => { break; }
                '.'  => { self.bump(); elems.push(Elem::Any); }
                _    => { self.bump(); elems.push(Elem::Test(Test::Char(c))); }
            }
        }
        Ok(Alternative { elems: elems })
    }

    fn escape(&mut self, index: usize) -> Result<Elem, RegexError> {
        self.bump(); // consume the '\\'
        match self.lookahead {
            None => return_err!(index, "escape at end of str"),
            Some((_, '!')) => {
                // (*) afaik, \! is not particularly meaningful or
                // useful in ordinary regular expressions, and there
                // doesn't seem to be any decent way to express
                // something like [^abc], so overload it

                self.bump();
                try!(self.expect('('));
                let regex = try!(self.regex());
                try!(self.expect(')'));
                Ok(Elem::NotGroup(regex))
            }
            Some((_, c)) => {
                self.bump();
                Ok(Elem::Test(Test::Char(c))) // FIXME there are other escapes, like \s
            }
        }
    }

    fn repeat(&mut self, elems: &mut Vec<Elem>, index: usize, op: RepeatOp) -> Result<(), RegexError> {
        self.bump(); // consume the `*`, `+`, `?`, etc
        match elems.pop() {
            Some(e) => Ok(elems.push(Elem::Repeat(op, Box::new(e)))),
            None => return_err!(index, "modifier `{}` has nothing to modify", op),
        }
    }

    fn group(&mut self) -> Result<Elem, RegexError> {
        self.bump(); // consume the `(`
        let regex = try!(self.regex());
        try!(self.expect(')'));
        Ok(Elem::Group(regex))
    }

    fn range(&mut self, index: usize) -> Result<Elem, RegexError> {
        // FIXME: This only supports the basic stuff, not the fancy
        // things like [:alpha:] or [.ch.], whatever the heck THAT is!

        self.bump(); // consume the `[`

        let negative = match self.lookahead {
            Some((_, '^')) => { self.bump(); true }
            _ => { false }
        };

        let mut chars = vec![];

        // as a special case, a `]` or `-` first in the list is treated as itself
        match self.lookahead {
            Some((_, ']')) => { self.bump(); chars.push(']'); }
            _ => { }
        }

        while let Some((index, c)) = self.lookahead {
            match c {
                '-' => {
                    self.bump();
                    try!(self.range_dash(index, &mut chars));
                }

                ']' => {
                    self.bump();

                    // "desugar" the range into `|` notation, so that
                    // [abc] becomes (a|b|c)
                    let regex = Regex {
                        alternatives: chars.into_iter()
                                           .map(|c| Alternative {
                                               elems: vec![Elem::Test(Test::Char(c))]
                                           })
                                           .collect()
                    };

                    return if negative {
                        Ok(Elem::NotGroup(regex))
                    } else {
                        Ok(Elem::Group(regex))
                    };
                }

                _ => {
                    self.bump();
                    chars.push(c);
                }
            }
        }

        return_err!(index, "unterminated `[`");
    }

    fn range_dash(&mut self, index: usize, chars: &mut Vec<char>) -> Result<(), RegexError> {
        // as a special case, a `-` at the beginning of the range is treated as itself
        if chars.is_empty() {
            chars.push('-');
            return Ok(());
        }

        // similarly, a - as the last thing in the range is itself
        let end_char = match self.lookahead {
            None | Some((_, ']')) => {
                chars.push('-');
                return Ok(());
            }

            Some((_, d)) => {
                self.bump();
                d
            }
        };

        // otherwise, we will expand the last character we've seen
        let start_char = *chars.last().unwrap();
        let start_index = start_char as u32;
        if start_index >= 128 {
            return_err!(index, "dash notation only supported for ASCII");
        }

        let end_index = end_char as u32;
        if end_index >= 128 {
            return_err!(index, "dash notation only supported for ASCII");
        }

        if start_index > end_index {
            return_err!(index, "start of range is higher than the end of range");
        }

        for c in (start_index as u8)+1 .. (end_index as u8)+1 {
            chars.push(c as char);
        }

        Ok(())
    }
}

impl Debug for RepeatOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for RepeatOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            RepeatOp::Plus => write!(fmt, "+"),
            RepeatOp::Star => write!(fmt, "*"),
            RepeatOp::Question => write!(fmt, "?"),
        }
    }
}

impl Debug for Regex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if !self.alternatives.is_empty() {
            try!(write!(fmt, "{:?}", self.alternatives[0]));
            for alt in &self.alternatives[1..] { try!(write!(fmt, "|{:?}", alt)); }
        } else {
            try!(write!(fmt, "()"));
        }
        Ok(())
    }
}

impl Debug for Alternative {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        for elem in &self.elems {
            try!(write!(fmt, "{:?}", elem));
        }
        Ok(())
    }
}

impl Debug for Elem {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Elem::Any => write!(fmt, "."),
            Elem::Test(Test::Char(c)) => {
                if ".[]()?+*!".contains(c) {
                    write!(fmt, "\\{}", c)
                } else {
                    write!(fmt, "{}", c)
                }
            }
            Elem::Group(ref regex) => write!(fmt, "({:?})", regex),
            Elem::NotGroup(ref regex) => write!(fmt, "\\!({:?})", regex), // see (*) above
            Elem::Repeat(op, ref elem) => write!(fmt, "{:?}{}", elem, op),
        }
    }
}

impl Test {
    pub fn meets(self, t: Test) -> bool {
        use self::Test::*;
        match (self, t) {
            (Char(c), Char(d)) => c == d,
        }
    }
}
