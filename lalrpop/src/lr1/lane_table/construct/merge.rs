use super::*;

/// The "merge" phase of the algorithm is described in "Step 3c" of
/// [the README][r].  It consists of walking through the various
/// states in the lane table and merging them into sets of states that
/// have compatible context sets; if we encounter a state S that has a
/// successor T but where the context set of S is not compatible with
/// T, then we will clone T into a new T2 (and hopefully the context
/// set of S will be compatible with the reduced context of T2).
///
/// [r]: ../README.md
pub struct Merge<'m, 'grammar: 'm> {
    table: &'m LaneTable<'grammar>,
    unify: &'m mut UnificationTable<StateSet>,
    states: &'m mut Vec<LR1State<'grammar>>,
}

impl<'m, 'grammar> Merge<'m, 'grammar> {
    pub fn new(
        table: &'m LaneTable<'grammar>,
        unify: &'m mut UnificationTable<StateSet>,
        states: &'m mut Vec<LR1State<'grammar>>,
    ) -> Self {
        Merge { table, unify, states }
    }

    pub fn start(&mut self, beachhead_state: StateIndex)
                 -> Result<(), StateIndex> {
        panic!("unimplemented")
    }
}

