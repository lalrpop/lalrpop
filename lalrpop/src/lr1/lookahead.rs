use bit_set::{self, BitSet};
use std::fmt::{Debug, Formatter, Error};
use std::hash::Hash;
use grammar::repr::*;

pub trait Lookahead: Copy + Debug + Eq + Ord + Hash {
    fn fmt_as_item_suffix(&self, fmt: &mut Formatter) -> Result<(), Error>;
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nil;

impl Lookahead for Nil {
    fn fmt_as_item_suffix(&self, _fmt: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

/// I have semi-arbitrarily decided to use the term "token" to mean
/// either one of the terminals of our language, or else the
/// pseudo-symbol EOF that represents "end of input".
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,
    Terminal(TerminalString),
}

impl Lookahead for Token {
    fn fmt_as_item_suffix(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, " [{}]", self)
    }
}

impl Token {
    pub fn unwrap_terminal(self) -> TerminalString {
        match self {
            Token::Terminal(t) => t,
            Token::EOF => panic!("`unwrap_terminal()` invoked but with EOF"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenSet {
    bit_set: BitSet<u32>
}

impl TokenSet {
    pub fn new(grammar: &Grammar) -> Self {
        TokenSet {
            bit_set: BitSet::with_capacity(grammar.all_terminals.len() + 1)
        }
    }

    fn eof_bit(&self, grammar: &Grammar) -> usize {
        grammar.all_terminals.len()
    }

    fn bit(&self, grammar: &Grammar, lookahead: Token) -> usize {
        match lookahead {
            Token::EOF => self.eof_bit(grammar),
            Token::Terminal(t) => grammar.terminal_bits[&t],
        }
    }

    pub fn len(&self) -> usize {
        self.bit_set.len()
    }

    pub fn insert(&mut self, grammar: &Grammar, lookahead: Token) -> bool {
        let bit = self.bit(grammar, lookahead);
        self.bit_set.insert(bit)
    }

    pub fn insert_set(&mut self, set: &TokenSet) -> bool {
        let len = self.len();
        self.bit_set.union_with(&set.bit_set);
        self.len() != len
    }

    pub fn contains_eof(&self, grammar: &Grammar) -> bool {
        self.bit_set.contains(self.eof_bit(grammar))
    }

    /// If this set contains EOF, removes it from the set and returns
    /// true. Otherwise, returns false.
    pub fn take_eof(&mut self, grammar: &Grammar) -> bool {
        let eof_bit = self.eof_bit(grammar);
        let contains_eof = self.bit_set.contains(eof_bit);
        self.bit_set.remove(eof_bit);
        contains_eof
    }

    pub fn is_disjoint(&self, other: TokenSet) -> bool {
        self.bit_set.is_disjoint(&other.bit_set)
    }

    pub fn iter<'iter>(&'iter self, grammar: &'iter Grammar)
                       -> TokenSetIter<'iter> {
        TokenSetIter {
            bit_set: self.bit_set.iter(),
            grammar: grammar,
        }
    }

    #[allow(dead_code)]
    pub fn debug<'debug>(&'debug self, grammar: &'debug Grammar)
                       -> TokenSetDebug<'debug> {
        TokenSetDebug {
            set: self,
            grammar: grammar,
        }
    }
}

pub struct TokenSetIter<'iter> {
    bit_set: bit_set::Iter<'iter, u32>,
    grammar: &'iter Grammar,
}

impl<'iter> Iterator for TokenSetIter<'iter> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.bit_set.next()
                    .map(|bit| {
                        if bit == self.grammar.all_terminals.len() {
                            Token::EOF
                        } else {
                            Token::Terminal(self.grammar.all_terminals[bit])
                        }
                    })
    }
}

#[allow(dead_code)]
pub struct TokenSetDebug<'debug> {
    set: &'debug TokenSet,
    grammar: &'debug Grammar
}

impl<'debug> Debug for TokenSetDebug<'debug> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let terminals: Vec<_> = self.set.iter(self.grammar).collect();
        Debug::fmt(&terminals, fmt)
    }
}
