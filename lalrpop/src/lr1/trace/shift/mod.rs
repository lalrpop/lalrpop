use lr1::core::*;
use grammar::repr::*;

use super::Tracer;
use super::trace_graph::*;

#[cfg(test)] mod test;

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

impl<'trace, 'grammar> Tracer<'trace, 'grammar> {
    pub fn backtrace_shift_graph(mut self,
                                 item_state: StateIndex,
                                 item: LR0Item<'grammar>)
                                 -> TraceGraph<'grammar> {
        log!(self.session, Debug, "backtrace_shift_graph(item_state={:?}, item={:?})",
             item_state, item);

        let prefix = item.prefix();

        // The states `S`
        let pred_states = self.state_graph.trace_back(item_state, prefix);

        // Add the edge `[X] -{...p}-> [X = ...p (*) Token ...]`
        self.trace_graph.add_edge(item.production.nonterminal, item, prefix);

        for pred_state in pred_states {
            self.trace_epsilon_edges(pred_state, item.production.nonterminal);
        }

        self.trace_graph
    }

    // Because item.index is 0, we know we are at an index
    // like:
    //
    //     Y = (*) ...
    //
    // This can only arise if `Y` is the start nonterminal
    // or if there is an epsilon move from another item
    // like:
    //
    //     Z = ...p (*) Y ...
    //
    // So search for items like Z.
    fn trace_epsilon_edges(&mut self,
                           item_state: StateIndex,
                           nonterminal: NonterminalString) // "Y"
    {
        log!(self.session, Debug,
             "trace_epsilon_edges(item_state={:?}, nonterminal={:?})",
             item_state, nonterminal);
        if self.visited_set.insert((item_state, nonterminal)) {
            for &pred_item in self.states[item_state.0].items.vec.iter() {
                if self.can_shift(pred_item, nonterminal) {
                    if pred_item.index > 0 {
                        // Add an edge:
                        //
                        //     [Z = ...p (*) Y ...] -\epsilon-> [Y]
                        self.trace_graph.add_edge(pred_item, nonterminal, &[]);
                    } else {
                        // Trace back any incoming edges to [Z = ...p (*) Y ...].
                        let pred_nonterminal = pred_item.production.nonterminal;
                        self.trace_graph.add_edge(pred_nonterminal, nonterminal, &[]);
                        self.trace_epsilon_edges(item_state, pred_nonterminal);
                    }
                }
            }
        }
    }

    fn can_shift(&self,
                 item: Item<'grammar>,
                 nonterminal: NonterminalString)
                 -> bool
    {
        match item.shift_symbol() {
            Some((Symbol::Nonterminal(shifted), _)) => shifted == nonterminal,
            _ => false,
        }
    }
}
