use intern::intern;
use grammar::repr::*;
use test_util::{compare, expect_debug, normalized_grammar};
use lr1::build;
use lr1::core::*;
use tls::Tls;

use super::lane::LaneTracer;
use super::table::ConflictIndex;

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

#[test]
fn small_conflict_1() {
    let _tls = Tls::test();
    let grammar = paper_example_small();
    let lr0_states = build::build_lr0_states(&grammar, nt("G")).unwrap_err().states;
    assert_eq!(lr0_states.iter().filter(|s| !s.conflicts.is_empty()).count(), 1);
    let inconsistent_state = lr0_states.iter()
                                       .filter(|s| !s.conflicts.is_empty())
                                       .next()
                                       .unwrap();
    let conflicting_items = super::conflicting_items(inconsistent_state);
    println!("{:#?}", conflicting_items);
    let mut tracer = LaneTracer::new(&grammar, &lr0_states, conflicting_items.len());
    for (i, &conflicting_item) in conflicting_items.iter().enumerate() {
        tracer.start_trace(inconsistent_state.index,
                           ConflictIndex::new(i),
                           conflicting_item);
    }
    let table = tracer.into_table();
    println!("{:#?}", table);
    expect_debug(&table, r#"
| State | C0    | C1    | C2    | C3    | C4    | C5    | Successors |
| S0    | ["c"] | ["c"] | ["c"] | ["d"] | ["d"] | ["d"] |            |
| S3    | []    | []    | []    | []    | []    | []    | {S0, S3}   |
"#.trim_left());
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

