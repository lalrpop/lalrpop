use token::nfa::{NFA, Noop, Other};
use token::re::Test;

#[test]
fn edge_iter() {
    let mut nfa = NFA::new();
    let s0 = nfa.new_state();
    let s1 = nfa.new_state();
    let s2 = nfa.new_state();
    let s3 = nfa.new_state();

    nfa.push_edge(s2, Noop, s3);
    nfa.push_edge(s0, Noop, s1);
    nfa.push_edge(s0, Noop, s3);
    nfa.push_edge(s1, Noop, s2);

    // check that if we mixed up the indies between Noop/Other, we'd get wrong thing here
    nfa.push_edge(s0, Other, s2);

    let s0_edges: Vec<_> = nfa.edges::<Noop>(s0).map(|e| e.to).collect();
    let s1_edges: Vec<_> = nfa.edges::<Noop>(s1).map(|e| e.to).collect();
    let s2_edges: Vec<_> = nfa.edges::<Noop>(s2).map(|e| e.to).collect();
    let s3_edges: Vec<_> = nfa.edges::<Noop>(s3).map(|e| e.to).collect();

    let s0_other_edges: Vec<_> = nfa.edges::<Other>(s0).map(|e| e.to).collect();
    let s0_test_edges: Vec<_> = nfa.edges::<Test>(s0).map(|e| e.to).collect();

    assert_eq!(s0_edges, &[s1, s3]);
    assert_eq!(s1_edges, &[s2]);
    assert_eq!(s2_edges, &[s3]);
    assert_eq!(s3_edges, &[]);

    assert_eq!(s0_other_edges, &[s2]);
    assert_eq!(s0_test_edges, &[]);
}
