use intern::intern;
use grammar::parse_tree::TerminalLiteral;
use grammar::repr::*;
use lr1::build_states;
use lr1::interpret::interpret_partial;
use lr1::{Item, Lookahead};
use session::Session;
use test_util::{expect_debug, normalized_grammar};

use super::Tracer;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn term(t: &str) -> TerminalString {
    TerminalString::Literal(TerminalLiteral::Quoted(intern(t)))
}

macro_rules! terms {
    ($($t:expr),*) => {
        vec![$(term($t)),*]
    }
}

fn test_grammar1() -> Grammar {
    normalized_grammar(r#"
    grammar;

    pub Start: () = Stmt;

    pub Stmt: () = {
        Exprs ";",
        Exprs
    };

    Exprs: () = {
        Expr,
        Exprs "," Expr
    };

    Expr: () = {
        "Int",
        Expr "+" "Int",
    };
"#)
}

#[test]
fn backtrace1() {
    let grammar = test_grammar1();
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("Start")).unwrap();
    let mut tracer = Tracer::new(&session, &grammar, &states);
    let state_stack = interpret_partial(&states, terms!["Int"]).unwrap();
    let top_state = *state_stack.last().unwrap();

    // Top state will have items like:
    //
    // Expr = "Int" (*) [EOF],
    // Expr = "Int" (*) ["+"],
    // Expr = "Int" (*) [","],
    // Expr = "Int" (*) [";"]
    //
    // Select the last one.
    let semi = Lookahead::Terminal(term(";"));
    let semi_item = states[top_state.0].items.vec.iter()
                                                 .filter(|item| item.lookahead == semi)
                                                 .next()
                                                 .unwrap();

    let backtrace = tracer.backtrace(top_state, *semi_item);

    expect_debug(&backtrace, r#"BacktraceNode {
    item: Expr = "Int" (*) [";"],
    parents: [
        BacktraceNode {
            item: Exprs = (*) Expr [";"],
            parents: [
                BacktraceNode {
                    item: Stmt = (*) Exprs ";" [EOF],
                    parents: []
                }
            ]
        },
        BacktraceNode {
            item: Exprs = Exprs "," (*) Expr [";"],
            parents: [
                BacktraceNode {
                    item: Stmt = (*) Exprs ";" [EOF],
                    parents: []
                }
            ]
        }
    ]
}"#);
}

#[test]
fn backtrace2() {
    let grammar = test_grammar1();
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("Start")).unwrap();
    let mut tracer = Tracer::new(&session, &grammar, &states);
    let state_stack = interpret_partial(&states, terms!["Int"]).unwrap();
    let top_state = *state_stack.last().unwrap();

    // Top state will have items like:
    //
    // Expr = "Int" (*) [EOF],
    // Expr = "Int" (*) ["+"],
    // Expr = "Int" (*) [","],
    // Expr = "Int" (*) [";"]
    //
    // Select the last one.
    let plus = Lookahead::Terminal(term("+"));
    let plus_item = states[top_state.0].items.vec.iter()
                                                 .filter(|item| item.lookahead == plus)
                                                 .next()
                                                 .unwrap();

    let backtrace = tracer.backtrace(top_state, *plus_item);

    println!("{:#?}", backtrace);
    expect_debug(&backtrace, r#"BacktraceNode {
    item: Expr = "Int" (*) ["+"],
    parents: [
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [EOF],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" ["+"],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [","],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [";"],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [EOF],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" ["+"],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [","],
            parents: []
        },
        BacktraceNode {
            item: Expr = (*) Expr "+" "Int" [";"],
            parents: []
        }
    ]
}"#);
}

#[test]
fn backtrace3() {
    // This grammar yields a S/R conflict. Is it (int -> int) -> int
    // or int -> (int -> int)?
    let grammar = normalized_grammar(r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#);
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("Ty")).unwrap_err().states;
    let mut tracer = Tracer::new(&session, &grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|s| &s.conflicts)
              .flat_map(|(l, cs)| cs.iter().map(move |c| (l, c)))
              .next()
              .unwrap();
    let item = Item { production: conflict.production,
                      index: conflict.production.symbols.len(),
                      lookahead: lookahead };
    println!("item={:?}", item);
    let backtrace = tracer.backtrace(conflict.state, item);
    expect_debug(&backtrace, r#"BacktraceNode {
    item: Ty = Ty "->" Ty (*) ["->"],
    parents: [
        BacktraceNode {
            item: Ty = (*) Ty "->" Ty [EOF],
            parents: []
        },
        BacktraceNode {
            item: Ty = (*) Ty "->" Ty ["->"],
            parents: []
        },
        BacktraceNode {
            item: Ty = (*) Ty "->" Ty [EOF],
            parents: []
        },
        BacktraceNode {
            item: Ty = (*) Ty "->" Ty ["->"],
            parents: []
        },
        BacktraceNode {
            item: Ty = Ty "->" (*) Ty ["->"],
            parents: [
                BacktraceNode {
                    item: Ty = (*) Ty "->" Ty [EOF],
                    parents: []
                },
                BacktraceNode {
                    item: Ty = (*) Ty "->" Ty ["->"],
                    parents: []
                },
                BacktraceNode {
                    item: Ty = (*) Ty "->" Ty [EOF],
                    parents: []
                },
                BacktraceNode {
                    item: Ty = (*) Ty "->" Ty ["->"],
                    parents: []
                },
                BacktraceNode {
                    item: Ty = Ty "->" (*) Ty ["->"],
                    parents: []
                }
            ]
        }
    ]
}"#);
}
