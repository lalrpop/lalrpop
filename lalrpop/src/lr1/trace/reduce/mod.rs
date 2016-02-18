use lr1::core::*;
use lr1::lookahead::Lookahead;
use grammar::repr::*;

use super::Tracer;
use super::trace_graph::*;

use self::CanShiftResult::*;

#[cfg(test)] mod test;

impl<'trace, 'grammar> Tracer<'trace, 'grammar> {
    pub fn backtrace_reduce(mut self,
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
        //     [X] -{...p,_,...s}-> [X = ...p (*) ...s]
        //
        // because to reach that item we pushed `...p` from the start
        // of `X` and afterwards we expect to see `...s`.
        self.trace_graph.add_edge(nonterminal, item, item.symbol_sets());

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
                        //    [Z = ...p (*) Y ...s] -(...p,Y,...s)-> [Y]
                        //
                        // and stop.
                        self.trace_graph.add_edge(pred_item,
                                                  nonterminal,
                                                  pred_item.symbol_sets());
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
                            pred_item.symbol_sets());

                        self.trace_reduce_item(item_state, pred_item);
                    }
                }
            }
        }
    }

    fn can_shift_with_lookahead(&self,
                                item: Item<'grammar>,
                                nonterminal: NonterminalString,
                                lookahead: Lookahead)
                                -> CanShiftResult
    {
        if let Some((shifted, remainder)) = item.shift_symbol() {
            if shifted == Symbol::Nonterminal(nonterminal) {
                let (first, maybe_empty) =
                    self.first_sets.first(self.grammar, remainder, item.lookahead);
                if first.contains(self.grammar, lookahead) {
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
