#![allow(dead_code)] // still working on this stuff!

use collections::Set;
use lr1::build;
use lr1::core::*;
use lr1::lookahead::{Lookahead, Nil};
use grammar::repr::*;

mod lane;
mod table;

#[cfg(test)]
mod test;

pub fn build_lane_table_states<'grammar>(grammar: &'grammar Grammar,
                                         start: NonterminalString)
                                         -> LR1Result<'grammar> {
    let (lr0_states, lr0_conflicts) = match build::build_lr0_states(grammar, start) {
        Ok(s) => (s, vec![]),
        Err(e) => (e.states, e.conflicts),
    };

    unimplemented!()
}

fn conflicting_items<'grammar>(state: &LR0State<'grammar>) -> Set<LR0Item<'grammar>> {
    let conflicts = Nil::conflicts(state);

    let reductions1 = conflicts.iter()
                               .map(|c| Item::lr0(c.production, c.production.symbols.len()));

    let reductions2 = conflicts.iter()
                               .filter_map(|c| {
                                   match c.action {
                                       Action::Reduce(p) => Some(Item::lr0(p, p.symbols.len())),
                                       Action::Shift(..) => None,
                                   }
                               });

    let shifts = conflicts.iter()
                          .filter_map(|c| {
                              match c.action {
                                  Action::Shift(term, _) => Some(term),
                                  Action::Reduce(..) => None,
                              }
                          })
                          .flat_map(|term| {
                              state.items
                                   .vec
                                   .iter()
                                   .filter(move |item| item.can_shift_terminal(term))
                                   .cloned()
                          });

    reductions1.chain(reductions2).chain(shifts).collect()
}
