use intern::intern;
use grammar::repr::*;
use lr1::Lookahead;
use lr1::Lookahead::EOF;
use test_util::{normalized_grammar};
use super::FirstSets;

pub fn nt(t: &str) -> Symbol {
    Symbol::Nonterminal(NonterminalString(intern(t)))
}

pub fn term(t: &str) -> Symbol {
    Symbol::Terminal(TerminalString::quoted(intern(t)))
}

fn la(t: &str) -> Lookahead {
    Lookahead::Terminal(TerminalString::quoted(intern(t)))
}

fn first(first: &FirstSets, symbols: &[Symbol], lookahead: Lookahead) -> Vec<Lookahead> {
    let mut v = first.first(symbols, lookahead);
    v.sort();
    v
}

#[test]
fn basic() {
    let grammar = normalized_grammar(r#"
grammar;
    extern { enum Tok { } }
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1);
        => None;
    };
"#);
    let first_sets = FirstSets::new(&grammar);

    assert_eq!(
        first(&first_sets, &[nt("A")], EOF),
        vec![la("C"), la("D")]);

    assert_eq!(
        first(&first_sets, &[nt("B")], EOF),
        vec![EOF, la("D")]);

    assert_eq!(
        first(&first_sets, &[nt("B"), term("E")], EOF),
        vec![la("D"), la("E")]);
}
