//! Mega naive LALR(1) generation algorithm.

use collections::{map, Map};
use itertools::Itertools;
use lr1::build;
use lr1::core::*;
use lr1::lookahead::*;
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
    pub index: StateIndex,
    pub items: Vec<LR1Item<'grammar>>,
    pub shifts: Map<TerminalString, StateIndex>,
    pub reductions: Map<Token, &'grammar Production>,
    pub conflicts: Vec<LR1Conflict<'grammar>>,
    pub gotos: Map<NonterminalString, StateIndex>,
}

pub fn build_lalr_states<'grammar>(grammar: &'grammar Grammar,
                                   start: NonterminalString)
                                   -> Result<Vec<LR1State<'grammar>>,
                                             LR1TableConstructionError<'grammar>>
{
    // First build the LR(1) states
    let lr_states = try!(build::build_lr1_states(grammar, start));

    profile! {
        &Tls::session(),
        "LALR(1) state collapse",
        collapse_to_lalr_states(&lr_states)
    }
}

pub fn collapse_to_lalr_states<'grammar>(lr_states: &[LR1State<'grammar>])
                                         -> Result<Vec<LR1State<'grammar>>,
                                                   LR1TableConstructionError<'grammar>>
{
    // Now compress them. This vector stores, for each state, the
    // LALR(1) state to which we will remap it.
    let mut remap: Vec<_> = (0..lr_states.len()).map(|_| StateIndex(0)).collect();
    let mut lalr1_map: Map<Vec<LR0Item>, StateIndex> = map();
    let mut lalr1_states: Vec<LALR1State> = vec![];

    for (lr1_index, lr1_state) in lr_states.iter().enumerate() {
        let lr0_kernel: Vec<_> =
            lr1_state.items.vec.iter()
                               .map(|item| item.to_lr0())
                               .dedup()
                               .collect();

        let lalr1_index =
            *lalr1_map.entry(lr0_kernel)
                      .or_insert_with(|| {
                          let index = StateIndex(lalr1_states.len());
                          lalr1_states.push(LALR1State {
                              index: index,
                              items: vec![],
                              shifts: map(),
                              reductions: map(),
                              gotos: map(),
                              conflicts: lr1_state.conflicts.clone()
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

        for (&terminal, &lr1_state) in &lr1_state.shifts {
            let target_state = remap[lr1_state.0];
            let prev = lalr1_state.shifts.insert(terminal, target_state);

            // LALR(1) should not introduce shift/reduce
            assert!(prev.unwrap_or(target_state) == target_state);
        }

        for (&token, &production) in &lr1_state.reductions {
            let prev = lalr1_state.reductions.insert(token, production);
            if let Some(prev_production) = prev {
                if prev_production != production {
                    lalr1_state.conflicts.push(Conflict {
                        state: lalr1_index,
                        lookahead: token,
                        production: production,
                        action: Action::Reduce(prev_production),
                    });
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
                        shifts: lr.shifts,
                        reductions: lr.reductions,
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
