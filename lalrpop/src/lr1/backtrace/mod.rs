use lr1::first::FirstSets;
use lr1::{BacktraceNode, Item, State, StateIndex};
use grammar::repr::*;
use session::Session;
use self::state_graph::StateGraph;

mod state_graph;
mod test;

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
}

impl<'trace, 'grammar> Tracer<'trace, 'grammar> {
    pub fn new(session: &'trace Session,
               grammar: &'grammar Grammar,
               states: &'trace [State<'grammar>])
               -> Self {
        Tracer {
            session: session,
            states: states,
            first_sets: FirstSets::new(grammar),
            state_graph: StateGraph::new(states),
        }
    }

    /// Returns a backtrace explaining how the state `item_state` came
    /// to contain the item `item`:
    ///
    ///    NT = ... (*) ... [L]
    ///
    /// In particular, how we came to be able to reduce `NT` with
    /// lookahead `L`.
    pub fn backtrace(&self, item_state: StateIndex, item: Item<'grammar>)
                     -> BacktraceNode<'grammar> {
        log!(self.session, Debug, "backtrace(item_state={:?} item={:?})", item_state, item);

        let mut result_node = BacktraceNode::new(item);

        // The nonterminal NT and lookahead L we are looking for
        let nt_sym = Symbol::Nonterminal(item.production.nonterminal);
        let lookahead = item.lookahead;

        // We will have arrived at the current state after pushing N
        // symbols, where N is the number of items pushed so far in
        // `item`. So walk backwards N states to find the state(s)
        // where we had something like
        //
        //     A := ... (*) NT ... [L1]
        let pred_states = self.state_graph.predecessors_at_distance(item_state, item.index);
        log!(self.session, Debug, "backtrace: pred_states={:?}", pred_states);

        // For each such predecessor state P...
        for pred_state in pred_states {
            // ...scan the items in P, looking for one like:
            //
            //     A := ... (*) NT ...x [L1]
            //
            // where the lookahead L is in FIRST(...x, L1).
            for item in self.states[pred_state.0].items.vec.iter() {
                log!(self.session, Debug, "backtrace: pred_state {:?} has item {:?}",
                     pred_state, item);
                if let Some((shifted, remainder)) = item.shift_symbol() {
                    if shifted == nt_sym {
                        let (first, maybe_empty) = self.first_sets.first(remainder, item.lookahead);
                        log!(self.session, Debug, "backtrace: first={:?} maybe_empty={:?}",
                             first, maybe_empty);
                        if first.contains(&lookahead) {
                            // Found such a state. Now, continue
                            // tracing back so long as the lookahead
                            // may still have come from the
                            // surrounding context. This can occur if
                            // `...x` may be empty *and* the lookahead
                            // matches (if the lookahead doesn't
                            // match, then the only source for L is
                            // `...x`).
                            if maybe_empty && item.lookahead == lookahead {
                                let parent_node = self.backtrace(pred_state, *item);
                                result_node.parents.push(parent_node);
                            } else {
                                result_node.parents.push(BacktraceNode::new(*item));
                            }
                        }
                    }
                }
            }
        }

        result_node
    }
}

