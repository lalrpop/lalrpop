use lr1::first::FirstSets;
use lr1::lookahead::{Lookahead, LookaheadSet};
use lr1::{LR0Item, Item, State, StateIndex};
use grammar::repr::*;
use session::Session;
use util::{Map, map};

mod example;
mod state_graph;
#[cfg(test)] mod test;

use self::CanShiftResult::*;
use self::state_graph::StateGraph;

pub use self::example::{Example, ExampleSymbol, ExampleIterator, ExampleStyles};

pub struct Tracer<'trace, 'grammar: 'trace> {
    session: &'trace Session,
    grammar: &'trace Grammar,
    states: &'trace [State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
    shift_cache: ShiftCache<'grammar>,
    reduce_stack: Vec<(StateIndex, Item<'grammar>)>,
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
#[derive(Clone, Debug)]
pub struct BacktraceNode<'grammar> {
    item: LR0Item<'grammar>,
    parents: Vec<BacktraceNode<'grammar>>,
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
            grammar: grammar,
            states: states,
            first_sets: FirstSets::new(grammar),
            state_graph: StateGraph::new(states),
            shift_cache: ShiftCache::new(),
            reduce_stack: vec![],
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
        let nt_sym = Symbol::Nonterminal(item.production.nonterminal);

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
                                                         nt_sym,
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
        let nt_sym = Symbol::Nonterminal(item.production.nonterminal);
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
                                nt_sym: Symbol,
                                lookahead: Lookahead)
                                -> CanShiftResult
    {
        self.can_shift_with_lookahead_core(
            item,
            nt_sym,
            |first| first.contains(self.grammar, lookahead))
    }

    fn can_shift_with_lookahead_from(&self,
                                     item: Item<'grammar>,
                                     nt_sym: Symbol,
                                     lookaheads: &LookaheadSet)
                                     -> CanShiftResult
    {
        self.can_shift_with_lookahead_core(
            item,
            nt_sym,
            |first| first.intersects(lookaheads))
    }

    fn can_shift_with_lookahead_core<F>(&self,
                                        item: Item<'grammar>,
                                        nt_sym: Symbol,
                                        mut contains: F)
                                        -> CanShiftResult
        where F: FnMut(&LookaheadSet) -> bool
    {
        if let Some((shifted, remainder)) = item.shift_symbol() {
            if shifted == nt_sym {
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

struct ShiftCache<'grammar> {
    cache: Map<ShiftKey<'grammar>, BacktraceNode<'grammar>>
}

impl<'grammar> ShiftCache<'grammar> {
    fn new() -> Self {
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
