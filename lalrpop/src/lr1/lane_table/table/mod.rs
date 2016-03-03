//! The "Lane Table". In the paper, this is depicted like so:
//!
//! ```
//! +-------+----+-----+----+------------+
//! + State | C1 | ... | Cn | Successors |
//! +-------+----+-----+----+------------+
//! ```
//!
//! where each row summarizes some state that potentially contributes
//! lookahead to the conflict. The columns `Ci` represent each of the
//! conflicts we are trying to disentangle; their values are each
//! `TokenSet` indicating the lookahead contributing by this state.
//! The Successors is a vector of further successors. For simplicity
//! though we store this using maps, at least for now.

use collections::{Map, Multimap, Set};
use grammar::repr::*;
use lr1::core::*;
use lr1::lookahead::*;
use std::default::Default;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConflictIndex {
    index: u32
}

pub struct LaneTable<'grammar> {
    grammar: &'grammar Grammar,
    conflicts: u32,
    lookaheads: Map<(StateIndex, ConflictIndex), TokenSet>,
    successors: Multimap<StateIndex, Set<StateIndex>>,
}

impl<'grammar> LaneTable<'grammar> {
    fn new(grammar: &'grammar Grammar, conflicts: u32) -> LaneTable {
        LaneTable {
            grammar: grammar,
            conflicts: conflicts,
            lookaheads: Map::default(),
            successors: Multimap::default(),
        }
    }

    fn add_lookahead(&mut self,
                     state: StateIndex,
                     conflict: ConflictIndex,
                     tokens: &TokenSet)
    {
        let grammar = self.grammar;
        self.lookaheads.entry((state, conflict))
                       .or_insert_with(|| TokenSet::new(grammar))
                       .insert_set(&tokens);
    }

    fn add_successor(&mut self,
                     state: StateIndex,
                     succ: StateIndex)
    {
        self.successors.push(state, succ);
    }
}
