use intern::intern;
use generate;
use grammar::repr::*;
use test_util::{compare, expect_debug, normalized_grammar};
use super::{State, Items, Lookahead, LR1};
use super::Lookahead::EOF;
use super::interpret::interpret;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

const ITERATIONS: usize = 22;

fn random_test(grammar: &Grammar, states: &[State], start_symbol: NonterminalString) {
    for i in 0..ITERATIONS {
        let input_tree = generate::random_parse_tree(grammar, start_symbol);
        let output_tree = interpret(&states, input_tree.terminals()).unwrap();

        println!("test {}", i);
        println!("input_tree = {}", input_tree);
        println!("output_tree = {}", output_tree);

        compare(output_tree, input_tree);
    }
}

macro_rules! tokens {
    ($($x:expr),*) => {
        vec![$(TerminalString(intern($x))),*].into_iter()
    }
}

fn items<'g>(grammar: &'g Grammar, nonterminal: &str, index: usize, la: Lookahead)
                      -> Items<'g>
{
    let lr1 = LR1::new(&grammar);
    let items =
        lr1.transitive_closure(
            lr1.items(nt(nonterminal), index, la));
    items
}

#[test]
fn start_state() {
    let grammar = normalized_grammar(r#"
grammar {
    token Tok where { };
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1);
        => None;
    };
}
"#);
    let items = items(&grammar, "A", 0, EOF);
    expect_debug(items, r#"[
    A = (*) B "C" [EOF],
    B = (*) "D" ["C"],
    B = (*) ["C"]
]"#);
}

#[test]
fn start_state_1() {
    let grammar = normalized_grammar(r#"
grammar {
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

    expect_debug(items(&grammar, "A", 0, EOF), r#"[
    A = (*) B C [EOF],
    B = (*) "B1" ["C1"],
    B = (*) ["C1"],
    B = (*) "B1" [EOF],
    B = (*) [EOF]
]"#);

    expect_debug(items(&grammar, "A", 1, EOF), r#"[
    A = B (*) C [EOF],
    C = (*) "C1" [EOF],
    C = (*) [EOF]
]"#);
}

#[test]
fn expr_grammar1() {
    let grammar = normalized_grammar(r#"
grammar {
    token Tok where { };

    S: () =
        E => ();

    E: () = {
        E "-" T => ();
        T => ();
    };

    T: () = {
        "N" => ();
        "(" E ")" => ();
    };
}
"#);

    // for now, just test that process does not result in an error
    // and yields expected number of states.
    let states = build_states(&grammar, nt("S")).unwrap();
    assert_eq!(states.len(), 16);

    // execute it on some sample inputs.
    let tree = interpret(&states, tokens!["N", "-", "(", "N", "-", "N", ")"]).unwrap();
    assert_eq!(
        &format!("{:?}", tree)[..],
        r#"[S: [E: [E: [T: "N"]], "-", [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]]]"#);

    // incomplete:
    assert!(interpret(&states, tokens!["N", "-", "(", "N", "-", "N"]).is_err());

    // incomplete:
    assert!(interpret(&states, tokens!["N", "-"]).is_err());

    // unexpected character:
    assert!(interpret(&states, tokens!["N", "-", ")", "N", "-", "N", "("]).is_err());

    // parens first:
    let tree = interpret(&states, tokens!["(", "N", "-", "N", ")", "-", "N"]).unwrap();
    println!("{}", tree);
    assert_eq!(
        &format!("{}", tree)[..],
        r#"[S: [E: [E: [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]], "-", [T: "N"]]]"#);

    // run some random tests
    random_test(&grammar, &states, nt("S"));
}
