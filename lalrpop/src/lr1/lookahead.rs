use bit_set::{self, BitSet};
use lr1::tls::Lr1Tls;
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

fn with<OP,RET>(op: OP) -> RET
    where OP: FnOnce(&TerminalSet) -> RET
{
    Lr1Tls::with(op)
}

impl TokenSet {
    pub fn new() -> Self {
        with(|terminals| {
            TokenSet {
                bit_set: BitSet::with_capacity(terminals.all.len() + 1)
            }
        })
    }

    fn eof_bit(&self) -> usize {
        with(|terminals| {
            terminals.all.len()
        })
    }

    fn bit(&self, lookahead: Token) -> usize {
        match lookahead {
            Token::EOF => self.eof_bit(),
            Token::Terminal(t) => with(|terminals| terminals.bits[&t])
        }
    }

    pub fn len(&self) -> usize {
        self.bit_set.len()
    }

    pub fn insert(&mut self, lookahead: Token) -> bool {
        let bit = self.bit(lookahead);
        self.bit_set.insert(bit)
    }

    pub fn insert_eof(&mut self) -> bool {
        let bit = self.eof_bit();
        self.bit_set.insert(bit)
    }

    pub fn insert_set(&mut self, set: &TokenSet) -> bool {
        let len = self.len();
        self.bit_set.union_with(&set.bit_set);
        self.len() != len
    }

    pub fn contains(&self, token: Token) -> bool {
        self.bit_set.contains(self.bit(token))
    }

    pub fn contains_eof(&self) -> bool {
        self.bit_set.contains(self.eof_bit())
    }

    /// If this set contains EOF, removes it from the set and returns
    /// true. Otherwise, returns false.
    pub fn take_eof(&mut self) -> bool {
        let eof_bit = self.eof_bit();
        let contains_eof = self.bit_set.contains(eof_bit);
        self.bit_set.remove(eof_bit);
        contains_eof
    }

    pub fn is_disjoint(&self, other: TokenSet) -> bool {
        self.bit_set.is_disjoint(&other.bit_set)
    }

    pub fn iter<'iter>(&'iter self) -> TokenSetIter<'iter> {
        TokenSetIter {
            bit_set: self.bit_set.iter(),
        }
    }
}

pub struct TokenSetIter<'iter> {
    bit_set: bit_set::Iter<'iter, u32>,
}

impl<'iter> Iterator for TokenSetIter<'iter> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.bit_set.next()
                    .map(|bit| {
                        with(|terminals| {
                            if bit == terminals.all.len() {
                                Token::EOF
                            } else {
                                Token::Terminal(terminals.all[bit])
                            }
                        })
                    })
    }
}

impl<'debug> Debug for TokenSet {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let terminals: Vec<_> = self.iter().collect();
        Debug::fmt(&terminals, fmt)
    }
}
