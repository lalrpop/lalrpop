//! Constructs a DFA which picks the longest matching regular
//! expression from the input.

use kernel_set::StateIndex;
use kernel_set::KernelSet;
use std::rc::Rc;
use token::re;
use token::nfa::{self, NFA, NFAStateIndex};
use util::{map, Map, Multimap, Set};

pub struct DFA {
    nfas: Vec<NFA>,
}

struct State {
    item_set: DFAItemSet,
    test_edges: Map<re::Test, DFAStateIndex>,
    other_edge: DFAStateIndex,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct NFAIndex(usize);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DFAStateIndex(usize);

type DFAKernelSet = KernelSet<DFAStateIndex>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct DFAItemSet {
    items: Rc<Vec<Item>>
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    // which regular expression?
    nfa_index: NFAIndex,

    // what state within the NFA are we at?
    nfa_state: NFAStateIndex,
}

impl DFA {
    fn build_states(&self) -> Vec<State> {
        let mut kernel_set = KernelSet::new();
        let mut states = vec![];

        let state_index = self.start_state(&mut kernel_set);
        while let Some(item_set) = kernel_set.next() {
            // collect all the specific tests we expect from any of
            // the items in this state
            let tests: Set<re::Test> =
                item_set.items
                        .iter()
                        .flat_map(|&item| {
                            self.nfa(item)
                                .edges::<re::Test>(item.nfa_state)
                                .map(|edge| edge.label)
                        })
                        .collect();

            // for each specific test, find what happens if we see a
            // character matching that test
            let test_edges: Map<re::Test, DFAStateIndex> =
                tests.iter()
                     .map(|&test| {
                         let items: Vec<_> =
                             item_set.items.iter()
                                           .filter_map(|&item| self.accept_test(item, test))
                                           .collect();

                         // at least one of those items should accept this test
                         assert!(!items.is_empty());

                         (test, kernel_set.add_state(self.transitive_closure(items)))
                     })
                     .collect();

            // Consider what there is some cahracter that doesn't meet
            // any of the tests. In this case, we can just ignore all
            // the test edges for each of the items and just union all
            // the "other" edges -- because if it were one of those
            // test edges, then that transition is represented above.
            let other_transitions: Vec<_> =
                item_set.items.iter()
                              .filter_map(|&item| self.accept_other(item))
                              .collect();

            // we never know the full set
            assert!(!other_transitions.is_empty());

            let other_edge = kernel_set.add_state(self.transitive_closure(other_transitions));

            states.push(State {
                item_set: item_set,
                test_edges: test_edges,
                other_edge: other_edge,
            });
        }

        states
    }

    fn start_state(&self, kernel_set: &mut DFAKernelSet) -> DFAStateIndex {
        // starting state is at the beginning of all regular expressions
        let items: Vec<_> =
            (0..self.nfas.len())
            .map(|i| Item { nfa_index: NFAIndex(i),
                            nfa_state: nfa::START })
            .collect();
        let item_set = self.transitive_closure(items);
        kernel_set.add_state(item_set)
    }

    fn accept_test(&self, item: Item, test: re::Test) -> Option<Item> {
        let nfa = self.nfa(item);

        let matching_test =
            nfa.edges::<re::Test>(item.nfa_state)
               .filter(|edge| edge.label.meets(test))
               .map(|edge| item.to(edge.to));

        let matching_other =
            nfa.edges::<nfa::Other>(item.nfa_state)
               .map(|edge| item.to(edge.to));

        matching_test.chain(matching_other).next()
    }

    fn accept_other(&self, item: Item) -> Option<Item> {
        let nfa = self.nfa(item);
        nfa.edges::<nfa::Other>(item.nfa_state)
            .map(|edge| item.to(edge.to))
            .next()
    }

    fn transitive_closure(&self, mut items: Vec<Item>) -> DFAItemSet {
        let mut observed: Set<Item> = items.iter().cloned().collect();

        let mut counter = 0;
        while counter < items.len() {
            let item = items[counter];
            let derived_states =
                self.nfa(item)
                    .edges::<nfa::Noop>(item.nfa_state)
                    .map(|edge| item.to(edge.to))
                    .filter(|&item| observed.insert(item));
            items.extend(derived_states);
        }

        items.sort();

        DFAItemSet { items: Rc::new(items) }
    }

    fn nfa(&self, item: Item) -> &NFA {
        &self.nfas[item.nfa_index.0]
    }
}

impl StateIndex for DFAStateIndex {
    type Kernel = DFAItemSet;

    fn from(c: usize) -> DFAStateIndex {
        DFAStateIndex(c)
    }
}

impl Item {
    fn to(&self, s: NFAStateIndex) -> Item {
        Item { nfa_index: self.nfa_index, nfa_state: s }
    }
}
