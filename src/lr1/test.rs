use intern::intern;
use grammar::repr::*;
use test_util::{expect_debug, normalized_grammar};
use super::{Configuration, Configurations, Lookahead, LR1};
use super::Lookahead::EOF;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn configurations<'g>(grammar: &'g Grammar, nonterminal: &str, index: usize, la: Lookahead)
                      -> Configurations<'g>
{
    let lr1 = LR1::new(&grammar);
    let configurations =
        lr1.transitive_closure(
            lr1.configurations(nt(nonterminal), index, la));
    configurations
}

#[test]
fn start_state() {
    let grammar = normalized_grammar(r#"
grammar Foo {
    token Tok where { };
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1);
        => None;
    };
}
"#);
    let configurations = configurations(&grammar, "A", 0, EOF);
    expect_debug(configurations, r#"[
    A = (*) B "C" [EOF],
    B = (*) "D" ["C"],
    B = (*) ["C"]
]"#);
}

#[test]
fn start_state_1() {
    let grammar = normalized_grammar(r#"
grammar Foo {
    token Tok where { };
    A = B C;
    B: Option<u32> = {
        "B1" => Some(1);
        => None;
    };
    C: Option<u32> = {
        "C1" => Some(1);
        => None;
    };
}
"#);

    expect_debug(configurations(&grammar, "A", 0, EOF), r#"[
    A = (*) B C [EOF],
    B = (*) "B1" ["C1"],
    B = (*) ["C1"],
    B = (*) "B1" [EOF],
    B = (*) [EOF]
]"#);

    expect_debug(configurations(&grammar, "A", 1, EOF), r#"[
    A = B (*) C [EOF],
    C = (*) "C1" [EOF],
    C = (*) [EOF]
]"#);
}
