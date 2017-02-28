use grammar::repr::*;
use lr1::core::*;
use lr1::lookahead::Lookahead;
use petgraph::{Direction, Graph};
use petgraph::graph::NodeIndex;

// Each state `s` corresponds to the node in the graph with index
// `s`. The edges are the shift transitions.
pub struct StateGraph {
    graph: Graph<(), Symbol>
}

impl StateGraph {
    pub fn new<'grammar, L>(states: &[State<'grammar, L>]) -> StateGraph
        where L: Lookahead
    {
        let mut graph = Graph::new();

        // First, create the nodes.
        for i in 0..states.len() {
            let j = graph.add_node(());
            assert_eq!(i, j.index());
        }

        // Add in the edges.
        for (i, state) in states.iter().enumerate() {
            // Successors of a node arise from:
            // - shifts (found in the `conflicts` and `tokens` maps)
            // - gotos (found in the `gotos` map)
            graph.extend_with_edges(
                state.shifts
                     .iter()
                     .map(|(&terminal, &state)| {
                         (Symbol::Terminal(terminal), state)
                     })
                     .chain(
                         state.gotos
                              .iter()
                              .map(|(&nt, &state)| (Symbol::Nonterminal(nt), state)))
                     .map(|(symbol, successor)| {
                         (NodeIndex::new(i), NodeIndex::new(successor.0), symbol)
                     }));
        }

        StateGraph { graph: graph }
    }

    /// Given a list of symbols `[X, Y, Z]`, traces back from
    /// `initial_state_index` to find the set of states whence we
    /// could have arrived at `initial_state_index` after pushing `X`,
    /// `Y`, and `Z`.
    pub fn trace_back(&self,
                      initial_state_index: StateIndex,
                      initial_symbols: &[Symbol])
                      -> Vec<StateIndex> {
        let mut stack = vec![(initial_state_index, initial_symbols)];
        let mut result = vec![];
        while let Some((state_index, symbols)) = stack.pop() {
            if let Some((head, tail)) = symbols.split_last() {
                stack.extend(
                    self.graph.edges_directed(NodeIndex::new(state_index.0),
                                              Direction::Incoming)
                              .filter(|&(_, symbol)| symbol == head)
                              .map(|(pred, _)| (StateIndex(pred.index()), tail)));
            } else {
                result.push(state_index);
            }
        }
        result.sort();
        result.dedup();
        result
    }

    pub fn successors(&self,
                      state_index: StateIndex)
                      -> Vec<StateIndex> {
        self.graph.edges_directed(NodeIndex::new(state_index.0),
                                  Direction::Outgoing)
                  .map(|(succ, _)| StateIndex(succ.index()))
                  .collect()
    }

    pub fn predecessors(&self,
                        state_index: StateIndex,
                        symbol: Symbol)
                        -> Vec<StateIndex> {
        self.graph.edges_directed(NodeIndex::new(state_index.0),
                                  Direction::Incoming)
                  .filter(|&(_, s)| *s == symbol)
                  .map(|(pred, _)| StateIndex(pred.index()))
                  .collect()
    }
}
