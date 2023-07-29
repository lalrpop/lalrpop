use std::{fmt, marker::PhantomData};

use crate::ParseError;

use regex_automata::dfa::{dense, Automaton, StartKind};
use regex_automata::nfa::thompson::Config as NfaConfig;
use regex_automata::util::primitives::StateID;
use regex_automata::util::syntax::Config as SyntaxConfig;
use regex_automata::{Anchored, MatchKind};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'input>(pub usize, pub &'input str);
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.1, formatter)
    }
}

pub struct MatcherBuilder {
    dfa: dense::DFA<Vec<u32>>,
    skip_vec: Vec<bool>,
}

impl MatcherBuilder {
    #[allow(clippy::result_large_err)]
    pub fn new<S>(
        exprs: impl IntoIterator<Item = (S, bool)>,
    ) -> Result<MatcherBuilder, dense::BuildError>
    where
        S: AsRef<str>,
    {
        let exprs = exprs.into_iter();
        let mut regex_vec = Vec::with_capacity(exprs.size_hint().0);
        let mut skip_vec = Vec::with_capacity(exprs.size_hint().0);
        for (regex, skip) in exprs {
            regex_vec.push(regex);
            skip_vec.push(skip);
        }

        let enable_unicode = cfg!(feature = "unicode");
        let dfa = dense::Builder::new()
            .configure(
                dense::Config::new()
                    .minimize(true)
                    .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::All),
            )
            .syntax(
                SyntaxConfig::new()
                    .unicode(enable_unicode)
                    .utf8(enable_unicode),
            )
            .thompson(NfaConfig::new().utf8(enable_unicode).shrink(true))
            .build_many(&regex_vec)?;

        Ok(MatcherBuilder { dfa, skip_vec })
    }

    pub fn matcher<'input, 'builder, E>(
        &'builder self,
        text: &'input str,
    ) -> Matcher<'input, 'builder, E> {
        let start = self.dfa.universal_start_state(Anchored::Yes).unwrap();
        Matcher {
            text,
            consumed: 0,
            start,
            dfa: &self.dfa,
            skip_vec: &self.skip_vec,
            _marker: PhantomData,
        }
    }
}

pub struct Matcher<'input, 'builder, E> {
    text: &'input str,
    consumed: usize,
    start: StateID,
    dfa: &'builder dense::DFA<Vec<u32>>,
    skip_vec: &'builder [bool],
    _marker: PhantomData<fn() -> E>,
}

impl<'input, 'builder, E> Iterator for Matcher<'input, 'builder, E> {
    type Item = Result<(usize, Token<'input>, usize), ParseError<usize, Token<'input>, E>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let text = self.text;
            let start_offset = self.consumed;
            if text.is_empty() {
                self.consumed = start_offset;
                return None;
            }

            let mut match_ = None;
            'search: {
                let mut state = self.start;
                for (i, byte) in text.bytes().enumerate() {
                    state = self.dfa.next_state(state, byte);
                    if self.dfa.is_match_state(state) {
                        match_ = Some((state, i));
                    } else if self.dfa.is_dead_state(state) {
                        break 'search;
                    }
                }
                state = self.dfa.next_eoi_state(state);
                if self.dfa.is_match_state(state) {
                    match_ = Some((state, text.len()));
                }
            }

            let (match_state, longest_match) = match match_ {
                Some(match_) => match_,
                None => {
                    return Some(Err(ParseError::InvalidToken {
                        location: start_offset,
                    }))
                }
            };
            let index = (0..self.dfa.match_len(match_state))
                .map(|n| self.dfa.match_pattern(match_state, n).as_usize())
                .max()
                .unwrap();

            let result = &text[..longest_match];
            let remaining = &text[longest_match..];
            let end_offset = start_offset + longest_match;
            self.text = remaining;
            self.consumed = end_offset;

            if self.skip_vec[index] {
                continue;
            }

            return Some(Ok((start_offset, Token(index, result), end_offset)));
        }
    }
}
