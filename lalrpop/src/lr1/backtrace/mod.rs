use lr1::first::FirstSets;
use lr1::{LR0Item, Item, State, StateIndex};
use grammar::repr::*;
use session::Session;

mod example;
mod state_graph;
#[cfg(test)] mod test;

use self::example::ExampleIterator;
use self::state_graph::StateGraph;

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
    stack: Vec<(StateIndex, Item<'grammar>)>,
}

/// Stores a backtrace tree used in error reporting. Consider a simple
/// example where we want the backtrace of EXPR with lookahead `,`,
/// given this grammar:
///
///     START = EXPRS ";"
///           | EXPRS
///     EXPRS = EXPR
///           | EXPRS "," EXPR
///     EXPR = ...
///
/// We would result in a sort of inverted tree like:
///
///     EXPR = ... (*)
///         EXPRS = (*) EXPR
///             EXPRS = (*) EXPRS "," EXPR
///                 START = (*) EXPRS ";"
///         EXPRS = EXPRS "," (*) EXPR
///             START = (*) EXPRS ";"
#[derive(Debug)]
pub struct BacktraceNode<'grammar> {
    item: LR0Item<'grammar>,
    parents: Vec<BacktraceNode<'grammar>>,
}

/// An "example" input and the way it was derived. This can be
/// serialized into useful text. For example, it might represent
/// something like this:
///
/// ```
/// Ty "->" Ty "->" Ty
/// |        |       |
/// +-Ty-----+       |
/// |                |
/// +-Ty-------------+
/// ```
///
/// The top-line is the `symbols` vector. The groupings below are
/// stored in the `reductions` vector, in order from smallest to
/// largest (they are always properly nested).
///
/// The `symbols` vector is actually `Option<Symbol>` to account
/// for empty reductions:
///
/// ```
/// A       B
/// | |   | |
/// | +-Y-+ |
/// +-Z-----+
/// ```
///
/// The "empty space" between A and B would be represented as `None`.
#[derive(Debug)]
pub struct Example {
    pub symbols: Vec<Option<Symbol>>,
    pub reductions: Vec<Reduction>,
}

#[derive(Debug)]
pub struct Reduction {
    pub start: usize,
    pub end: usize,
    pub nonterminal: NonterminalString,
}

impl<'grammar> BacktraceNode<'grammar> {
    fn new(item: Item<'grammar>) -> Self {
        BacktraceNode { item: LR0Item { production: item.production,
                                        index: item.index },
                        parents: vec![] }
    }

    fn merge_parent(&mut self, new_parent: BacktraceNode<'grammar>) {
        for old_parent in &mut self.parents {
            if old_parent.item == new_parent.item {
                for new_grandparent in new_parent.parents {
                    old_parent.merge_parent(new_grandparent);
                }
                return;
            }
        }

        self.parents.push(new_parent);
    }

    pub fn examples<'ex>(&'ex self) -> ExampleIterator<'ex> {
        ExampleIterator::new(self)
    }
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
            stack: vec![],
        }
    }

    /// Returns a backtrace explaining how the state `item_state` came
    /// to contain the item `item`:
    ///
    ///    NT = ... (*) ... [L]
    ///
    /// In particular, how we came to be able to reduce `NT` with
    /// lookahead `L`.
    pub fn backtrace(&mut self, item_state: StateIndex, item: Item<'grammar>)
                     -> BacktraceNode<'grammar> {
        log!(self.session, Debug, "backtrace(item_state={:?} item={:?})", item_state, item);

        let mut result_node = BacktraceNode::new(item);

        self.stack.push((item_state, item));

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
                            // surrounding context (1), and this will not
                            // trigger an infinite loop (2). This can
                            // occur if `...x` may be empty *and* the
                            // lookahead matches (if the lookahead
                            // doesn't match, then the only source for
                            // L is `...x`).

                            let continue_tracing =
                                maybe_empty && // (1)
                                item.lookahead == lookahead && // (1)
                                !self.stack.contains(&(pred_state, *item)); // (2)

                            if continue_tracing {
                                let parent_node = self.backtrace(pred_state, *item);
                                result_node.merge_parent(parent_node);
                            } else {
                                result_node.merge_parent(BacktraceNode::new(*item));
                            }
                        }
                    }
                }
            }
        }

        self.stack.pop().unwrap();

        result_node
    }
}

