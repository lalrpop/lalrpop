//! LR(1) state construction algorithm.

use collections::{map, Multimap, Set};
use kernel_set;
use grammar::repr::*;
use lr1::core::*;
use lr1::first;
use lr1::lookahead::*;
use std::rc::Rc;
use tls::Tls;

#[cfg(test)]
mod test;

pub fn build_lr1_states<'grammar>(grammar: &'grammar Grammar,
                                  start: NonterminalString)
                                  -> Result<Vec<LR1State<'grammar>>,
                                            LR1TableConstructionError<'grammar>>
{
    profile! {
        &Tls::session(),
        "LR(1) state construction",
        {
            let mut lr1: LR<'grammar, Token> = LR::new(grammar, start, Token::EOF);
            lr1.set_permit_early_stop(true);
            lr1.build_states()
        }
    }
}

pub fn build_lr0_states<'grammar>(grammar: &'grammar Grammar,
                                  start: NonterminalString)
                                  -> Result<Vec<LR0State<'grammar>>,
                                            LR0TableConstructionError<'grammar>>
{
    let lr1 = LR::new(grammar, start, Nil);
    lr1.build_states()
}

struct LR<'grammar, L: LookaheadBuild> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
    start_nt: NonterminalString,
    start_lookahead: L,
    permit_early_stop: bool,
}

impl<'grammar, L: LookaheadBuild> LR<'grammar, L> {
    fn new(grammar: &'grammar Grammar,
           start_nt: NonterminalString,
           start_lookahead: L)
           -> Self {
        LR {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
            start_nt: start_nt,
            start_lookahead: start_lookahead,
            permit_early_stop: false,
        }
    }

    fn set_permit_early_stop(&mut self, v: bool) {
        self.permit_early_stop = v;
    }

    fn build_states(&self)
                    -> Result<Vec<State<'grammar, L>>,
                              TableConstructionError<'grammar, L>>
    {
        let session = Tls::session();
        let mut kernel_set = kernel_set::KernelSet::new();
        let mut states = vec![];
        let mut errors = 0;

        // create the starting state
        kernel_set.add_state(Kernel::start(self.items(self.start_nt,
                                                      0,
                                                      self.start_lookahead)));

        while let Some(Kernel { items: seed_items }) = kernel_set.next() {
            let items = self.transitive_closure(seed_items);
            let index = StateIndex(states.len());

            if index.0 % 5000 == 0 && index.0 > 0 {
                log!(session, Verbose, "{} states created so far.", index.0);
            }

            let mut this_state = State { index: index, items: items.clone(),
                                         shifts: map(), reductions: map(),
                                         conflicts: vec!(), gotos: map() };

            // group the items that we can transition into by shifting
            // over a term or nonterm
            let transitions: Multimap<Symbol, Vec<Item<'grammar, L>>> =
                items.vec
                     .iter()
                     .filter_map(|item| item.shifted_item())
                     .collect();

            for (symbol, shifted_items) in transitions.into_iter() {
                // Not entirely obvious: if the original set of items
                // is sorted to begin with (and it is), then this new
                // set of shifted items is *also* sorted. This is
                // because it is produced from the old items by simply
                // incrementing the index by 1.
                let next_state = kernel_set.add_state(Kernel::shifted(shifted_items));

                match symbol {
                    Symbol::Terminal(s) => {
                        let prev = this_state.shifts.insert(s, next_state);
                        assert!(prev.is_none()); // cannot have a shift/shift conflict
                    }

                    Symbol::Nonterminal(s) => {
                        let prev = this_state.gotos.insert(s, next_state);
                        assert!(prev.is_none());
                    }
                }
            }

            // finally, consider the reductions
            for item in items.vec.iter().filter(|i| i.can_reduce()) {
                let prev = this_state.reductions.insert(item.lookahead, item.production);
                if let Some(other_production) = prev {
                    log!(session, Verbose, "Encountered conflict in state {}",
                         index.0);
                    this_state.conflicts.push(Conflict {
                        state: index,
                        lookahead: item.lookahead,
                        production: item.production,
                        action: Action::Reduce(other_production),
                    });
                }
            }

            // check for shift-reduce conflicts (reduce-reduce detected above)
            L::find_shift_reduce(self, &mut this_state);

            // track total conflicts thus far
            errors += this_state.conflicts.len();

            // extract a new state
            states.push(this_state);

            if self.permit_early_stop && session.stop_after(errors) {
                log!(session, Verbose,
                     "{} conflicts encountered, stopping.", errors);
                break;
            }
        }

        if states.iter().any(|s| !s.conflicts.is_empty()) {
            Err(TableConstructionError { states: states })
        } else {
            Ok(states)
        }
    }

    fn items(&self,
             id: NonterminalString,
             index: usize,
             lookahead: L)
             -> Vec<Item<'grammar, L>>
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
    fn transitive_closure(&self, mut items: Vec<Item<'grammar, L>>)
                          -> Items<'grammar, L>
    {
        let mut counter = 0;

        let mut set: Set<Item<'grammar, L>> =
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
                    L::epsilon_moves(self, nt, remainder, lookahead)
                })
                .filter(|&item| set.insert(item))
                .collect();

            counter = items.len();
            items.extend(new_items);
        }

        items.sort();
        items.dedup();

        Items { vec: Rc::new(items) }
    }
}

/// Except for the initial state, the kernel sets always contain
/// a set of "seed" items where something has been pushed (that is,
/// index > 0). In other words, items like this:
///
///    A = ...p (*) ...
///
/// where ...p is non-empty. We now have to expand to include any
/// epsilon moves:
///
///    A = ... (*) B ...
///    B = (*) ...        // added by transitive_closure algorithm
///
/// But note that the state is completely identified by its
/// kernel set: the same kernel sets always expand to the
/// same transitive closures, and different kernel sets
/// always expand to different transitive closures. The
/// first point is obvious, but the latter point follows
/// because the transitive closure algorithm only adds
/// items where `index == 0`, and hence it can never add an
/// item found in a kernel set.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Kernel<'grammar, L: LookaheadBuild> {
    items: Vec<Item<'grammar, L>>
}

impl<'grammar, L: LookaheadBuild> Kernel<'grammar, L> {
    pub fn start(items: Vec<Item<'grammar, L>>) -> Kernel<'grammar, L> {
        // In start state, kernel should have only items with `index == 0`.
        debug_assert!(items.iter().all(|item| item.index == 0));
        Kernel { items: items }
    }

    pub fn shifted(items: Vec<Item<'grammar, L>>) -> Kernel<'grammar, L> {
        // Assert that this kernel consists only of shifted items
        // where `index > 0`. This assertion could cost real time to
        // check so only do it in debug mode.
        debug_assert!(items.iter().all(|item| item.index > 0));
        Kernel { items: items }
    }
}

impl<'grammar, L: LookaheadBuild> kernel_set::Kernel for Kernel<'grammar, L> {
    type Index = StateIndex;

    fn index(c: usize) -> StateIndex {
        StateIndex(c)
    }
}


trait LookaheadBuild: Lookahead {
    // Given that there exists an item
    //
    //     X = ... (*) Y ...s [L]
    //
    // where `nt` is `Y`, `remainder` is `...s`, and `lookahead` is
    // `L`, computes the new items resulting from epislon moves (if
    // any). The technique of doing this will depend on the amount of
    // lookahead.
    //
    // For example, if we have an LR0 item, then for each `Y = ...`
    // production, we just add an `Y = (*) ...` item. But for LR1
    // items, we have to add multiple items where we consider the
    // lookahead from `FIRST(...s, L)`.
    fn epsilon_moves<'grammar>(lr: &LR<'grammar, Self>,
                               nt: NonterminalString,
                               remainder: &[Symbol],
                               lookahead: Self)
                               -> Vec<Item<'grammar, Self>>;


    fn find_shift_reduce<'grammar>(lr: &LR<'grammar, Self>,
                                   this_state: &mut State<'grammar, Self>);
}

impl LookaheadBuild for Nil {
    fn epsilon_moves<'grammar>(lr: &LR<'grammar, Self>,
                               nt: NonterminalString,
                               remainder: &[Symbol],
                               lookahead: Nil)
                               -> Vec<LR0Item<'grammar>>
    {
        lr.items(nt, 0, lookahead)
    }

    fn find_shift_reduce<'grammar>(_lr: &LR<'grammar, Self>,
                                   this_state: &mut State<'grammar, Self>)
    {
        let index = this_state.index;
        for (&terminal, &next_state) in &this_state.shifts {
            this_state.conflicts.extend(
                this_state.reductions
                          .values()
                          .map(|production| Conflict {
                              state: index,
                              lookahead: Nil,
                              production: production,
                              action: Action::Shift(next_state),
                          }));
        }
    }
}

impl LookaheadBuild for Token {
    fn epsilon_moves<'grammar>(lr: &LR<'grammar, Self>,
                               nt: NonterminalString,
                               remainder: &[Symbol],
                               lookahead: Self)
                               -> Vec<LR1Item<'grammar>>
    {
        let first_set = lr.first_sets.first1(lr.grammar, remainder, lookahead);
        first_set.iter(lr.grammar)
                 .flat_map(|l| lr.items(nt, 0, l))
                 .collect::<Vec<_>>()
    }

    fn find_shift_reduce<'grammar>(_lr: &LR<'grammar, Self>,
                                   this_state: &mut State<'grammar, Self>)
    {
        for (&terminal, &next_state) in &this_state.shifts {
            let token = Token::Terminal(terminal);
            if let Some(&production) = this_state.reductions.get(&token) {
                this_state.conflicts.push(Conflict {
                    state: this_state.index,
                    lookahead: token,
                    production: production,
                    action: Action::Shift(next_state),
                });
            }
        }
    }
}
