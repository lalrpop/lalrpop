//!

use collections::{Map, Set};
use grammar::repr::*;
use lr1::build;
use lr1::core::*;
use lr1::lookahead::TokenSet;
use lr1::lane_table::lane::LaneTracer;
use lr1::lane_table::table::{ConflictIndex, LaneTable};
use std::rc::Rc;

pub struct LaneTableConstruct<'grammar> {
    grammar: &'grammar Grammar,
    start: NonterminalString,
}

impl<'grammar> LaneTableConstruct<'grammar> {
    pub fn new(grammar: &'grammar Grammar, start: NonterminalString) -> Self {
        Self { grammar, start }
    }

    pub fn construct(self) -> Result<Vec<LR1State<'grammar>>, LR1TableConstructionError<'grammar>> {
        let TableConstructionError { states, conflicts } = {
            match build::build_lr0_states(self.grammar, self.start) {
                // This is the easy (and very rare...) case.
                Ok(lr0) => return Ok(self.promote_lr0_states(lr0)),
                Err(err) => err,
            }
        };

        // Convert the LR(0) states into LR(0-1) states.
        let mut states = self.promote_lr0_states(states);

        // For each inconsistent state, apply the lane-table algorithm to
        // resolve it.
        let conflicting_states: Set<StateIndex> = conflicts.iter().map(|c| c.state).collect();
        for state_index in conflicting_states {
            self.resolve_inconsistencies(&mut states, state_index)?;
        }

        Ok(states)
    }

    /// Given a set of LR0 states, returns LR1 states where the lookahead
    /// is always `TokenSet::all()`. We refer to these states as LR(0-1)
    /// states in the README.
    fn promote_lr0_states(&self, lr0: Vec<LR0State<'grammar>>) -> Vec<LR1State<'grammar>> {
        let all = TokenSet::all();
        lr0.into_iter()
            .map(|s| {
                let items = s.items
                    .vec
                    .iter()
                    .map(|item| {
                        Item {
                            production: item.production,
                            index: item.index,
                            lookahead: all.clone(),
                        }
                    })
                    .collect();
                let reductions = s.reductions
                    .into_iter()
                    .map(|(_, p)| (all.clone(), p))
                    .collect();
                State {
                    index: s.index,
                    items: Items { vec: Rc::new(items) },
                    shifts: s.shifts,
                    reductions: reductions,
                    gotos: s.gotos,
                }
            })
            .collect()
    }

    fn resolve_inconsistencies(&self,
                               states: &mut Vec<LR1State<'grammar>>,
                               inconsistent_state: StateIndex)
                               -> Result<(), LR1TableConstructionError<'grammar>> {
        let actions = super::conflicting_actions(&states[inconsistent_state.0]);
        let table = self.build_lane_table(states, inconsistent_state, &actions);

        // Consider first the "LALR" case, where the lookaheads for each
        // action are completely disjoint.
        if self.attempt_lalr(&mut states[inconsistent_state.0], &table, &actions) {
            return Ok(());
        }

        panic!("lane table selected where not LALR not yet implemented")
    }

    fn attempt_lalr(&self,
                    state: &mut LR1State<'grammar>,
                    table: &LaneTable<'grammar>,
                    actions: &Set<Action<'grammar>>)
                    -> bool {
        let columns = table.columns();

        debug!("attempt_lalr, columns={:#?}", columns);

        // check whether the lookaheads for all columns are mutually disjoint
        for i in 0..columns.len() {
            for j in 0..columns.len() {
                if i != j {
                    if !columns[i].is_disjoint(&columns[j]) {
                        debug!("column {} not disjoint from {}", i, j);
                        return false;
                    }
                }
            }
        }

        // create a map from each action to its lookahead
        let lookaheads: Map<Action<'grammar>, &TokenSet> = actions.iter()
            .enumerate()
            .map(|(index, &action)| (action, &columns[index]))
            .collect();

        for &mut (ref mut lookahead, production) in &mut state.reductions {
            let action = Action::Reduce(production);
            *lookahead = lookaheads[&action].clone();
        }

        true
    }

    fn build_lane_table(&self,
                        states: &[LR1State<'grammar>],
                        inconsistent_state: StateIndex,
                        actions: &Set<Action<'grammar>>)
                        -> LaneTable<'grammar> {
        let mut tracer = LaneTracer::new(self.grammar, states, actions.len());
        for (i, &action) in actions.iter().enumerate() {
            tracer.start_trace(inconsistent_state, ConflictIndex::new(i), action);
        }
        tracer.into_table()
    }
}
