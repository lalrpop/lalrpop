use super::*;

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

