use lr1::core::*;
use lr1::example::*;
use lr1::first::FirstSets;
use lr1::lookahead::{Lookahead, LookaheadSet};
use lr1::state_graph::StateGraph;
use grammar::repr::*;
use session::Session;
use std::rc::Rc;
use util::{Map, map};

/// A backtrace explaining how a particular shift:
///
///    X = ...p (*) Token ...
///
/// came to be in the list of items for some state S. This backtrace
/// always has a particular form. First, we can walk back over the
/// prefix, which will bring us to some set of states S1 all of which
/// contain the same item, but with the cursor at the front:
///
///    X = (*) ...p Token ...
///
/// Then we can walk back within those states some number of epsilon
/// moves, traversing nonterminals of the form:
///
///    Y = (*) X ...s
///
/// (Note that each nonterminal `Y` may potentially have many
/// productions of this form. I am not sure yet if they all matter or
/// not.)
///
/// Finally, either we are in the start state, or else we reach some
/// production of the form:
///
///    Z = ...p (*) Y ...s
///
/// Ultimately this "trace" is best represented as a DAG. The problem
/// is that some of those nonterminals could, for example, be
/// optional.
pub struct ShiftTrace<'grammar> {
    contents: ShiftTraceContents<'grammar>,
    parents: Vec<Rc<ShiftTrace<'grammar>>>,
}

enum ShiftTraceContents<'grammar> {
    Item(LR0Item<'grammar>),
    Nonterminal(NonterminalString),
}

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    grammar: &'trace Grammar,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
}

impl<'trace, 'grammar> Tracer<'trace, 'grammar> {
    pub fn backtrace_shift(&mut self,
                           item_state: StateIndex,
                           item: LR0Item<'grammar>)
                           -> BacktraceNode<'grammar> {
        log!(self.session, Debug, "backtrace_shift(item_state={:?}, item={:?})");

        let mut head_node = ShiftTrace {
            contents: ShiftTraceContents::Item(item),
            parents: vec![],
        };

        // Find the predecessor states which contain:
        //
        //     X = (*) ...p Token ...
        //
        // and then iterate through their items to uncover
        // items of the kind:
        //
        //    Y = ...p (*) X ...s
        //
        // If `...p` is empty, these are intermediate nonterminals;
        // otherwise, these are terminating states.
        let pred_states = self.state_graph.trace_back(item_state, item.prefix());
        
    }

    fn backtrace_epsilon(&mut self,
                         item_state: StateIndex,
                         nonterminal: NonterminalString)
                         -> Arc<ShiftTrace<'grammar>>
    {
    }
}
