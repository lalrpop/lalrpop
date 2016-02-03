use lr1::first::FirstSets;
use lr1::{LR0Item, Item, State, StateIndex};
use grammar::repr::*;
use session::Session;

mod state_graph;
mod test;

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

pub fn examples<'ex>(backtrace_node: &'ex BacktraceNode<'ex>)
                     -> ExampleIterator<'ex>
{
    ExampleIterator::new(backtrace_node)
}

pub struct ExampleIterator<'ex> {
    stack: Vec<ExampleState<'ex>>,
}

#[derive(Debug)]
pub struct Example {
    pub symbols: Vec<Symbol>,
    pub reductions: Vec<Reduction>,
}

#[derive(Debug)]
pub struct Reduction {
    pub start: usize,
    pub end: usize,
    pub nonterminal: NonterminalString,
}

#[derive(Debug)]
struct ExampleState<'ex> {
    // Node we are exploring
    node: &'ex BacktraceNode<'ex>,

    // Index of next parent to explore
    index: usize,
}

impl<'ex> ExampleIterator<'ex> {
    pub fn new(backtrace: &'ex BacktraceNode<'ex>) -> Self {
        let mut this = ExampleIterator { stack: vec![] };
        this.stack.push(ExampleState { node: backtrace, index: 0 });
        this.populate();
        this
    }

    fn populate(&mut self) -> bool {
        println!("populate(self.stack={:?})", self.stack);
        let parent = {
            // Obtain parent from top of stack, if any, and increment
            // index for top of stack.
            let top = self.stack.last_mut().expect("populate called but stack is empty");
            let index = top.index;
            if index == top.node.parents.len() {
                return false; // top of stack has no parent
            }
            top.index += 1;
            &top.node.parents[index]
        };
        self.stack.push(ExampleState { node: parent, index: 0 });
        self.populate();
        return true; // top of stack had a parent (now pushed)
    }

    fn iterate(&mut self) {
        // When this function is called, the top of the stack should
        // always be some leaf node in the tree.
        let top = self.stack.pop().unwrap();
        assert!(top.node.parents.len() == 0 && top.index == 0);

        println!("iterate(top={:?})", top);

        while !self.stack.is_empty() {
            if self.populate() {
                return;
            }

            self.stack.pop();
        }
    }

    fn unwind<I: Iterator<Item=&'ex LR0Item<'ex>>>(&self,
                                                   mut rev_items: I,
                                                   example: &mut Example) {
        let item = if let Some(item) = rev_items.next() {
            item
        } else {
            return;
        };

        println!("unwind(item={:?}", item);

        let start = example.symbols.len();

        let prefix = &item.production.symbols[..item.index];
        println!("unwind: prefix={:?}", prefix);
        example.symbols.extend(prefix);

        self.unwind(rev_items, example);

        if item.index != item.production.symbols.len() {
            let suffix = &item.production.symbols[item.index+1..];
            println!("unwind: suffix={:?}", suffix);
            example.symbols.extend(suffix);
        }

        let end = example.symbols.len();
        example.reductions.push(Reduction {
            start: start,
            end: end,
            nonterminal: item.production.nonterminal
        });
    }
}

impl<'ex> Iterator for ExampleIterator<'ex> {
    type Item = Example;

    fn next(&mut self) -> Option<Example> {
        println!("stack: {:?}", self.stack);

        if self.stack.is_empty() {
            return None;
        }

        let mut example = Example {
            symbols: vec![],
            reductions: vec![],
        };

        {
            let rev_items = self.stack.iter().rev().map(|s| &s.node.item);
            self.unwind(rev_items, &mut example);
        }

        self.iterate();

        Some(example)
    }
}

