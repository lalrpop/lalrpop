//! LR(1) state construction algorithm.

use collections::{map, Multimap, Set};
use kernel_set;
use grammar::repr::*;
use lr1::core::*;
use lr1::first;
use lr1::lookahead::Lookahead;
use std::rc::Rc;
use tls::Tls;

#[cfg(test)]
mod test;

pub fn build_lr1_states<'grammar>(grammar: &'grammar Grammar,
                                  start: NonterminalString)
                                  -> Result<Vec<LR1State<'grammar>>,
                                            TableConstructionError<'grammar>>
{
    profile! {
        &Tls::session(),
        "LR(1) state construction",
        {
            let lr1 = LR1::new(grammar);
            lr1.build_states(start)
        }
    }
}

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> Self {
        LR1 {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
        }
    }

    fn build_states(&self, start_nt: NonterminalString)
                    -> Result<Vec<LR1State<'grammar>>, TableConstructionError<'grammar>>
    {
        let session = Tls::session();
        let mut kernel_set = kernel_set::KernelSet::new();
        let mut states = vec![];
        let mut errors = 0;

        // create the starting state
        kernel_set.add_state(Kernel::start(self.items(start_nt, 0, Lookahead::EOF)));

        while let Some(Kernel { items: seed_items }) = kernel_set.next() {
            let items = self.transitive_closure(seed_items);
            let index = StateIndex(states.len());

            if index.0 % 5000 == 0 && index.0 > 0 {
                log!(session, Verbose, "{} states created so far.", index.0);
            }

            let mut this_state = State { index: index, items: items.clone(),
                                         tokens: map(), gotos: map(),
                                         conflicts: map() };

            // group the items that we can transition into by shifting
            // over a term or nonterm
            let transitions: Multimap<Symbol, Vec<LR1Item<'grammar>>> =
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
            for item in items.vec.iter().filter(|i| i.can_reduce()) {
                let action = Action::Reduce(item.production);
                let prev = this_state.tokens.insert(item.lookahead, action);
                if let Some(conflict) = prev {
                    log!(session, Verbose, "Encountered conflict in state {}",
                         index.0);
                    this_state.conflicts.entry(item.lookahead)
                                        .or_insert(vec![])
                                        .push(Conflict {
                                            state: index,
                                            production: item.production,
                                            action: conflict,
                                        });
                    errors += 1;
                }
            }

            // extract a new state
            states.push(this_state);

            if session.stop_after(errors) {
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
             lookahead: Lookahead)
             -> Vec<LR1Item<'grammar>>
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
    fn transitive_closure(&self, mut items: Vec<LR1Item<'grammar>>)
                          -> LR1Items<'grammar>
    {
        let mut counter = 0;

        let mut set: Set<LR1Item<'grammar>> =
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
                    let (first_set, _) =
                        self.first_sets.first(self.grammar, remainder, lookahead);
                    first_set.iter(self.grammar)
                             .flat_map(|l| self.items(nt, 0, l))
                             .collect::<Vec<_>>()
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
struct Kernel<'grammar> {
    items: Vec<LR1Item<'grammar>>
}

impl<'grammar> Kernel<'grammar> {
    pub fn start(items: Vec<LR1Item<'grammar>>) -> Kernel<'grammar> {
        // In start state, kernel should have only items with `index == 0`.
        debug_assert!(items.iter().all(|item| item.index == 0));
        Kernel { items: items }
    }

    pub fn shifted(items: Vec<LR1Item<'grammar>>) -> Kernel<'grammar> {
        // Assert that this kernel consists only of shifted items
        // where `index > 0`. This assertion could cost real time to
        // check so only do it in debug mode.
        debug_assert!(items.iter().all(|item| item.index > 0));
        Kernel { items: items }
    }
}

impl<'grammar> kernel_set::Kernel for Kernel<'grammar> {
    type Index = StateIndex;

    fn index(c: usize) -> StateIndex {
        StateIndex(c)
    }
}
