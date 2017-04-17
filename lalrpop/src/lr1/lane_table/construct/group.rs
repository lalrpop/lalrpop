use lr1::lane_table::table::context_set::{ContextSet, OverlappingLookahead};
use lr1::core::*;
use std::cmp::max;
use ena::unify::{UnifyKey, UnifyValue, UnificationTable};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    index: usize
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupUnifyKey {
    index : u32
}

impl UnifyKey for GroupUnifyKey {
    type Value = Group;

    fn index(&self) -> u32 {
        self.index
    }

    fn from_index(u: u32) -> Self {
        GroupUnifyKey { index: u }
    }

    fn tag() -> &'static str {
        "GroupUnifyKey"
    }
}

impl UnifyValue for Group {
    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, (Self, Self)> {
        // correctness of merging algorithm relies on order used here
        Ok(max(*value1, *value2))
    }
}

pub struct Groups<'m> {
    // groups maps state index into group
    groups:             Vec<Option<Group>>,
    ///! maps group into unification key
    context_sets:       Vec<ContextSet>,
    unification_table:  &'m mut UnificationTable<GroupUnifyKey>,
    ///! maps group into unification key
    unification_keys:   Vec<GroupUnifyKey>,
}

impl<'m> Groups<'m> {
    pub fn new<'a>(number_of_states : usize, unify: &'m mut UnificationTable<GroupUnifyKey>) -> Self {
        let groups = vec![None; number_of_states];
        Groups {
            groups:             groups,
            context_sets:       Vec::new(),
            unification_keys:   Vec::new(),
            unification_table:  unify,
        }
    }

    pub fn allocate(&mut self, state: StateIndex, context_set : ContextSet) -> Group {
        let group = Group { index : self.context_sets.len() };
        self.context_sets.push(context_set);
        self.groups[state.0] = Some(group);
        self.unification_keys.push(self.unification_table.new_key(group));
        group
    }

    pub fn has_group(&self, state : StateIndex) -> bool {
        self.groups[state.0].is_some()
    }

    pub fn merge_state(&mut self, group: Group, state : StateIndex, context_set : &ContextSet)
        -> Result<(), OverlappingLookahead>
    {
        self.groups[state.0] = Some(group);
        self.context_set(group).inplace_union(context_set)
    }

    fn unify_key(&self, group: Group) -> GroupUnifyKey {
        self.unification_keys[group.index]
    }

    fn context_set(&mut self, group: Group) -> &mut ContextSet {
        let key = self.unify_key(group);
        &mut self.context_sets[self.unification_table.probe_value(key).index]
    }

    pub fn merge_groups(&mut self, group1: Group, group2: Group) -> Result<(), OverlappingLookahead>{
        let key1 = self.unify_key(group1);
        let key2 = self.unify_key(group2);
        self.unification_table.unify_var_var(key1, key2).is_ok();

        let context_set = {
            let context_set1 = &self.context_sets[group1.index];
            let context_set2 = &self.context_sets[group2.index];
            ContextSet::union(context_set1, context_set2)?
        };

        self.context_sets[max(group1, group2).index] = context_set;
        Ok(())
    }

}