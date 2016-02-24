//! Mega naive LALR(1) generation algorithm.

use collections::{map, Map};
use itertools::Itertools;
use lr1::build;
use lr1::core::*;
use lr1::core::Action::{Reduce, Shift};
use lr1::lookahead::Lookahead;
use grammar::repr::*;
use std::rc::Rc;
use tls::Tls;
use util::map::Entry;

#[cfg(test)]
mod test;

// Intermediate LALR(1) state. Identical to an LR(1) state, but that
// the items can be pushed to. We initially create these with an empty
// set of actions, as well.
struct LALR1State<'grammar> {
    index: StateIndex,
    items: Vec<Item<'grammar>>,
    tokens: Map<Lookahead, Action<'grammar>>,
    gotos: Map<NonterminalString, StateIndex>,
    conflicts: Map<Lookahead, Vec<Conflict<'grammar>>>,
}

pub fn build_lalr_states<'grammar>(grammar: &'grammar Grammar,
                                   start: NonterminalString)
                                   -> Result<Vec<State<'grammar>>,
                                             TableConstructionError<'grammar>>
{
    // First build the LR(1) states
    let lr_states = try!(build::build_lr1_states(grammar, start));

    profile! {
        &Tls::session(),
        "LALR(1) state collapse",
        collapse_to_lalr_states(&lr_states)
    }
}

pub fn collapse_to_lalr_states<'grammar>(lr_states: &[State<'grammar>])
                                         -> Result<Vec<State<'grammar>>,
                                                   TableConstructionError<'grammar>>
{
    // Now compress them. This vector stores, for each state, the
    // LALR(1) state to which we will remap it.
    let mut remap: Vec<_> = (0..lr_states.len()).map(|_| StateIndex(0)).collect();
    let mut lalr1_map: Map<Vec<LR0Item>, StateIndex> = map();
    let mut lalr1_states: Vec<LALR1State> = vec![];

    for (lr1_index, lr1_state) in lr_states.iter().enumerate() {
        let lr0_kernel: Vec<_> =
            lr1_state.items.vec.iter()
                               .map(|item| LR0Item {
                                   production: item.production,
                                   index: item.index,
                               })
                               .dedup()
                               .collect();

        let lalr1_index =
            *lalr1_map.entry(lr0_kernel)
                      .or_insert_with(|| {
                          let index = StateIndex(lalr1_states.len());
                          lalr1_states.push(LALR1State {
                              index: index,
                              items: vec![],
                              tokens: map(),
                              gotos: map(),
                              conflicts: map(),
                          });
                          index
                      });

        lalr1_states[lalr1_index.0].items.extend(
            lr1_state.items.vec.iter().cloned());

        remap[lr1_index] = lalr1_index;
    }

    // Now that items are fully built, create the actions
    for (lr1_index, lr1_state) in lr_states.iter().enumerate() {
        let lalr1_index = remap[lr1_index];
        let lalr1_state = &mut lalr1_states[lalr1_index.0];

        for (&lookahead, &lr1_action) in &lr1_state.tokens {
            let lalr1_action = match lr1_action {
                Action::Shift(state) => Action::Shift(remap[state.0]),
                Action::Reduce(prod) => Action::Reduce(prod),
            };

            match lalr1_state.tokens.entry(lookahead) {
                Entry::Occupied(slot) => {
                    let old_action = *slot.get();
                    if old_action != lalr1_action {
                        lalr1_state.conflicts
                                   .entry(lookahead)
                                   .or_insert(vec![])
                                   .push(conflict(lalr1_index, old_action, lalr1_action));
                    }
                }
                Entry::Vacant(slot) => {
                    slot.insert(lalr1_action);
                }
            }
        }

        for (&nt, &lr1_dest) in &lr1_state.gotos {
            let lalr1_dest = remap[lr1_dest.0];

            match lalr1_state.gotos.entry(nt) {
                Entry::Occupied(slot) => {
                    let old_dest = *slot.get();
                    assert_eq!(old_dest, lalr1_dest);
                }
                Entry::Vacant(slot) => {
                    slot.insert(lalr1_dest);
                }
            }
        }
    }

    // Finally, create the new states
    let lr1_states: Vec<_> =
        lalr1_states.into_iter()
                    .map(|lr| State {
                        index: lr.index,
                        items: Items { vec: Rc::new(lr.items) },
                        tokens: lr.tokens,
                        gotos: lr.gotos,
                        conflicts: lr.conflicts,
                    })
                    .collect();

    if lr1_states.iter().any(|s| !s.conflicts.is_empty()) {
        Err(TableConstructionError { states: lr1_states })
    } else {
        Ok(lr1_states)
    }
}

fn conflict<'grammar>(index: StateIndex,
                      action1: Action<'grammar>,
                      action2: Action<'grammar>)
                      -> Conflict<'grammar> {
    let (production, conflict) = match (action1, action2) {
        (c @ Shift(_), Reduce(p)) |
        (Reduce(p), c @ Shift(_)) |
        (Reduce(p), c @ Reduce(_)) => { (p, c) }
        _ => panic!("conflict between {:?} and {:?}", action1, action2)
    };

    Conflict {
        state: index,
        production: production,
        action: conflict,
    }
}
