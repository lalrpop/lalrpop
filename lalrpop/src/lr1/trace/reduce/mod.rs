use lr1::core::*;
use lr1::example::*;
use lr1::lookahead::{Lookahead, LookaheadSet};
use grammar::repr::*;
use util::{Map, map};

use super::Tracer;
use super::trace_graph::*;

use self::CanShiftResult::*;

#[cfg(test)] mod test;

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
#[derive(Clone, Debug)]
pub struct BacktraceNode<'grammar> {
    pub item: LR0Item<'grammar>,
    pub parents: Vec<BacktraceNode<'grammar>>,
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
    pub fn backtrace_reduce_graph(mut self,
                                  item_state: StateIndex,
                                  item: Item<'grammar>)
                                  -> TraceGraph<'grammar>
    {
        self.trace_reduce_item(item_state, item);
        self.trace_graph
    }

    fn trace_reduce_item(&mut self,
                         item_state: StateIndex,
                         item: Item<'grammar>)
    {
        log!(self.session, Debug, "trace_reduce_item(item_state={:?} item={:?})",
             item_state, item);

        // We start out with an item
        //
        //     X = ...p (*) ...s [L]
        //
        // which we can (eventually) reduce when we see [L], though we
        // may have to do some epsilon reductions first if ...s is
        // non-empty.
        let nonterminal = item.production.nonterminal; // X
        let lookahead = item.lookahead; // L

        // Add an edge
        //
        //     [X] -{...p}-> [X = ...p (*) ...s]
        //
        // because to reach that item we pushed `...p` from the start
        // of `X` (or, to interpret another way, we could pop `...p`
        // to reach the start of `X`).
        self.trace_graph.add_edge(nonterminal, item, item.prefix());

        // Walk back to the set of states S where we had:
        //
        //     X = (*) ...p
        let pred_states = self.state_graph.trace_back(item_state, item.prefix());

        // Add in edges from [X] to all the places [X] can be consumed.
        for pred_state in pred_states {
            self.trace_reduce_from_state(pred_state, nonterminal, lookahead);
        }
    }

    // We know that we can reduce the nonterminal Y with lookahead
    // L. We want to find out who will consume that reduced value. So
    // search for those items that can shift an `X`:
    //
    //     Z = ... (*) Y ...s [L1]
    //
    // where L is in FIRST(...s, L1).
    //
    // (Note that `lookahead` remains constant for the entire reduce backtrace.)
    fn trace_reduce_from_state(&mut self,
                               item_state: StateIndex,
                               nonterminal: NonterminalString, // "Y"
                               lookahead: Lookahead) // "L"
    {
        log!(self.session, Debug,
             "trace_reduce_from_state(item_state={:?}, \
              nonterminal={:?}, lookahead={:?})",
             item_state, nonterminal, lookahead);
        if !self.visited_set.insert((item_state, nonterminal)) {
            return;
        }
        for &pred_item in self.states[item_state.0].items.vec.iter() {
            match self.can_shift_with_lookahead(pred_item, nonterminal, lookahead) {
                CannotShift => { }
                CanShift(maybe_empty) => {
                    // Found a state:
                    //
                    //     Z = ...p (*) Y ...s [L1]
                    //
                    // where L is in FIRST(...s, L1). We need to
                    // continue tracing back so long as the lookahead
                    // may still have come from the surrounding
                    // context. This can occur if `...x` may be empty
                    // *and* the lookahead matches (if the lookahead
                    // doesn't match, then the only source for L is
                    // `...x`).
                    let continue_tracing =
                        maybe_empty &&
                        pred_item.lookahead == lookahead;

                    if !continue_tracing {
                        // Add an edge
                        //
                        //    [Z = ...p (*) Y ...s] -{}-> [Y]
                        //
                        // and stop.
                        self.trace_graph.add_edge(pred_item, nonterminal, &[]);
                    } else {
                        // Add an edge
                        //
                        //    [Z] -{..p}-> [Y]
                        //
                        // because we can reduce by consuming `...p`
                        // tokens, and continue tracing.
                        self.trace_graph.add_edge(
                            pred_item.production.nonterminal,
                            nonterminal,
                            pred_item.prefix());

                        self.trace_reduce_item(item_state, pred_item);
                    }
                }
            }
        }
    }

    /// Returns a backtrace explaining how the state `item_state` came
    /// to contain a set of shiftable items, each with the same LR0
    /// subset (represented by `item`) but with various lookaheads
    /// (`lookaheads`):
    ///
    ///    NT1 = ... (*) X ... [L0]
    ///    NT1 = ... (*) X ... [..]
    ///    NT1 = ... (*) X ... [Ln]
    ///
    /// In particular, we are looking for some prior state that gives
    /// a hint at the tokens preceding `NT1`, e.g.:
    ///
    ///    NT2 = ...p (*) NT1 ...s
    ///
    /// where the prefix `...p` is non-empty.
    pub fn backtrace_shift(&mut self,
                           item_state: StateIndex,
                           item: LR0Item<'grammar>,
                           lookaheads: LookaheadSet)
                           -> BacktraceNode<'grammar> {
        {
            let lookaheads = lookaheads.debug(self.grammar);
            log!(self.session, Debug, "backtrace_shift(item_state={:?}, item={:?}, \
                                       lookaheads={:?})",
                 item_state, item, lookaheads);
        }

        let key = ShiftKey {
            item_state: item_state,
            item: item,
            lookaheads: lookaheads,
        };

        // check for previous result (or a cycle)
        if let Some(result) = self.shift_cache.lookup(&key) {
            return result;
        }

        let mut result_node = BacktraceNode::new(item);

        // insert a placeholder so that we can detect cycles
        self.shift_cache.insert(key.clone(), result_node.clone());

        // `NT1`, in the example above.
        let nonterminal = item.production.nonterminal;

        // If the cursor is not at the start of `NT1`, then walk back
        // through the graph to the state(s) where it was.
        let pred_states = self.state_graph.trace_back(item_state, item.prefix());

        // Each of those pred states must contain `NT1 = (*) ... [Li]`
        // (where Li in lookaheads). Either this is the start state
        // (and NT1 is the start nonterminal), or else this item must
        // have been added by an epsilon transition from some other item
        //
        //     NT2 = ...p (*) NT1 ...s [L]
        //
        // Look for items like that and collect them up into a map
        // going from LR0 items to a lookahead set.
        let mut pred_items: Map<LR0Item<'grammar>, LookaheadSet> = map();
        for pred_state in pred_states {
            for &pred_item in self.states[pred_state.0].items.vec.iter() {
                log!(self.session, Debug,
                     "backtrace_shift: pred_state={:?} pred_item={:?}",
                     pred_state, pred_item);
                match self.can_shift_with_lookahead_from(pred_item,
                                                         nonterminal,
                                                         &key.lookaheads) {
                    CannotShift => { }
                    CanShift(_) => {
                        pred_items
                            .entry(LR0Item { production: pred_item.production,
                                             index: pred_item.index })
                            .or_insert_with(|| LookaheadSet::new(self.grammar))
                            .insert(self.grammar, pred_item.lookahead);
                    }
                }
            }
        }

        for (pred_item0, lookaheads) in pred_items {
            log!(self.session, Debug,
                 "backtrace_shift: pred_item0={:?} lookaheads={:?}",
                 pred_item0, lookaheads.debug(&*self.grammar));

            // For each of the items we found, we want to continue
            // tracing if `...p` is empty, since we haven't actually
            // gained any context. (Note that the recursive step will
            // also detect cycles.)
            let parent_node = if pred_item0.index == 0 {
                self.backtrace_shift(item_state, pred_item0, lookaheads)
            } else {
                BacktraceNode::new(pred_item0)
            };

            result_node.merge_parent(parent_node);
        }

        self.shift_cache.update(&key, result_node.clone());

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

        self.reduce_stack.push((item_state, item));

        // The nonterminal NT and lookahead L we are looking for
        let nonterminal = item.production.nonterminal;
        let lookahead = item.lookahead;

        // We will have arrived at the current state after pushing N
        // symbols, where N is the number of items pushed so far in
        // `item`. So walk backwards N states to find the state(s)
        // where we had something like
        //
        //     A := ... (*) NT ... [L1]
        let pred_states = self.state_graph.trace_back(item_state, item.prefix());
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
                match self.can_shift_with_lookahead(pred_item, nonterminal, lookahead) {
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
                            !self.reduce_stack.contains(&(pred_state, pred_item)); // (2)

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

        self.reduce_stack.pop().unwrap();

        result_node
    }

    fn can_shift_with_lookahead(&self,
                                item: Item<'grammar>,
                                nonterminal: NonterminalString,
                                lookahead: Lookahead)
                                -> CanShiftResult
    {
        self.can_shift_with_lookahead_core(
            item,
            nonterminal,
            |first| first.contains(self.grammar, lookahead))
    }

    fn can_shift_with_lookahead_from(&self,
                                     item: Item<'grammar>,
                                     nonterminal: NonterminalString,
                                     lookaheads: &LookaheadSet)
                                     -> CanShiftResult
    {
        self.can_shift_with_lookahead_core(
            item,
            nonterminal,
            |first| first.intersects(lookaheads))
    }

    fn can_shift_with_lookahead_core<F>(&self,
                                        item: Item<'grammar>,
                                        nonterminal: NonterminalString,
                                        mut contains: F)
                                        -> CanShiftResult
        where F: FnMut(&LookaheadSet) -> bool
    {
        if let Some((shifted, remainder)) = item.shift_symbol() {
            if shifted == Symbol::Nonterminal(nonterminal) {
                let (first, maybe_empty) =
                    self.first_sets.first(self.grammar, remainder, item.lookahead);
                if contains(&first) {
                    return CanShift(maybe_empty);
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

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct ShiftKey<'grammar> {
    item_state: StateIndex,
    item: LR0Item<'grammar>,
    lookaheads: LookaheadSet,
}

pub struct ShiftCache<'grammar> {
    cache: Map<ShiftKey<'grammar>, BacktraceNode<'grammar>>
}

impl<'grammar> ShiftCache<'grammar> {
    pub fn new() -> Self {
        ShiftCache { cache: map() }
    }

    fn lookup(&self, state: &ShiftKey<'grammar>) -> Option<BacktraceNode<'grammar>> {
        self.cache.get(state).cloned()
    }

    fn insert(&mut self, state: ShiftKey<'grammar>, node: BacktraceNode<'grammar>) {
        let prev = self.cache.insert(state, node);
        assert!(prev.is_none());
    }

    fn update(&mut self, state: &ShiftKey<'grammar>, node: BacktraceNode<'grammar>) {
        *self.cache.get_mut(state).unwrap() = node;
    }
}
