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
    state_sets: &'m mut Map<StateIndex, StateSet>,
    visited: Set<StateIndex>,
    clones: Map<StateIndex, StateIndex>,
}

impl<'m, 'grammar> Merge<'m, 'grammar> {
    pub fn new(table: &'m LaneTable<'grammar>,
               unify: &'m mut UnificationTable<StateSet>,
               states: &'m mut Vec<LR1State<'grammar>>,
               state_sets: &'m mut Map<StateIndex, StateSet>)
               -> Self {
        Merge {
            table,
            unify,
            states,
            state_sets,
            visited: Set::new(),
            clones: Map::new(),
        }
    }

    pub fn start(&mut self, beachhead_state: StateIndex) -> Result<(), (StateIndex, StateIndex)> {
        // Since we always start walks from beachhead states, and they
        // are not reachable from anyone else, this state should not
        // have been unioned with anything else yet.
        self.walk(beachhead_state)
    }

    /// If `state` is a cloned state, find its original index.  Useful
    /// for indexing into the lane table and so forth.
    fn original_index(&self, state: StateIndex) -> StateIndex {
        *self.clones.get(&state).unwrap_or(&state)
    }

    fn successors(&self, state: StateIndex) -> Option<&'m Set<StateIndex>> {
        self.table.successors(self.original_index(state))
    }

    fn walk(&mut self, state: StateIndex) -> Result<(), (StateIndex, StateIndex)> {
        if !self.visited.insert(state) {
            return Ok(());
        }

        for &successor in self.successors(state).iter().flat_map(|&s| s) {
            if self.union(state, successor) {
                self.walk(successor)?;
            } else {
                let successor1 = self.clone(successor);
                if self.union(state, successor1) {
                    self.walk(successor1)?;
                } else {
                    return Err((self.original_index(state),
                                self.original_index(successor1)));
                }
            }
        }

        Ok(())
    }

    fn union(&mut self, source: StateIndex, target: StateIndex) -> bool {
        let set1 = self.state_sets[&source];
        let set2 = self.state_sets[&target];
        self.unify.unify_var_var(set1, set2).is_ok()
    }

    fn clone(&mut self, state: StateIndex) -> StateIndex {
        // create a new state with same contents as the old one
        let new_index = StateIndex(self.states.len());
        let new_state = self.states[state.0].clone();
        self.states.push(new_state);

        // track the original index
        let original_index = self.original_index(state);
        self.clones.insert(new_index, original_index);

        // create a new unify key for this new state
        let context_set = self.table.context_set(original_index).unwrap();
        let state_set = self.unify.new_key(context_set);
        self.state_sets.insert(new_index, state_set);

        new_index
    }
}
