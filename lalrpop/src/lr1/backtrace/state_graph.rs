use lr1::{Action, State, StateIndex};
use petgraph::{EdgeDirection, Graph};
use petgraph::graph::NodeIndex;

// Each state `s` corresponds to the node in the graph with index
// `s`. The edges are the shift transitions.
pub struct StateGraph {
    graph: Graph<(), ()>
}

impl StateGraph {
    pub fn new<'grammar>(states: &[State<'grammar>]) -> StateGraph {
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
                state.conflicts.values()
                               .flat_map(|conflicts| conflicts)
                               .map(|conflict| &conflict.action)
                               .chain(state.tokens.values())
                               .filter_map(|action| match *action {
                                   Action::Shift(ref target) => Some(target),
                                   Action::Reduce(_) => None,
                               })
                               .chain(state.gotos.values())
                               .map(|&successor| (NodeIndex::new(i), NodeIndex::new(successor.0))));
        }

        StateGraph { graph: graph }
    }

    pub fn predecessors_at_distance(&self,
                                    state_index: StateIndex,
                                    distance: usize)
                                    -> Vec<StateIndex> {
        let mut result = vec![];
        let mut stack = Vec::new();
        stack.push((state_index, 0));
        while let Some((n, d)) = stack.pop() {
            if d == distance {
                result.push(n);
            } else {
                stack.extend(
                    self.graph.neighbors_directed(NodeIndex::new(n.0), EdgeDirection::Incoming)
                              .map(|pred| (StateIndex(pred.index()), d + 1)));
            }
        }
        result
    }

    /// Given a state `s`, returns all states `p` where either `p ==
    /// s` or `p` is an immediate predecessor of `s`.
    pub fn predecessors_or_self(&self, state_index: StateIndex) -> Vec<StateIndex> {
        self.graph.neighbors_directed(NodeIndex::new(state_index.0), EdgeDirection::Incoming)
                  .map(|n| StateIndex(n.index()))
                  .chain(Some(state_index))
                  .collect()
    }
}
