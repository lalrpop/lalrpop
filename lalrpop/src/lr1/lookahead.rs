use bit_set::{self, BitSet};
use std::fmt::{Debug, Formatter, Error};
use grammar::repr::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lookahead {
    EOF,
    Terminal(TerminalString),
}

impl Lookahead {
    pub fn unwrap_terminal(self) -> TerminalString {
        match self {
            Lookahead::Terminal(t) => t,
            Lookahead::EOF => panic!("`unwrap_terminal()` invoked but with EOF"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LookaheadSet {
    bit_set: BitSet<u32>
}

impl LookaheadSet {
    pub fn new(grammar: &Grammar) -> Self {
        LookaheadSet {
            bit_set: BitSet::with_capacity(grammar.all_terminals.len() + 1)
        }
    }

    fn bit(&self, grammar: &Grammar, lookahead: Lookahead) -> usize {
        match lookahead {
            Lookahead::EOF => grammar.all_terminals.len(),
            Lookahead::Terminal(t) => grammar.terminal_bits[&t],
        }
    }

    pub fn len(&self) -> usize {
        self.bit_set.len()
    }

    pub fn insert(&mut self, grammar: &Grammar, lookahead: Lookahead) -> bool {
        let bit = self.bit(grammar, lookahead);
        self.bit_set.insert(bit)
    }

    pub fn insert_set(&mut self, set: &LookaheadSet) -> bool {
        let len = self.len();
        self.bit_set.union_with(&set.bit_set);
        self.len() != len
    }

    pub fn contains(&self, grammar: &Grammar, lookahead: Lookahead) -> bool {
        self.bit_set.contains(self.bit(grammar, lookahead))
    }

    pub fn iter<'iter>(&'iter self, grammar: &'iter Grammar)
                       -> LookaheadSetIter<'iter> {
        LookaheadSetIter {
            bit_set: self.bit_set.iter(),
            grammar: grammar,
        }
    }

    #[allow(dead_code)]
    pub fn debug<'debug>(&'debug self, grammar: &'debug Grammar)
                       -> LookaheadSetDebug<'debug> {
        LookaheadSetDebug {
            set: self,
            grammar: grammar,
        }
    }
}

pub struct LookaheadSetIter<'iter> {
    bit_set: bit_set::Iter<'iter, u32>,
    grammar: &'iter Grammar,
}

impl<'iter> Iterator for LookaheadSetIter<'iter> {
    type Item = Lookahead;

    fn next(&mut self) -> Option<Lookahead> {
        self.bit_set.next()
                    .map(|bit| {
                        if bit == self.grammar.all_terminals.len() {
                            Lookahead::EOF
                        } else {
                            Lookahead::Terminal(self.grammar.all_terminals[bit])
                        }
                    })
    }
}

#[allow(dead_code)]
pub struct LookaheadSetDebug<'debug> {
    set: &'debug LookaheadSet,
    grammar: &'debug Grammar
}

impl<'debug> Debug for LookaheadSetDebug<'debug> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let terminals: Vec<_> = self.set.iter(self.grammar).collect();
        Debug::fmt(&terminals, fmt)
    }
}
