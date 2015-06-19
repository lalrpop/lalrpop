//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;
use util::{map, Map, Multimap, Set, Prefix};

mod first;

#[cfg(test)] mod test;

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
}

#[derive(Debug)]
struct State<'grammar> {
    items: Items<'grammar>,
    shifts: Vec<(TerminalString, StateIndex)>,
    gotos: Vec<(NonterminalString, StateIndex)>,
}

type Items<'grammar> = Rc<Vec<Item<'grammar>>>;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StateIndex(usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Lookahead {
    EOF,
    Terminal(TerminalString),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Item<'grammar> {
    production: &'grammar Production,
    index: usize, // the dot comes before `index`, so `index` would be 1 for X = A (*) B C
    lookahead: Lookahead,
}

struct StateSet<'grammar> {
    states: Vec<State<'grammar>>,
    state_map: Map<Items<'grammar>, StateIndex>,
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> LR1 {
        LR1 {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
        }
    }

    fn build_states(&self, start_nt: NonterminalString) -> Vec<State<'grammar>> {
        let mut state_set = StateSet::new();

        // create the starting state
        state_set.add_state(
            self.transitive_closure(
                self.items(start_nt, 0, Lookahead::EOF)));

        let mut counter = 0;
        while counter < state_set.states.len() {
            let items = state_set.states[counter].items.clone();

            // group the items that we can transition into by shifting
            // over a term or nonterm
            let transitions: Multimap<Symbol, Item<'grammar>> =
                items.iter()
                     .filter_map(|item| item.shifted_item())
                     .collect();

            for (symbol, items) in transitions.into_iter() {
                let items = self.transitive_closure(items);
                let next_state = state_set.add_state(items);

                // FIXME check for conflicts
                match symbol {
                    Symbol::Terminal(t) => {
                        state_set.states[counter].shifts.push((t, next_state));
                    }
                    Symbol::Nonterminal(t) => {
                        state_set.states[counter].gotos.push((t, next_state));
                    }
                }
            }

            // extract a new state
            counter += 1;
        }

        state_set.states
    }

    fn items(&self,
                      id: NonterminalString,
                      index: usize,
                      lookahead: Lookahead)
                      -> Vec<Item<'grammar>>
    {
        self.grammar.productions_for(id)
                    .iter()
                    .map(|production| {
                        debug_assert!(index <= production.symbols.len());
                        Item { production: production,
                                        index: index,
                                        lookahead: lookahead }
                    })
                    .collect()
    }

    // expands `state` with epsilon moves
    fn transitive_closure(&self, mut items: Vec<Item<'grammar>>)
                          -> Items<'grammar>
    {
        let mut counter = 0;

        let mut set: Set<Item<'grammar>> =
            items.iter().cloned().collect();

        while counter < items.len() {
            let new_items: Vec<_> =
                items[counter..]
                .iter()
                .filter_map(|item| {
                    let shift_symbol = item.shift_symbol();
                    match shift_symbol {
                        None => None, // requires a reduce
                        Some((Symbol::Terminal(_), _)) => None, // requires a shift
                        Some((Symbol::Nonterminal(nt), remainder)) => {
                            Some((nt, remainder, item.lookahead))
                        }
                    }
                })
                .flat_map(|(nt, remainder, lookahead)| {
                    let first_set = self.first_sets.first(remainder, lookahead);
                    first_set.into_iter()
                             .flat_map(move |l| self.items(nt, 0, l))
                })
                .filter(|&item| set.insert(item))
                .collect();

            counter = items.len();
            items.extend(new_items);
        }

        Rc::new(items)
    }
}

impl<'grammar> Item<'grammar> {
    fn can_shift(&self) -> bool {
        self.index < self.production.symbols.len()
    }

    fn can_reduce(&self) -> bool {
        self.index == self.production.symbols.len()
    }

    fn shifted_item(&self) -> Option<(Symbol, Item<'grammar>)> {
        if self.can_shift() {
            Some((self.production.symbols[self.index],
                  Item { production: self.production,
                                  index: self.index + 1,
                                  lookahead: self.lookahead }))
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
            state_map: map(),
        }
    }

    fn add_state(&mut self, items: Items<'grammar>) -> StateIndex {
        let states = &mut self.states;
        *self.state_map.entry(items.clone()).or_insert_with(|| {
            let index = StateIndex(states.len());
            states.push(State { items: items,
                                shifts: Vec::new(),
                                gotos: Vec::new() });
            index
        })
    }
}

impl<'grammar> Debug for Item<'grammar> {
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

impl Debug for StateIndex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "S{}", self.0)
    }
}
