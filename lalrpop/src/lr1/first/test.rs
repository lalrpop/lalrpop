use intern::intern;
use grammar::repr::*;
use lr1::lookahead::Token;
use lr1::lookahead::Token::EOF;
use test_util::{normalized_grammar};
use super::FirstSets;

pub fn nt(t: &str) -> Symbol {
    Symbol::Nonterminal(NonterminalString(intern(t)))
}

pub fn term(t: &str) -> Symbol {
    Symbol::Terminal(TerminalString::quoted(intern(t)))
}

fn la(t: &str) -> Token {
    Token::Terminal(TerminalString::quoted(intern(t)))
}

fn first0(grammar: &Grammar,
          first: &FirstSets,
          symbols: &[Symbol])
          -> Vec<Token>
{
    let v = first.first0(grammar, symbols);
    v.iter(grammar).collect()
}

fn first1(grammar: &Grammar,
          first: &FirstSets,
          symbols: &[Symbol],
          lookahead: Token)
          -> Vec<Token>
{
    let v = first.first1(grammar, symbols, lookahead);
    v.iter(grammar).collect()
}

#[test]
fn basic_first1() {
    let grammar = normalized_grammar(r#"
    grammar;
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1),
        => None
    };
    X = "E"; // intentionally unreachable
"#);
    let first_sets = FirstSets::new(&grammar);

    assert_eq!(
        first1(&grammar, &first_sets, &[nt("A")], EOF),
        vec![la("C"), la("D")]);

    assert_eq!(
        first1(&grammar, &first_sets, &[nt("B")], EOF),
        vec![la("D"), EOF]);

    assert_eq!(
        first1(&grammar, &first_sets, &[nt("B"), term("E")], EOF),
        vec![la("D"), la("E")]);

    assert_eq!(
        first1(&grammar, &first_sets, &[nt("B"), nt("X")], EOF),
        vec![la("D"), la("E")]);
}

#[test]
fn basic_first0() {
    let grammar = normalized_grammar(r#"
    grammar;
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1),
        => None
    };
    X = "E"; // intentionally unreachable
"#);
    let first_sets = FirstSets::new(&grammar);

    assert_eq!(
        first0(&grammar, &first_sets, &[nt("A")]),
        vec![la("C"), la("D")]);

    assert_eq!(
        first0(&grammar, &first_sets, &[nt("B")]),
        vec![la("D"), EOF]);

    assert_eq!(
        first0(&grammar, &first_sets, &[nt("B"), term("E")]),
        vec![la("D"), la("E")]);

    assert_eq!(
        first0(&grammar, &first_sets, &[nt("B"), nt("X")]),
        vec![la("D"), la("E")]);

    assert_eq!(
        first0(&grammar, &first_sets, &[nt("X")]),
        vec![la("E")]);
}
