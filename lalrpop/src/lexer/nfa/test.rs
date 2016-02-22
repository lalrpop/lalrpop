use lexer::nfa::{NFA, Noop, Other, StateKind, Test};
use lexer::nfa::interpret::interpret;
use lexer::re::{self};

#[test]
fn edge_iter() {
    let mut nfa = NFA::new();
    let s0 = nfa.new_state(StateKind::Neither);
    let s1 = nfa.new_state(StateKind::Neither);
    let s2 = nfa.new_state(StateKind::Neither);
    let s3 = nfa.new_state(StateKind::Neither);

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

#[test]
fn identifier_regex() {
    let ident = re::parse_regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap();
    println!("{:#?}", ident);
    let nfa = NFA::from_re(&ident);
    println!("{:#?}", nfa);
    assert_eq!(interpret(&nfa, "0123"), None);
    assert_eq!(interpret(&nfa, "hello0123"), Some("hello0123"));
    assert_eq!(interpret(&nfa, "hello0123 abc"), Some("hello0123"));
    assert_eq!(interpret(&nfa, "_0123 abc"), Some("_0123"));
}

#[test]
fn regex_star_group() {
    let ident = re::parse_regex(r#"(abc)*"#).unwrap();
    let nfa = NFA::from_re(&ident);
    assert_eq!(interpret(&nfa, "abcabcabcab"), Some("abcabcabc"));
}

#[test]
fn regex_number() {
    let num = re::parse_regex(r#"[0-9]+"#).unwrap();
    let nfa = NFA::from_re(&num);
    assert_eq!(interpret(&nfa, "123"), Some("123"));
}

#[test]
fn max_range() {
    let num = re::parse_regex(r#"ab{2,4}"#).unwrap();
    let nfa = NFA::from_re(&num);
    assert_eq!(interpret(&nfa, "a"), None);
    assert_eq!(interpret(&nfa, "ab"), None);
    assert_eq!(interpret(&nfa, "abb"), Some("abb"));
    assert_eq!(interpret(&nfa, "abbb"), Some("abbb"));
    assert_eq!(interpret(&nfa, "abbbb"), Some("abbbb"));
    assert_eq!(interpret(&nfa, "abbbbb"), Some("abbbb"));
    assert_eq!(interpret(&nfa, "ac"), None);
}

#[test]
fn literal() {
    let num = re::parse_regex(r#"(?i:aBCdeF)"#).unwrap();
    let nfa = NFA::from_re(&num);
    assert_eq!(interpret(&nfa, "abcdef"), Some("abcdef"));
    assert_eq!(interpret(&nfa, "AbcDEf"), Some("AbcDEf"));
}
