//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use std::collections::{HashMap};

mod first;

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    states: Vec<State<'grammar>>,
    first_sets: first::FirstSets,
}

struct State<'grammar> {
    items: Vec<Configuration<'grammar>>,
    shifts: HashMap<TerminalString, StateIndex>,
    gotos: HashMap<NonterminalString, StateIndex>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StateIndex(usize);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Lookahead {
    EOF,
    Terminal(TerminalString),
}

struct Configuration<'grammar> {
    production: &'grammar Production,
    index: usize, // the dot comes before `index`, so `index` would be 1 for X = A (*) B C
    lookahead: Lookahead,
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> LR1 {
        LR1 {
            grammar: grammar,
            states: vec![],
            first_sets: first::FirstSets::new(grammar),
        }
    }

}

