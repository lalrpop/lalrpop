//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;
use util::Prefix;

mod first;

#[cfg(test)] mod test;

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
}

struct State<'grammar> {
    configurations: Configurations<'grammar>,
    shifts: HashMap<TerminalString, StateIndex>,
    gotos: HashMap<NonterminalString, StateIndex>,
}

type Configurations<'grammar> = Rc<Vec<Configuration<'grammar>>>;

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

struct StateSet<'grammar> {
    states: Vec<State<'grammar>>,
    state_map: HashMap<Configurations<'grammar>, StateIndex>,
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> LR1 {
        LR1 {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
        }
    }

    fn build_states(&mut self, start_nt: NonterminalString) -> Vec<State<'grammar>> {
        let mut state_set = StateSet::new();

        // create the starting state
        state_set.add_state(
            self.transitive_closure(
                self.configurations(start_nt, 0, Lookahead::EOF)));

        let mut counter = 0;
        while counter < state_set.states.len() {
            let configurations = state_set.states[counter].configurations.clone();
            counter += 1;

            // for each configuration where we can shift, do so, and
            // create the transitive closure of the resulting state
            let shifted_configurations =
                configurations
                .iter()
                .filter_map(|configuration| configuration.shifted_configuration())
                .map(|configuration| self.transitive_closure(vec![configuration]));

            // add a state for each of those cases where we did a shift
            for configuration in shifted_configurations {
                state_set.add_state(configuration);
            }

            // extract a new state
            counter += 1;
        }

        state_set.states
    }

    fn configurations(&self,
                      id: NonterminalString,
                      index: usize,
                      lookahead: Lookahead)
                      -> Vec<Configuration<'grammar>>
    {
        self.grammar.productions_for(id)
                    .iter()
                    .map(|production| {
                        debug_assert!(index <= production.symbols.len());
                        Configuration { production: production,
                                        index: index,
                                        lookahead: lookahead }
                    })
                    .collect()
    }

    // expands `state` with epsilon moves
    fn transitive_closure(&self, mut configurations: Vec<Configuration<'grammar>>)
                          -> Configurations<'grammar>
    {
        let mut counter = 0;

        let mut set: HashSet<Configuration<'grammar>> =
            configurations.iter().cloned().collect();

        while counter < configurations.len() {
            let new_configurations: Vec<_> =
                configurations[counter..]
                .iter()
                .filter_map(|configuration| {
                    let shift_symbol = configuration.shift_symbol();
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
                    first_set.into_iter()
                             .flat_map(move |l| self.configurations(nt, 0, l))
                })
                .filter(|&configuration| set.insert(configuration))
                .collect();

            counter = configurations.len();
            configurations.extend(new_configurations);
        }

        Rc::new(configurations)
    }
}

impl<'grammar> Configuration<'grammar> {
    fn can_shift(&self) -> bool {
        self.index < self.production.symbols.len()
    }

    fn can_reduce(&self) -> bool {
        self.index == self.production.symbols.len()
    }

    fn shifted_configuration(&self) -> Option<Configuration<'grammar>> {
        if self.can_shift() {
            Some(Configuration { production: self.production,
                                 index: self.index + 1,
                                 lookahead: self.lookahead })
        } else {
            None
        }
    }

    fn shift_symbol(&self) -> Option<(Symbol, &[Symbol])> {
        if self.can_shift() {
            Some((self.production.symbols[self.index], &self.production.symbols[self.index+1..]))
        } else {
            None
        }
    }
}

impl<'grammar> StateSet<'grammar> {
    fn new() -> StateSet<'grammar> {
        StateSet {
            states: vec![],
            state_map: HashMap::new(),
        }
    }

    fn add_state(&mut self, configurations: Configurations<'grammar>) -> StateIndex {
        let states = &mut self.states;
        *self.state_map.entry(configurations.clone()).or_insert_with(|| {
            let index = StateIndex(states.len());
            states.push(State { configurations: configurations,
                                     shifts: HashMap::new(),
                                     gotos: HashMap::new() });
            index
        })
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
