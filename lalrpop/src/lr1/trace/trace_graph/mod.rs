use lr1::core::*;
use grammar::repr::*;
use petgraph::{EdgeDirection, Graph};
use petgraph::graph::{Edges, NodeIndex};
use std::fmt::{Debug, Formatter, Error};
use util::{Map, map};

#[cfg(test)] mod test;

pub struct TraceGraph<'grammar> {
    // A -L-> B means:
    //
    //     Transition from a state containing A to a state containing
    //     B by (pushing|popping) the symbols L.
    //
    // If this trace graph represents a shift backtrace, then the
    // labels are symbols that are pushed. Otherwise they are labels
    // that are popped.
    graph: Graph<TraceGraphNode<'grammar>, &'grammar [Symbol]>,
    indices: Map<TraceGraphNode<'grammar>, NodeIndex>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum TraceGraphNode<'grammar> {
    Nonterminal(NonterminalString),
    Item(LR0Item<'grammar>),
}

impl<'grammar> TraceGraph<'grammar> {
    pub fn new() -> TraceGraph<'grammar> {
        TraceGraph {
            graph: Graph::new(),
            indices: map(),
        }
    }

    pub fn add_node<T>(&mut self, node: T) -> NodeIndex
        where T: Into<TraceGraphNode<'grammar>>
    {
        let node = node.into();
        let graph = &mut self.graph;
        *self.indices.entry(node)
                     .or_insert_with(|| graph.add_node(node))
    }

    pub fn add_edge<F,T>(&mut self,
                         from: F,
                         to: T,
                         labels: &'grammar [Symbol])
        where F: Into<TraceGraphNode<'grammar>>,
              T: Into<TraceGraphNode<'grammar>>,
    {
        let from = self.add_node(from.into());
        let to = self.add_node(to.into());
        println!("add_edge({:?} -{:?}-> {:?})",
                 self.graph[from], labels, self.graph[to]);
        if !self.graph.edges_directed(from, EdgeDirection::Outgoing)
                      .any(|(t, &l)| t == to && l == labels)
        {
            self.graph.add_edge(from, to, labels);
        }
    }

    pub fn enumerate_paths_from<'graph>(&'graph self,
                                        lr0_item: LR0Item<'grammar>)
                                        -> PathEnumerator<'graph, 'grammar>
    {
        PathEnumerator::new(self, lr0_item)
    }
}

impl<'grammar> Into<TraceGraphNode<'grammar>> for NonterminalString {
    fn into(self) -> TraceGraphNode<'grammar> {
        TraceGraphNode::Nonterminal(self)
    }
}

impl<'grammar> Into<TraceGraphNode<'grammar>> for LR0Item<'grammar> {
    fn into(self) -> TraceGraphNode<'grammar> {
        TraceGraphNode::Item(self)
    }
}

impl<'grammar> Into<TraceGraphNode<'grammar>> for Item<'grammar> {
    fn into(self) -> TraceGraphNode<'grammar> {
        TraceGraphNode::Item(self.to_lr0())
    }
}

// This just exists to help with the `Debug` impl
struct TraceGraphEdge<'grammar> {
    from: TraceGraphNode<'grammar>,
    to: TraceGraphNode<'grammar>,
    label: &'grammar [Symbol]
}

impl<'grammar> Debug for TraceGraphEdge<'grammar> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({:?} -{:?}-> {:?})", self.from, self.label, self.to)
    }
}

impl<'grammar> Debug for TraceGraph<'grammar> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut s = fmt.debug_list();
        for (&node, &index) in &self.indices {
            for (target, label) in
                self.graph.edges_directed(index, EdgeDirection::Outgoing)
            {
                s.entry(&TraceGraphEdge { from: node,
                                          to: self.graph[target],
                                          label: label });
            }
        }
        s.finish()
    }
}

///////////////////////////////////////////////////////////////////////////
// PathEnumerator
//
// The path enumerater walks a trace graph searching for paths that
// start at a given item and terminate at another item. If such a path
// is found, you can then find the complete list of symbols by calling
// `symbols_and_cursor` and also get access to the state.

pub struct PathEnumerator<'graph, 'grammar: 'graph> {
    graph: &'graph TraceGraph<'grammar>,
    stack: Vec<EnumeratorState<'graph, 'grammar>>,

    // The list of symbols that have been pushed onto the path so far.
    // During iteration, this list is reversed so that we can push
    // onto the end rather than inserting at the front. When we find a
    // path, we reverse it back.
    symbols: Vec<Symbol>,

    // The cursor represents the point where the error occurred. It
    // too must be mirrored when the list of symbols is reversed.
    cursor: usize,

    // When we find a path, the final item may have some number of
    // items to tack onto the end of the symbols list. `reset` stores
    // the length before those were appended so that, when `advance`
    // is next called, they can be truncated. The reason that this
    // needs a special value is that those symbols effectively
    // represent a "suffix" that will change as we walk the graph.
    reset: usize,
}

struct EnumeratorState<'graph, 'grammar: 'graph> {
    index: NodeIndex,
    symbols_len: usize,
    edges: Edges<'graph, &'grammar [Symbol]>,
}

impl<'graph, 'grammar> PathEnumerator<'graph, 'grammar> {
    fn new(graph: &'graph TraceGraph<'grammar>,
           lr0_item: LR0Item<'grammar>)
           -> Self {
        let start_state = graph.indices[&TraceGraphNode::Item(lr0_item)];
        let start_symbols = &lr0_item.production.symbols[lr0_item.index..];
        let mut enumerator = PathEnumerator {
            graph: graph,
            stack: vec![],
            symbols: start_symbols.iter()
                                  .cloned()
                                  .rev()
                                  .collect(),
            cursor: start_symbols.len(),
            reset: 0,
        };
        let edges = enumerator.incoming_edges(start_state);
        enumerator.stack.push(EnumeratorState {
            index: start_state,
            symbols_len: start_symbols.len(),
            edges: edges,
        });
        enumerator.find_next_trace();
        enumerator
    }

    /// Advance to the next example. Returns false if there are no more
    /// examples.
    pub fn advance(&mut self) -> bool {
        // while advancing, we keep the symbols reversed so that we
        // can push onto the back rather than inserting in the front
        self.symbols.truncate(self.reset);
        self.symbols.reverse();
        self.cursor = self.symbols.len() - self.cursor;
        self.find_next_trace()
    }

    fn incoming_edges(&self, index: NodeIndex) -> Edges<'graph, &'grammar [Symbol]> {
        self.graph.graph.edges_directed(index, EdgeDirection::Incoming)
    }

    /// This is the main operation, written in CPS style and hence it
    /// can seem a bit confusing. The idea is that `find_next_trace`
    /// is called when we are ready to consider the next child of
    /// whatever is on the top of the stack. It simply withdraws
    /// that next child (if any) and hands it to `push_next`.
    fn find_next_trace(&mut self) -> bool {
        println!("proceed()");
        if !self.stack.is_empty() {
            let next_edge = {
                let top_of_stack = self.stack.last_mut().unwrap();
                self.symbols.truncate(top_of_stack.symbols_len);
                top_of_stack.edges.next()
            };
            self.push_next_child_if_any(next_edge)
        } else {
            false
        }
    }

    /// Invoked with the next child (if any) of the node on the top of
    /// the stack.
    ///
    /// If `next` is `Some`, we simply call `push_next_child`.
    ///
    /// If `next` is `None`, then the node on the top of
    /// the stack *has* no next child, and so it is popped, and then
    /// we call `find_next_trace` again to start with the next child
    /// of the new top of the stack.
    fn push_next_child_if_any(&mut self,
                              next: Option<(NodeIndex, &'graph &'grammar [Symbol])>)
                              -> bool {
        if let Some((index, &prefix)) = next {
            self.push_next_child(index, prefix)
        } else {
            self.stack.pop();
            self.find_next_trace()
        }
    }

    /// Push the next child of the top of the stack onto the stack,
    /// making the child the new top.
    ///
    /// If the child is an `Item` node, we have found the next trace,
    /// and hence our search terminates. We push the symbols from this
    /// item node into the symbols vector and then call `found_trace`
    /// (which will ultimately return `true`).
    ///
    /// Otherwise, we check whether this new node would cause a cycle.
    /// If so, we do *not* push it, and instead just call
    /// `find_next_trace` again to proceed to the next child of the
    /// current top.
    ///
    /// Finally, if the new node would NOT cause a cycle, then we can
    /// push it onto the stack so that it becomes the new top, and
    /// call `find_next_trace` to start searching its children.
    fn push_next_child(&mut self, index: NodeIndex, prefix: &'grammar [Symbol]) -> bool {
        println!("push(index={:?}, prefix={:?})", self.graph.graph[index], prefix);

        self.symbols.extend(prefix.iter().cloned().rev());

        match self.graph.graph[index] {
            TraceGraphNode::Item(item) => {
                // If we reached an item like
                //
                //     X = ...p (*) ...s
                //
                // then we are done, but we still need to push on the
                // symbols `...p`.
                self.found_trace(item)
            }
            TraceGraphNode::Nonterminal(_) => {
                // If this node already appears on the stack, do not
                // visit its children.
                if !self.stack.iter().any(|state| state.index == index) {
                    let edges = self.incoming_edges(index);
                    self.stack.push(EnumeratorState {
                        index: index,
                        symbols_len: self.symbols.len(),
                        edges: edges,
                    });
                }
                self.find_next_trace()
            }
        }
    }

    /// In between iterations, we keep the symbols in reverse order so
    /// that people can read from them.
    fn found_trace(&mut self, item: LR0Item<'grammar>) -> bool {
        println!("found_trace(item={:?})", item);

        // the end point of a trace is always some place we could have
        // shifted a nonterminal
        assert!(item.can_shift());
        self.symbols.extend(item.prefix().iter().cloned().rev());
        self.symbols.reverse();

        // mirror the cursor around
        self.cursor = self.symbols.len() - self.cursor;

        // when we resume, we will want to immediately truncate off
        // the tail of symbols we are about to push on the end
        self.reset = self.symbols.len();

        self.symbols.extend(&item.production.symbols[item.index+1..]);

        println!("found_trace: symbols={:?} cursor={:?} reset={:?}",
                 self.symbols, self.cursor, self.reset);
        true
    }

    /// Return the symbols of the current trace, or None if there is
    /// no current trace.
    pub fn symbols_and_cursor(&self) -> Option<(&[Symbol], usize)> {
        if self.stack.is_empty() {
            None
        } else {
            Some((&self.symbols[..], self.cursor))
        }
    }
}

impl<'graph, 'grammar> Iterator for PathEnumerator<'graph, 'grammar> {
    type Item = (Vec<Symbol>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let this =
            self.symbols_and_cursor()
                .map(|(symbols, cursor)| (symbols.to_vec(), cursor));
        self.advance();
        this
    }
}
