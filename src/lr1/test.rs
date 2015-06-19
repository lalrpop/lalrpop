use intern::intern;
use grammar::repr::*;
use test_util::{expect_debug, normalized_grammar};
use super::{Items, Lookahead, LR1};
use super::Lookahead::EOF;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
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
grammar Foo {
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
grammar Foo {
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

    let lr1 = LR1::new(&grammar);
    let mut states = lr1.build_states(nt("S"));
    for state in &mut states {
        state.shifts.sort();
        state.gotos.sort();
    }
    expect_debug(&states, r#"[
    State {
        items: [
            S = (*) E [EOF],
            E = (*) E "-" T [EOF],
            E = (*) T [EOF],
            E = (*) E "-" T ["-"],
            E = (*) T ["-"],
            T = (*) "N" [EOF],
            T = (*) "(" E ")" [EOF],
            T = (*) "N" ["-"],
            T = (*) "(" E ")" ["-"]
        ],
        shifts: [
            ("(", S4),
            ("N", S3)
        ],
        gotos: [
            (E, S2),
            (T, S1)
        ]
    },
    State {
        items: [
            E = T (*) [EOF],
            E = T (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            S = E (*) [EOF],
            E = E (*) "-" T [EOF],
            E = E (*) "-" T ["-"]
        ],
        shifts: [
            ("-", S5)
        ],
        gotos: []
    },
    State {
        items: [
            T = "N" (*) [EOF],
            T = "N" (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            T = "(" (*) E ")" [EOF],
            T = "(" (*) E ")" ["-"],
            E = (*) E "-" T [")"],
            E = (*) T [")"],
            E = (*) E "-" T ["-"],
            E = (*) T ["-"],
            T = (*) "N" [")"],
            T = (*) "(" E ")" [")"],
            T = (*) "N" ["-"],
            T = (*) "(" E ")" ["-"]
        ],
        shifts: [
            ("(", S8),
            ("N", S9)
        ],
        gotos: [
            (E, S7),
            (T, S6)
        ]
    },
    State {
        items: [
            E = E "-" (*) T [EOF],
            E = E "-" (*) T ["-"],
            T = (*) "N" [EOF],
            T = (*) "(" E ")" [EOF],
            T = (*) "N" ["-"],
            T = (*) "(" E ")" ["-"]
        ],
        shifts: [
            ("(", S4),
            ("N", S3)
        ],
        gotos: [
            (T, S10)
        ]
    },
    State {
        items: [
            E = T (*) [")"],
            E = T (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            T = "(" E (*) ")" [EOF],
            T = "(" E (*) ")" ["-"],
            E = E (*) "-" T [")"],
            E = E (*) "-" T ["-"]
        ],
        shifts: [
            (")", S12),
            ("-", S11)
        ],
        gotos: []
    },
    State {
        items: [
            T = "(" (*) E ")" [")"],
            T = "(" (*) E ")" ["-"],
            E = (*) E "-" T [")"],
            E = (*) T [")"],
            E = (*) E "-" T ["-"],
            E = (*) T ["-"],
            T = (*) "N" [")"],
            T = (*) "(" E ")" [")"],
            T = (*) "N" ["-"],
            T = (*) "(" E ")" ["-"]
        ],
        shifts: [
            ("(", S8),
            ("N", S9)
        ],
        gotos: [
            (E, S13),
            (T, S6)
        ]
    },
    State {
        items: [
            T = "N" (*) [")"],
            T = "N" (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            E = E "-" T (*) [EOF],
            E = E "-" T (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            E = E "-" (*) T [")"],
            E = E "-" (*) T ["-"],
            T = (*) "N" [")"],
            T = (*) "(" E ")" [")"],
            T = (*) "N" ["-"],
            T = (*) "(" E ")" ["-"]
        ],
        shifts: [
            ("(", S8),
            ("N", S9)
        ],
        gotos: [
            (T, S14)
        ]
    },
    State {
        items: [
            T = "(" E ")" (*) [EOF],
            T = "(" E ")" (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            T = "(" E (*) ")" [")"],
            T = "(" E (*) ")" ["-"],
            E = E (*) "-" T [")"],
            E = E (*) "-" T ["-"]
        ],
        shifts: [
            (")", S15),
            ("-", S11)
        ],
        gotos: []
    },
    State {
        items: [
            E = E "-" T (*) [")"],
            E = E "-" T (*) ["-"]
        ],
        shifts: [],
        gotos: []
    },
    State {
        items: [
            T = "(" E ")" (*) [")"],
            T = "(" E ")" (*) ["-"]
        ],
        shifts: [],
        gotos: []
    }
]"#);

}
