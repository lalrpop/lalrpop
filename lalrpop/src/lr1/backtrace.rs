use petgraph::graph::{Graph, NodeIndex};

struct Backtrace<'grammar> {
    items: Vec<Item<'grammar>>
}

/// Given that the state `state` can reduce `production` when seeing
/// `lookahead` --- in other words, that it contains
///
///    NT = ... (*) [L]
///
/// where `production = NT = ...` --- then this function traverses the
/// state graph to give a backtrace explaining *why* this state exists
/// with that production and lookahead in the first place.
pub fn backtraces(states: &[State<'grammar>],
                  state: StateIndex,
                  production: &'grammar Production,
                  lookahead: Lookaround)
                  -> Vec<Backtrace<'grammar>> {
}

// Each state `s` corresponds to the node in the graph with index
// `s`. The edges are the shift transitions.
type StateGraph = Graph<(), TerminalString>;

fn state_graph(states: &[State<'grammar>]) -> StateGraph {
    let mut graph = Graph::new();

    // First, create the nodes.
    for i in 0..states.len() {
        let j = graph.add_node(());
        assert_eq!(i, j);
    }

    // Add in the edges.
    for (i, state) in states.iter().enumerate() {
        let all_actions =
            state.conflicts.iter()
                           .flat_map(|(lookahead, conflicts)| {
                               conflicts.iter()
                                        .map(|c| (lookahead, &c.action))
                           })
                           .chain(state.actions.iter());
        for (&lookahead, action) in all_actions {
            match action {
                Action::Shift(target) => { graph.add_edge(i, target.0 as usize, lookahead); }
                Action::Reduce(_) => { }
            }
        }
    }
}

{
    if item.index == 0 {
        // The item that caused a conflict looked like:
        //
        //     X := (*) ...y [K]
        //
        // This could arise in two scenarios. Either there
        // is another item in the same state like:
        //
        //     Z := ... (*) X ...
        //
        // and hence we added the conflicting item due to an epsilon
        // move, or else in some other item we have:
        
        

        //     
        
        //
        //     
    }

    
    for state in states {
        for item in &state.items {
            // Look for an item in some state like:
            //
            //     X := ...x (*) Y ...z [K]
            //
            // or
            //
            //     Y := ...x (*) [K]
            //
            // where `Y` is the nonterminal that we are looking for, and
            // lookahead in `FIRST(...z, K)`.
            // Note that these items might appear in the *same state*
            
            
        }
    }
}

