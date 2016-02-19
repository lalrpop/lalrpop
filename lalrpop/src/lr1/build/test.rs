use intern::intern;
use generate;
use grammar::repr::*;
use test_util::{compare, expect_debug, normalized_grammar};
use lr1::core::*;
use lr1::interpret::interpret;
use lr1::lookahead::Lookahead;
use lr1::lookahead::Lookahead::EOF;
use tls::Tls;

use super::{LR1, build_lr1_states};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

const ITERATIONS: usize = 22;

fn random_test<'g>(grammar: &Grammar, states: &'g [State<'g>], start_symbol: NonterminalString) {
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
        vec![$(TerminalString::quoted(intern($x))),*].into_iter()
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
grammar;
    extern { enum Tok { "C" => .., "D" => .. } }
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1),
        () => None
    };
"#);
    let items = items(&grammar, "A", 0, EOF);
    expect_debug(items.vec, r#"[
    A = (*) B "C" [EOF],
    B = (*) ["C"],
    B = (*) "D" ["C"]
]"#);
}

#[test]
fn start_state_1() {
    let grammar = normalized_grammar(r#"
grammar;
extern { enum Tok { "B1" => .., "C1" => .. } }
A = B C;
B: Option<u32> = {
    "B1" => Some(1),
    () => None
};
C: Option<u32> = {
    "C1" => Some(1),
    () => None
};
"#);

    expect_debug(items(&grammar, "A", 0, EOF).vec, r#"[
    A = (*) B C [EOF],
    B = (*) [EOF],
    B = (*) ["C1"],
    B = (*) "B1" [EOF],
    B = (*) "B1" ["C1"]
]"#);

    expect_debug(items(&grammar, "A", 1, EOF).vec, r#"[
    A = B (*) C [EOF],
    C = (*) [EOF],
    C = (*) "C1" [EOF]
]"#);
}

#[test]
fn expr_grammar1() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(r#"
grammar;
    extern { enum Tok { "-" => .., "N" => .., "(" => .., ")" => .. } }

    S: () =
        E => ();

    E: () = {
        E "-" T => (),
        T => ()
    };

    T: () = {
        "N" => (),
        "(" E ")" => ()
    };
"#);

    // for now, just test that process does not result in an error
    // and yields expected number of states.
    let states = build_lr1_states(&grammar, nt("S")).unwrap();
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

#[test]
fn shift_reduce_conflict1() {
    let _tls = Tls::test();

    // This grammar gets a shift-reduce conflict because if the input
    // is "&" (*) "L", then we see two possibilities, and we must decide
    // between them:
    //
    // "&" (*) "L" E
    //  |       |  |
    //  +-------+--|
    //          |
    //          E
    //
    // or
    //
    // "&"      (*) "L"
    //  |            |
    //  |  OPT_L     E
    //  |   |        |
    //  +---+---+----+
    //          |
    //          E
    //
    // to some extent this may be a false conflict, in that inlined
    // rules would address it, but it's an interesting one for
    // producing a useful error message.

    let grammar = normalized_grammar(r#"
        grammar;
        extern { enum Tok { "L" => .., "&" => .., } }
        E: () = {
            "L",
            "&" OPT_L E
        };
        OPT_L: () = {
            (),
            "L"
        };
    "#);

    assert!(build_lr1_states(&grammar, nt("E")).is_err());
}
