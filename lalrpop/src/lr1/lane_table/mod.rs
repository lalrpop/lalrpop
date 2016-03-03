use collections::{map, Map};
use itertools::Itertools;
use lr1::build;
use lr1::core::*;
use lr1::lookahead::*;
use grammar::repr::*;
use std::rc::Rc;
use tls::Tls;
use util::map::Entry;

mod table;

pub fn build_lane_table_states<'grammar>
    (grammar: &'grammar Grammar,
     start: NonterminalString)
     -> Result<Vec<LR1State<'grammar>>, LR1TableConstructionError<'grammar>> {
    // Just plain assume that there will be some inconsistent states.
    let lr0_states = match build::build_lr0_states(grammar, start) {
        Ok(s) => s,
        Err(e) => e.states,
    };

    unimplemented!()
}
