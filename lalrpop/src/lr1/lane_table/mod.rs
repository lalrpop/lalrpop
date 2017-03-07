use collections::Set;
use lr1::build;
use lr1::core::*;
use lr1::lookahead::{Lookahead, Nil};
use grammar::repr::*;

mod lane;
mod table;

#[cfg(test)]
mod test;

use self::lane::*;
use self::table::*;

pub fn build_lane_table_states<'grammar>(grammar: &'grammar Grammar,
                                         start: NonterminalString)
                                         -> LR1Result<'grammar> {
    let (lr0_states, lr0_conflicts) = match build::build_lr0_states(grammar, start) {
        Ok(s) => (s, vec![]),
        Err(e) => (e.states, e.conflicts),
    };

    // this is mostly just dummy code to ensure that things get used
    // and avoid dead-code warnings
    for conflict in lr0_conflicts {
        let inconsistent_state = &lr0_states[conflict.state.0];
        let conflicting_actions = conflicting_actions(inconsistent_state);
        println!("conflicting_actions={:#?}", conflicting_actions);
        let mut tracer = LaneTracer::new(&grammar, &lr0_states, conflicting_actions.len());
        for (i, &conflicting_action) in conflicting_actions.iter().enumerate() {
            tracer.start_trace(inconsistent_state.index,
                               ConflictIndex::new(i),
                               conflicting_action);
        }
        let _ = tracer.into_table();
    }

    unimplemented!()
}

fn conflicting_actions<'grammar>(state: &LR0State<'grammar>) -> Set<Action<'grammar>> {
    let conflicts = Nil::conflicts(state);
    let reductions = conflicts.iter().map(|c| Action::Reduce(c.production));
    let actions = conflicts.iter().map(|c| c.action);
    reductions.chain(actions).collect()
}
