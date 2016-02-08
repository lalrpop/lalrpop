use lr1::first::FirstSets;
use lr1::{LR0Item, Item, Lookahead, State, StateIndex};
use grammar::repr::*;
use session::Session;
use util::Multimap;

mod example;
mod state_graph;
#[cfg(test)] mod test;

use self::CanShiftResult::*;
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
#[derive(Clone, Debug)]
pub struct Example {
    pub symbols: Vec<ExampleSymbol>,
    pub reductions: Vec<Reduction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExampleSymbol {
    Symbol(Symbol),
    Cursor,
    Epsilon,
}

#[derive(Copy, Clone, Debug)]
pub struct Reduction {
    pub start: usize,
    pub end: usize,
    pub nonterminal: NonterminalString,
}

impl<'grammar> BacktraceNode<'grammar> {
    fn new(item: LR0Item<'grammar>) -> Self {
        BacktraceNode { item: item, parents: vec![] }
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

    pub fn backtrace_shift(&mut self,
                           item_state: StateIndex,
                           item: LR0Item<'grammar>,
                           lookaheads: &[Lookahead])
                           -> BacktraceNode<'grammar> {
        log!(self.session, Debug, "backtrace_shift(item_state={:?}, item={:?})",
             item_state, item);

        let mut result_node = BacktraceNode::new(item);

        // Otherwise, we have something like `Foo = (*) ... [L]`.  We
        // want to find out where this item came from. Either we are
        // in the start state, or else it arose from a \epsilon
        // transition.  In the latter case, then there is another item
        // in the same state like `Bar = ...p (*) Foo ...s [L1]` where
        // `L in First(...s, L1)`. Find that item.

        let nt_sym = Symbol::Nonterminal(item.production.nonterminal);

        let mut pred_items = Multimap::new();
        let pred_states = self.state_graph.predecessors_at_distance(item_state, item.index);
        for pred_state in pred_states {
            for &pred_item in self.states[pred_state.0].items.vec.iter() {
                log!(self.session, Debug,
                     "backtrace_shift: pred_state={:?} pred_item={:?}",
                     pred_state, pred_item);
                match self.can_shift_with_lookahead_from(pred_item, nt_sym, lookaheads) {
                    CannotShift => { }
                    CanShift(_) => {
                        // Found something like the:
                        //
                        //     Bar = ...p (*) Foo ...s [L1]
                        //
                        // that we were looking for. At this point, if
                        // `...p` is non-empty, we are
                        // done. Otherwise, we must recurse further.
                        pred_items.push(LR0Item { production: pred_item.production,
                                                  index: pred_item.index },
                                        pred_item.lookahead);
                    }
                }
            }
        }

        for (pred_item0, lookaheads) in pred_items {
            log!(self.session, Debug,
                 "backtrace_shift: pred_item0={:?} lookaheads={:?}",
                 pred_item0, lookaheads);
            let parent_node = if pred_item0.index > 0 {
                BacktraceNode::new(pred_item0)
            } else {
                self.backtrace_shift(item_state, pred_item0, &lookaheads)
            };
            result_node.merge_parent(parent_node);
        }

        result_node
    }

    /// Returns a backtrace explaining how the state `item_state` came
    /// to contain the reducable item `item`:
    ///
    ///    NT = ... (*) [L]
    ///
    /// In particular, how we came to be able to reduce `NT` with
    /// lookahead `L`. Basically, we want to walk back through the
    /// states to find one where there is some concrete context:
    ///
    ///    NT = ... (*) L ...
    pub fn backtrace_reduce(&mut self, item_state: StateIndex, item: Item<'grammar>)
                            -> BacktraceNode<'grammar> {
        log!(self.session, Debug, "backtrace_reduce(item_state={:?} item={:?})", item_state, item);

        let mut result_node = BacktraceNode::new(item.to_lr0());

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
            for &pred_item in self.states[pred_state.0].items.vec.iter() {
                log!(self.session, Debug, "backtrace: pred_state {:?} has item {:?}",
                     pred_state, pred_item);
                match self.can_shift_with_lookahead(pred_item, nt_sym, lookahead) {
                    CannotShift => { }
                    CanShift(maybe_empty) => {
                        // Found such a state. Now, continue tracing
                        // back so long as the lookahead may still
                        // have come from the surrounding context (1),
                        // and this will not trigger an infinite loop
                        // (2). This can occur if `...x` may be empty
                        // *and* the lookahead matches (if the
                        // lookahead doesn't match, then the only
                        // source for L is `...x`).
                        log!(self.session, Debug, "backtrace: maybe_empty={:?}",
                             maybe_empty);

                        let continue_tracing =
                            maybe_empty && // (1)
                            pred_item.lookahead == lookahead && // (1)
                            !self.stack.contains(&(pred_state, pred_item)); // (2)

                        let parent_node = if continue_tracing {
                            self.backtrace_reduce(pred_state, pred_item)
                        } else {
                            BacktraceNode::new(pred_item.to_lr0())
                        };
                        result_node.merge_parent(parent_node);
                    }
                }
            }
        }

        self.stack.pop().unwrap();

        result_node
    }

    fn can_shift_with_lookahead(&self,
                                item: Item<'grammar>,
                                nt_sym: Symbol,
                                lookahead: Lookahead)
                                -> CanShiftResult
    {
        self.can_shift_with_lookahead_from(item, nt_sym, &[lookahead])
    }

    fn can_shift_with_lookahead_from(&self,
                                     item: Item<'grammar>,
                                     nt_sym: Symbol,
                                     lookaheads: &[Lookahead])
                                     -> CanShiftResult
    {
        if let Some((shifted, remainder)) = item.shift_symbol() {
            if shifted == nt_sym {
                let (first, maybe_empty) =
                    self.first_sets.first(remainder, item.lookahead);
                for l in lookaheads {
                    if first.contains(&l) {
                        return CanShift(maybe_empty);
                    }
                }
            }
        }
        CannotShift
    }
}

enum CanShiftResult {
    CannotShift,
    CanShift(bool) // if true, remainder is maybe empty
}
