//! Code to trace out a single lane, collecting information into the
//! lane table as we go.

use collections::Set;
use grammar::repr::*;
use lr1::core::*;
use lr1::first::FirstSets;
use lr1::lookahead::*;
use lr1::state_graph::StateGraph;

use super::table::{ConflictIndex, LaneTable};

pub struct LaneTracer<'trace, 'grammar: 'trace> {
    grammar: &'grammar Grammar,
    states: &'trace [LR0State<'grammar>],
    first_sets: FirstSets,
    state_graph: StateGraph,
    table: LaneTable<'grammar>,
}

impl<'trace, 'grammar> LaneTracer<'trace, 'grammar> {
    pub fn new(grammar: &'grammar Grammar,
               states: &'trace [LR0State<'grammar>],
               conflicts: usize)
               -> Self {
        LaneTracer {
            grammar: grammar,
            states: states,
            first_sets: FirstSets::new(grammar),
            state_graph: StateGraph::new(states),
            table: LaneTable::new(grammar, conflicts),
        }
    }

    pub fn into_table(self) -> LaneTable<'grammar> {
        self.table
    }

    pub fn start_trace(&mut self,
                       state: StateIndex,
                       conflict: ConflictIndex,
                       item: LR0Item<'grammar>) {
        let mut visited_set = Set::default();

        // if the conflict item is a "shift" item, then the context
        // is always the termianl to shift (and conflicts only arise
        // around shifting terminal, so it must be a terminal)
        match item.shift_symbol() {
            Some((Symbol::Terminal(term), _)) => {
                let mut token_set = TokenSet::new(self.grammar);
                token_set.insert(self.grammar, Token::Terminal(term));
                self.table.add_lookahead(state, conflict, &token_set);
            }

            Some((Symbol::Nonterminal(_), _)) => {
                panic!("invalid conflict item `{:?}`: shifts nonterminal",
                       item);
            }

            None => {
                self.continue_trace(state, conflict, item, &mut visited_set);
            }
        }
    }

    fn continue_trace(&mut self,
                      state: StateIndex,
                      conflict: ConflictIndex,
                      item: LR0Item<'grammar>,
                      visited: &mut Set<(StateIndex, LR0Item<'grammar>)>) {
        if !visited.insert((state, item)) {
            return;
        }

        if item.index > 0 {
            // This item was reached by shifting some symbol.  We need
            // to unshift that symbol, which means we walk backwards
            // to predecessors of `state` in the state graph.
            //
            // Example:
            //
            //     X = ...p T (*) ...s
            //
            // Here we would be "unshifting" T, which means we will
            // walk to predecessors of the current state that were
            // reached by shifting T. Those predecessors will contain
            // an item like `X = ...p (*) T ...s`, which we will then
            // process in turn.
            let shifted_symbol = item.production.symbols[item.index - 1];
            let unshifted_item = Item { index: item.index - 1, ..item };
            let predecessors = self.state_graph.predecessors(state, shifted_symbol);
            for predecessor in predecessors {
                self.table.add_successor(state, predecessor);
                self.continue_trace(predecessor, conflict, unshifted_item, visited);
            }
            return;
        }

        // Either: we are in the start state, or this item was
        // reached by an epsilon transition. We have to
        // "unepsilon", which means that we search elsewhere in
        // the state for where the epsilon transition could have
        // come from.
        //
        // Example:
        //
        //     X = (*) ...
        //
        // We will search for other items in the same state like:
        //
        //     Y = ...p (*) X ...s
        //
        // We can then insert `FIRST(...s)` as lookahead for
        // `conflict`. If `...s` may derive epsilon, though, we
        // have to recurse and search with the previous item.

        let state_items = &self.states[state.0].items.vec;
        let nonterminal = item.production.nonterminal;
        for &pred_item in state_items.iter()
                                     .filter(|i| i.can_shift_nonterminal(nonterminal)) {
            let symbol_sets = pred_item.symbol_sets();
            let mut first = self.first_sets.first0(self.grammar, symbol_sets.suffix);
            let derives_epsilon = first.take_eof(self.grammar);
            self.table.add_lookahead(state, conflict, &first);
            if derives_epsilon {
                self.continue_trace(state, conflict, pred_item, visited);
            }
        }
    }
}
