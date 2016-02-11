use lr1::core::*;
use lr1::first::FirstSets;
use lr1::state_graph::StateGraph;
use grammar::repr::*;
use session::Session;
use util::{Set, set};

mod reduce;
mod shift;
mod trace_graph;

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    grammar: &'trace Grammar,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
    trace_graph: TraceGraph<'grammar>,
    visited_set: Set<(StateIndex, NonterminalString)>,
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
            trace_graph: TraceGraph::new(),
            visited_set: set(),
            shift_cache: reduce::ShiftCache::new(),
            reduce_stack: vec![],
        }
    }
}

pub use self::reduce::BacktraceNode;
pub use self::trace_graph::TraceGraph;
