use lr1::core::*;
use grammar::repr::*;
use petgraph::{EdgeDirection, Graph};
use petgraph::graph::NodeIndex;
use std::fmt::{Debug, Formatter, Error};
use util::{Map, map};

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
        if !self.graph.edges_directed(from, EdgeDirection::Outgoing)
                      .any(|(t, &l)| t == to && l == labels)
        {
            self.graph.add_edge(from, to, labels);
        }
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


