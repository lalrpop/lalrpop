//! Naive LR(1) generation algorithm.

use grammar::repr::*;
use std::fmt::{Debug, Formatter, Error};
use std::rc::Rc;
use util::{map, Map, Multimap, Set, Prefix};

pub mod ascent;
mod error;
mod first;
mod interpret;

#[cfg(test)] mod test;

pub use self::error::report_error;

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
}

#[derive(Debug)]
pub struct State<'grammar> {
    index: StateIndex,
    items: Items<'grammar>,
    tokens: Map<Lookahead, Action<'grammar>>,
    gotos: Map<NonterminalString, StateIndex>,
}

#[derive(Debug)]
enum Action<'grammar> {
    Shift(StateIndex),
    Reduce(&'grammar Production),
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

#[derive(Debug)]
struct StateSet<'grammar> {
    states: Vec<State<'grammar>>,
    state_map: Map<Items<'grammar>, StateIndex>,
}

#[derive(Debug)]
pub struct TableConstructionError<'grammar> {
    // when in this state:
    items: Items<'grammar>,

    // and looking at this token:
    lookahead: Lookahead,

    // we can reduce using this production:
    production: &'grammar Production,

    // but we can also:
    conflict: Action<'grammar>,
}

pub fn build_states<'grammar>(grammar: &'grammar Grammar,
                              start: NonterminalString)
                              -> Result<Vec<State<'grammar>>, TableConstructionError<'grammar>>
{
    let lr1 = LR1::new(grammar);
    lr1.build_states(start)
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> LR1 {
        LR1 {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
        }
    }

    fn build_states(&self, start_nt: NonterminalString)
                    -> Result<Vec<State<'grammar>>, TableConstructionError<'grammar>>
    {
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

                let this_state = &mut state_set.states[counter];
                match symbol {
                    Symbol::Terminal(s) => {
                        let action = Action::Shift(next_state);
                        let prev = this_state.tokens.insert(Lookahead::Terminal(s), action);
                        assert!(prev.is_none()); // cannot have a shift/shift conflict
                    }

                    Symbol::Nonterminal(s) => {
                        let prev = this_state.gotos.insert(s, next_state);
                        assert!(prev.is_none());
                    }
                }
            }

            // finally, consider the reductions
            let this_state = &mut state_set.states[counter];
            for item in items.iter().filter(|i| i.can_reduce()) {
                let action = Action::Reduce(item.production);
                let prev = this_state.tokens.insert(item.lookahead, action);
                if let Some(conflict) = prev {
                    return Err(TableConstructionError {
                        items: items.clone(),
                        lookahead: item.lookahead,
                        production: item.production,
                        conflict: conflict,
                    });
                }
            }

            // extract a new state
            counter += 1;
        }

        Ok(state_set.states)
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
            states.push(State { index: index, items: items, tokens: map(), gotos: map() });
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

impl<'grammar> State<'grammar> {
    fn prefix(&self) -> &'grammar [Symbol] {
        // Each state fn takes as argument the longest prefix of any
        // item. Note that all items must have compatible prefixes.
        let (_, prefix) =
            self.items.iter()
                      .map(|item| &item.production.symbols[..item.index])
                      .map(|symbols| (symbols.len(), symbols))
                      .max() // grr, max_by is unstable :(
                      .unwrap();

        debug_assert!(
            self.items.iter()
                      .all(|item| prefix.ends_with(&item.production.symbols[..item.index])));

        prefix
    }
}

