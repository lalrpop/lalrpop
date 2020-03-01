use std::{fmt, marker::PhantomData};

use ParseError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'input>(pub usize, pub &'input str);
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.1, formatter)
    }
}

pub struct MatcherBuilder {
    regex_set: regex::RegexSet,
    regex_vec: Vec<regex::Regex>,
}

impl MatcherBuilder {
    pub fn new<S>(exprs: impl IntoIterator<Item = S>) -> Result<MatcherBuilder, regex::Error>
    where
        S: AsRef<str>,
    {
        let exprs = exprs.into_iter();
        let mut regex_vec = Vec::with_capacity(exprs.size_hint().0);
        let mut first_error = None;
        let regex_set_result = regex::RegexSet::new(exprs.scan((), |_, s| {
            regex_vec.push(match regex::Regex::new(s.as_ref()) {
                Ok(regex) => regex,
                Err(err) => {
                    first_error = Some(err);
                    return None;
                }
            });
            Some(s)
        }));

        if let Some(err) = first_error {
            return Err(err);
        }
        let regex_set = regex_set_result?;

        Ok(MatcherBuilder {
            regex_set,
            regex_vec,
        })
    }
    pub fn matcher<'input, 'builder, E>(
        &'builder self,
        s: &'input str,
    ) -> Matcher<'input, 'builder, E> {
        Matcher {
            text: s,
            consumed: 0,
            regex_set: &self.regex_set,
            regex_vec: &self.regex_vec,
            _marker: PhantomData,
        }
    }
}

pub struct Matcher<'input, 'builder, E> {
    text: &'input str,
    consumed: usize,
    regex_set: &'builder regex::RegexSet,
    regex_vec: &'builder Vec<regex::Regex>,
    _marker: PhantomData<fn() -> E>,
}

impl<'input, 'builder, E> Iterator for Matcher<'input, 'builder, E> {
    type Item = Result<(usize, Token<'input>, usize), ParseError<usize, Token<'input>, E>>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text.trim_start();
        let whitespace = self.text.len() - text.len();
        let start_offset = self.consumed + whitespace;
        if text.is_empty() {
            self.text = text;
            self.consumed = start_offset;
            None
        } else {
            let matches = self.regex_set.matches(text);
            if !matches.matched_any() {
                Some(Err(ParseError::InvalidToken {
                    location: start_offset,
                }))
            } else {
                let mut longest_match = 0;
                let mut index = 0;
                for i in matches.iter() {
                    let match_ = self.regex_vec[i].find(text).unwrap();
                    let len = match_.end();
                    if len >= longest_match {
                        longest_match = len;
                        index = i;
                    }
                }
                let result = &text[..longest_match];
                let remaining = &text[longest_match..];
                let end_offset = start_offset + longest_match;
                self.text = remaining;
                self.consumed = end_offset;
                Some(Ok((start_offset, Token(index, result), end_offset)))
            }
        }
    }
}
