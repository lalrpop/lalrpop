use intern::intern;
use grammar::repr::*;
use test_util::{expect_debug, normalized_grammar};
use lr1::build;
use lr1::core::*;
use lr1::interpret;
use lr1::tls::Lr1Tls;
use tls::Tls;

use super::lane::*;
use super::table::*;

fn sym(t: &str) -> Symbol {
    if t.chars().next().unwrap().is_uppercase() {
        Symbol::Nonterminal(nt(t))
    } else {
        Symbol::Terminal(term(t))
    }
}

fn term(t: &str) -> TerminalString {
    TerminalString::Literal(TerminalLiteral::Quoted(intern(t)))
}

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn traverse(states: &[LR0State], tokens: &[&str]) -> StateIndex {
    interpret::interpret_partial(states, tokens.iter().map(|&s| term(s))).unwrap().pop().unwrap()
}

/// A simplified version of the paper's initial grammar; this version
/// only has one inconsistent state (the same state they talk about in
/// the paper).
pub fn paper_example_small() -> Grammar {
    normalized_grammar(r#"
grammar;

pub G: () = {
    X "c",
    Y "d",
};

X: () = {
    "e" X,
    "e",
};

Y: () = {
    "e" Y,
    "e"
};
"#)
}

fn build_table<'grammar>(grammar: &'grammar Grammar,
                         goal: &str,
                         tokens: &[&str])
                         -> LaneTable<'grammar> {
    let lr0_err = build::build_lr0_states(&grammar, nt(goal)).unwrap_err();

    // Push the `tokens` to find the index of the inconsistent state
    let inconsistent_state_index = traverse(&lr0_err.states, tokens);
    assert!(lr0_err.conflicts.iter().any(|c| c.state == inconsistent_state_index));
    let inconsistent_state = &lr0_err.states[inconsistent_state_index.0];
    println!("inconsistent_state={:#?}", inconsistent_state.items);

    // Extract conflicting items and trace the lanes, constructing a table
    let conflicting_items = super::conflicting_items(inconsistent_state);
    println!("conflicting_items={:#?}", conflicting_items);
    let mut tracer = LaneTracer::new(&grammar, &lr0_err.states, conflicting_items.len());
    for (i, &conflicting_item) in conflicting_items.iter().enumerate() {
        tracer.start_trace(inconsistent_state.index,
                           ConflictIndex::new(i),
                           conflicting_item);
    }

    tracer.into_table()
}

#[test]
fn small_conflict_1() {
    let _tls = Tls::test();
    let grammar = paper_example_small();
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let table = build_table(&grammar, "G", &["e"]);
    println!("{:#?}", table);
    expect_debug(&table,
                 r#"
| State | C0    | C1    | C2    | C3    | C4    | C5    | Successors |
| S0    |       | ["c"] |       |       | ["d"] |       | {S3}       |
| S3    | ["e"] | []    | ["e"] | ["e"] | []    | ["e"] | {S3}       |
"#
                     .trim_left());
}

pub fn paper_example_large() -> Grammar {
    normalized_grammar(r#"
grammar;

pub G: () = {
    "x" W "a",
    "x" V "t",
    "y" W "b",
    "y" V "t",
    "z" W "r",
    "z" V "b",
    "u" U X "a",
    "u" U Y "r",
};

W: () = {
    U X C
};

V: () = {
    U Y "d"
};

X: () = {
    "k" "t" U X P,
    "k" "t"
};

Y: () = {
    "k" "t" U Y "u",
    "k" "t"
};

U: () = {
    U "k" "t",
    "s"
};

E: () = {
    "a",
    "b",
    "c",
    "v",
};

C: () = {
    "c",
    "w"
};

P: () = {
    "z"
};
"#)
}

#[test]
fn large_conflict_1() {
    let _tls = Tls::test();
    let grammar = paper_example_large();
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let table = build_table(&grammar, "G", &["x", "s", "k", "t"]);
    println!("{:#?}", table);
    expect_debug(&table,
                 r#"
| State | C0    | C1    | C2         | C3    | Successors |
| S1    | ["k"] |       |            |       | {S5}       |
| S2    | ["k"] |       |            |       | {S7}       |
| S3    | ["k"] |       |            |       | {S7}       |
| S4    | ["k"] |       |            |       | {S7}       |
| S5    |       |       | ["a"]      | ["r"] | {S16}      |
| S7    |       |       | ["c", "w"] | ["d"] | {S16}      |
| S16   |       |       |            |       | {S27}      |
| S27   | ["k"] | ["s"] |            |       | {S32}      |
| S32   |       |       | ["z"]      | ["u"] | {S16}      |
"#
                 .trim_left());

    // ^^ This differs in some particulars from what appears in the
    // paper, but I believe it to be correct, and the paper to be wrong.
    //
    // Here is the table using the state names from the paper. I've marked
    // the differences with `(*)`.
    //
    // | State | pi1   | pi2   | pi3        | Successors |
    // | B     | ["k"] |       | *1         | {G}        |
    // | C     | ["k"] |       | *1         | {G}        |
    // | D     | ["k"] |       | *1         | {G}        |
    // | E     | ["k"] |       |            | {F}        |
    // | F     |       | ["r"] | ["a"]      | {H}        |
    // | G     |       | ["d"] | ["c", "w"] | {H}        |
    // | H     |       |       |            | {I}        |
    // | I     | ["k"] |       |            | {J}        |
    // | J     |       | ["u"] | ["z"] *2   | {H}        |
    //
    // *1 - the paper lists "a", "b", and "r" here as lookaheads.  We
    // do not. This is because when we trace back pi3, we never reach
    // those states, as we have already acquired the necessary token
    // of context earlier. I can imagine a distinct lane tracing
    // algorithm that considers *sets* of conflicts and only
    // terminates when all sets have context, but it's much more
    // complex to implement, and seems to add little value.
    //
    // *2 - the paper does not list this context, and yet it seems to
    // present. If you trace back "t" and "k" you reach state J which
    // has the item "X = k t (*)". This "unepsilons" to "X = k t U (*)
    // X P", and the lookahead from the "X" here is FIRST(P) which is
    // "z".
}
