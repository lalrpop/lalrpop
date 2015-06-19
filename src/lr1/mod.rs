//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, Error};
use util::Prefix;

mod first;

#[cfg(test)] mod test;

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    states: Vec<State<'grammar>>,
    first_sets: first::FirstSets,
}

struct State<'grammar> {
    configurations: Vec<Configuration<'grammar>>,
    shifts: HashMap<TerminalString, StateIndex>,
    gotos: HashMap<NonterminalString, StateIndex>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StateIndex(usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Lookahead {
    EOF,
    Terminal(TerminalString),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
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

    fn build_states(&mut self, start_nt: NonterminalString) {
        debug_assert!(self.states.is_empty());

        let state0 = self.start_state(start_nt, Lookahead::EOF);
        self.states.push(state0);
    }

    fn start_state(&self, id: NonterminalString, lookahead: Lookahead) -> State<'grammar> {
        let configurations =
            self.transitive_closure(
                self.start_configurations(id, lookahead));
        State { configurations: configurations,
                shifts: HashMap::new(),
                gotos: HashMap::new() }
    }

    fn start_configurations(&self,
                            id: NonterminalString,
                            lookahead: Lookahead)
                            -> Vec<Configuration<'grammar>>
    {
        self.grammar.productions_for(id)
                    .iter()
                    .map(|production| {
                        Configuration { production: production,
                                        index: 0,
                                        lookahead: lookahead }
                    })
                    .collect()
    }

    // expands `state` with epsilon moves
    fn transitive_closure(&self, mut configurations: Vec<Configuration<'grammar>>)
                          -> Vec<Configuration<'grammar>>
    {
        println!("expand_configurations({:?})", configurations);

        let mut counter = 0;

        let mut set: HashSet<Configuration<'grammar>> =
            configurations.iter().cloned().collect();

        while counter < configurations.len() {
            println!("expand_configurations: counter={:?}", counter);

            let new_configurations: Vec<_> =
                configurations[counter..]
                .iter()
                .filter_map(|configuration| {
                    let shift_symbol = configuration.shift_symbol();
                    println!("expand_configurations: configuration: {:?} shift_symbol: {:?}",
                             configuration, shift_symbol);
                    match shift_symbol {
                        None => None, // requires a reduce
                        Some((Symbol::Terminal(_), _)) => None, // requires a shift
                        Some((Symbol::Nonterminal(nt), remainder)) => {
                            Some((nt, remainder, configuration.lookahead))
                        }
                    }
                })
                .flat_map(|(nt, remainder, lookahead)| {
                    let first_set = self.first_sets.first(remainder, lookahead);
                    println!("expand_configurations: ({:?}, {:?}, {:?}) first_set={:?}",
                             nt, remainder, lookahead, first_set);
                    first_set.into_iter()
                             .flat_map(move |l| self.start_configurations(nt, l))
                })
                .filter(|&configuration| set.insert(configuration))
                .collect();

            counter = configurations.len();
            configurations.extend(new_configurations);
        }

        configurations
    }
}

impl<'grammar> Configuration<'grammar> {
    fn shift_symbol(&self) -> Option<(Symbol, &[Symbol])> {
        if self.index == self.production.symbols.len() {
            None
        } else {
            Some((self.production.symbols[self.index],
                  &self.production.symbols[self.index+1..]))
        }
    }
}

impl<'grammar> Debug for Configuration<'grammar> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} ={} (*){} [{:?}]",
               self.production.nonterminal,
               Prefix(" ", &self.production.symbols[..self.index]),
               Prefix(" ", &self.production.symbols[self.index..]),
               self.lookahead)
    }
}

impl Debug for Lookahead {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Lookahead::EOF => write!(fmt, "EOF"),
            Lookahead::Terminal(s) => write!(fmt, "{}", s),
        }
    }
}
