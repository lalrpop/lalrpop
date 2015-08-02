use intern::intern;
use generate;
use grammar::repr::*;
use test_util::{compare, expect_debug, normalized_grammar};
use super::lalr_states;
use super::super::interpret::interpret;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

macro_rules! tokens {
    ($($x:expr),*) => {
        vec![$(TerminalString(intern($x))),*].into_iter()
    }
}

#[test]
fn figure9_23() {
    let grammar = normalized_grammar(r#"
        grammar;
        extern token { enum Tok { } }
        S: () = E       => ();
        E: () = {
            E "-" T     => ();
            T           => ();
        };
        T: () = {
            "N"         => ();
            "(" E ")"   => ();
        };
   "#);

    let states = lalr_states(&grammar, nt("S")).unwrap();
    println!("{:#?}", states);

    let tree = interpret(&states, tokens!["N", "-", "(", "N", "-", "N", ")"]).unwrap();
    assert_eq!(
        &format!("{:?}", tree)[..],
        r#"[S: [E: [E: [T: "N"]], "-", [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]]]"#);
}
