//! Generate valid parse trees.

use grammar::repr::*;
use rand::{self, Rng};
use std::iter::Iterator;

#[derive(PartialEq, Eq)]
pub enum ParseTree {
    Nonterminal(NonterminalString, Vec<ParseTree>),
    Terminal(TerminalString),
}

pub fn random_parse_tree(grammar: &Grammar, symbol: NonterminalString) -> ParseTree {
    let mut gen = Generator { grammar: grammar, rng: rand::thread_rng() };
    gen.nonterminal(symbol)
}

struct Generator<'grammar> {
    grammar: &'grammar Grammar,
    rng: rand::ThreadRng,
}

impl<'grammar> Generator<'grammar> {
    fn nonterminal(&mut self, nt: NonterminalString) -> ParseTree {
        let productions = &self.grammar.productions[&nt];
        let index: usize = self.rng.gen_range(0, productions.len());
        let production = &productions[index];
        let trees: Vec<_> = production.symbols.iter()
                                              .map(|&sym| self.symbol(sym))
                                              .collect();
        ParseTree::Nonterminal(nt, trees)
    }

    fn symbol(&mut self, symbol: Symbol) -> ParseTree {
        match symbol {
            Symbol::Nonterminal(nt) => self.nonterminal(nt),
            Symbol::Terminal(t) => ParseTree::Terminal(t),
        }
    }
}

impl ParseTree {
    pub fn terminals(&self) -> Vec<TerminalString> {
        let mut vec = vec![];
        self.push_terminals(&mut vec);
        vec
    }

    fn push_terminals(&self, vec: &mut Vec<TerminalString>) {
        match *self {
            ParseTree::Terminal(s) => vec.push(s),
            ParseTree::Nonterminal(_, ref trees) => {
                for tree in trees {
                    tree.push_terminals(vec);
                }
            }
        }
    }
}

