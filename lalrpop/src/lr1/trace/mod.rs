use lr1::core::*;
use lr1::example::*;
use lr1::first::FirstSets;
use lr1::lookahead::{Lookahead, LookaheadSet};
use lr1::state_graph::StateGraph;
use grammar::repr::*;
use session::Session;
use std::rc::Rc;
use util::{Map, map};

// mod shift;
mod reduce;

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    grammar: &'trace Grammar,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
    shift_cache: reduce::ShiftCache<'grammar>,
    reduce_stack: Vec<(StateIndex, Item<'grammar>)>,
}

impl<'trace, 'grammar> Tracer<'trace, 'grammar> {
    pub fn new(session: &'trace Session,
               grammar: &'grammar Grammar,
               states: &'trace [State<'grammar>])
               -> Self {
        Tracer {
            session: session,
            grammar: grammar,
            states: states,
            first_sets: FirstSets::new(grammar),
            state_graph: StateGraph::new(states),
            shift_cache: reduce::ShiftCache::new(),
            reduce_stack: vec![],
        }
    }
}

pub use self::reduce::BacktraceNode;
