use intern::intern;
use grammar::repr::*;
use test_util::{expect_debug, normalized_grammar};
use super::{Configuration, Lookahead, LR1};
use super::Lookahead::EOF;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn configurations<'g>(grammar: &'g Grammar, nonterminal: &str, la: Lookahead)
                      -> Vec<Configuration<'g>>
{
    let lr1 = LR1::new(&grammar);
    let configurations =
        lr1.transitive_closure(
            lr1.start_configurations(nt(nonterminal), la));
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
    let configurations = configurations(&grammar, "A", EOF);
    expect_debug(configurations, r#"[
    A = (*) B "C" [EOF],
    B = (*) "D" ["C"],
    B = (*) ["C"]
]"#);
}

