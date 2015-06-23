//! The NFA we construct for each regex. Since the states are not
//! really of interest, we represent this just as a vector of labeled
//! edges.

use super::re::{Regex, Alternative, Elem, RepeatOp, Test};
use std::cmp;
use std::usize;

#[cfg(test)]

mod test;

pub struct NFA {
    states: Vec<State>,
    edges: Edges
}

// An "epsilon" edge -- no input
#[derive(Debug, PartialEq, Eq)]
pub struct Noop;

// An "other" edge -- fallback if no other edges apply
#[derive(Debug, PartialEq, Eq)]
pub struct Other;

/// For each state, we just store the indices of the first char and
/// test edges, or usize::MAX if no such edge. You can then find all
/// edges by enumerating subsequent edges in the vectors until you
/// find one with a different `from` value.
struct State {
    first_noop_edge: usize,
    first_test_edge: usize,
    first_other_edge: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StateIndex(usize);

const INVALID_STATE_INDEX: StateIndex = StateIndex(usize::MAX);

/// A set of edges for the state machine. Edges are kept sorted by the
/// type of label they have. Within a vector, all edges with the same
/// `from` are grouped together so they can be enumerated later (for
/// now we just ensure this during construction, but one could easily
/// sort).
struct Edges {
    noop_edges: Vec<Edge<Noop>>,
    test_edges: Vec<Edge<Test>>,
    other_edges: Vec<Edge<Other>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge<L> {
    pub from: StateIndex,
    pub label: L,
    pub to: StateIndex,
}

pub const ACCEPT: StateIndex = StateIndex(0);
pub const REJECT: StateIndex = StateIndex(1);
pub const START: StateIndex = StateIndex(2);

impl NFA {
    pub fn from_re(regex: &Regex) -> NFA {
        let mut nfa = NFA::new();
        let s0 = nfa.regex(regex, ACCEPT, REJECT);
        nfa.push_edge(s0, Noop, START);
        nfa
    }

    ///////////////////////////////////////////////////////////////////////////
    // Public methods for querying an NFA

    pub fn edges<L:EdgeLabel>(&self, from: StateIndex) -> EdgeIterator<L> {
        assert!(from.0 < self.states.len(),
                "invalid state index {:?}, max {:?}", from, self.states.len());

        let vec = L::vec(&self.edges);
        let first = *L::first(&self.states[from.0]);
        EdgeIterator { edges: vec, from: from, index: first }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Private methods for building an NFA

    fn new() -> NFA {
        let mut nfa = NFA {
            states: vec![],
            edges: Edges {
                noop_edges: vec![],
                test_edges: vec![],
                other_edges: vec![],
            }
        };

        // reserve the ACCEPT, REJECT, and START states ahead of time
        assert!(nfa.new_state() == ACCEPT);
        assert!(nfa.new_state() == REJECT);
        assert!(nfa.new_state() == START);

        nfa
    }

    fn new_state(&mut self) -> StateIndex {
        let index = self.states.len();

        // these edge indices will be patched later by patch_edges()
        self.states.push(State { first_noop_edge: usize::MAX,
                                 first_test_edge: usize::MAX,
                                 first_other_edge: usize::MAX });

        StateIndex(index)
    }

    // pushes an edge: note that all outgoing edges from a particular
    // state should be pushed together, so that the edge vectors are
    // suitably sorted
    fn push_edge<L:EdgeLabel>(&mut self, from: StateIndex, label: L, to: StateIndex) {
        let edge_vec = L::vec_mut(&mut self.edges);
        let edge_index = edge_vec.len();
        edge_vec.push(Edge { from: from, label: label, to: to });

        // if this is the first edge from the `from` state, set the
        // index
        let first_index = L::first_mut(&mut self.states[from.0]);
        if *first_index == usize::MAX {
            *first_index = edge_index;
        } else{
            // otherwise, check that all edges are continuous
            assert_eq!(edge_vec[edge_index - 1].from, from);
        }
    }

    fn regex(&mut self, regex: &Regex, accept: StateIndex, reject: StateIndex) -> StateIndex {
        match regex.alternatives.len() {
            0 => accept, // matches the empty string
            1 => self.alternative(&regex.alternatives[0], accept, reject),
            _ => {
                // NB -- it is important that we *collect* into a
                // vector, because we don't want to intersperse
                // compiling each alternative with adding the edges
                // below
                let alt_starts: Vec<_> =
                    regex.alternatives.iter()
                                      .map(|alt| self.alternative(alt, accept, reject))
                                      .collect();

                let start = self.new_state();
                for alt_start in alt_starts {
                    self.push_edge(start, Noop, alt_start);
                }

                start
            }
        }
    }

    fn alternative(&mut self, alt: &Alternative, accept: StateIndex, reject: StateIndex)
                   -> StateIndex {
        // build our way from the back
        let mut p = accept;
        for elem in alt.elems.iter().rev() {
            p = self.elem(elem, p, reject);
        }
        p
    }

    fn elem(&mut self, elem: &Elem, accept: StateIndex, reject: StateIndex) -> StateIndex {
        match *elem {
            Elem::Test(test) => {
                // [s0] -----c---> [accept]
                //   |
                //   +-otherwise-> [reject]

                let s0 = self.new_state();
                self.push_edge(s0, test, accept);
                self.push_edge(s0, Other, reject);

                s0
            }

            Elem::Group(ref regex) => {
                self.regex(regex, accept, reject)
            }

            Elem::NotGroup(ref regex) => {
                self.regex(regex, reject, accept) // NB: swapped accept/reject here :)
            }

            Elem::Repeat(RepeatOp::Question, ref elem) => {
                // [s0] ----> [accept]
                //   |           ^
                //   v           |
                // [s1] --...----+
                //         |
                //         v
                //      [reject]

                let s1 = self.elem(elem, accept, reject);

                let s0 = self.new_state();
                self.push_edge(s0, Noop, accept); // they might supply nothing
                self.push_edge(s0, Noop, s1);

                s0
            }

            Elem::Repeat(RepeatOp::Star, ref elem) => {
                // [s0] ----> [accept]
                //  | ^
                //  | |
                //  | +----------+
                //  v            |
                // [s1] --...----+
                //         |
                //         v
                //      [reject]

                let s0 = self.new_state();

                let s1 = self.elem(elem, s0, reject);

                self.push_edge(s0, Noop, accept);
                self.push_edge(s0, Noop, s1);

                s0
            }

            Elem::Repeat(RepeatOp::Plus, ref elem) => {
                //            [accept]
                //               ^
                //               |
                //    +----------+
                //    v          |
                // [s0] --...--[s1]
                //         |
                //         v
                //      [reject]

                let s1 = self.new_state();
                self.push_edge(s1, Noop, accept);

                let s0 = self.elem(elem, s1, reject);

                s0
            }
        }
    }
}

pub trait EdgeLabel {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Self>>;
    fn vec(nfa: &Edges) -> &Vec<Edge<Self>>;
    fn first_mut(state: &mut State) -> &mut usize;
    fn first(state: &State) -> &usize;
}

impl EdgeLabel for Noop {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Noop>> { &mut nfa.noop_edges }
    fn first_mut(state: &mut State) -> &mut usize { &mut state.first_noop_edge }
    fn vec(nfa: &Edges) -> &Vec<Edge<Noop>> { &nfa.noop_edges }
    fn first(state: &State) -> &usize { &state.first_noop_edge }
}

impl EdgeLabel for Other {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Other>> { &mut nfa.other_edges }
    fn first_mut(state: &mut State) -> &mut usize { &mut state.first_other_edge }
    fn vec(nfa: &Edges) -> &Vec<Edge<Other>> { &nfa.other_edges }
    fn first(state: &State) -> &usize { &state.first_other_edge }
}

impl EdgeLabel for Test {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Test>> { &mut nfa.test_edges }
    fn first_mut(state: &mut State) -> &mut usize { &mut state.first_test_edge }
    fn vec(nfa: &Edges) -> &Vec<Edge<Test>> { &nfa.test_edges }
    fn first(state: &State) -> &usize { &state.first_test_edge }
}

pub struct EdgeIterator<'nfa,L:EdgeLabel+'nfa> {
    edges: &'nfa [Edge<L>],
    from: StateIndex,
    index: usize,
}

impl<'nfa,L:EdgeLabel> Iterator for EdgeIterator<'nfa,L> {
    type Item = &'nfa Edge<L>;

    fn next(&mut self) -> Option<&'nfa Edge<L>> {
        let index = self.index;
        if index == usize::MAX {
            return None;
        }

        let next_index = index + 1;
        if next_index >= self.edges.len() || self.edges[next_index].from != self.from {
            self.index = usize::MAX;
        } else {
            self.index = next_index;
        }

        Some(&self.edges[index])
    }
}
