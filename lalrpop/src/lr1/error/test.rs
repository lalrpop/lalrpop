use crate::grammar::repr::*;
use crate::lr1::build_states;
use crate::lr1::tls::Lr1Tls;
use crate::test_util::normalized_grammar;
use crate::tls::Tls;
use string_cache::DefaultAtom as Atom;

use super::{ConflictClassification, ErrorReportingCx};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(Atom::from(t))
}

#[test]
fn priority_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(
        r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt("Ty")).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    let conflict = &conflicts[0][0];

    println!("conflict={conflict:?}");

    match cx.classify(conflict) {
        ConflictClassification::Precedence {
            shift,
            reduce,
            nonterminal,
        } => {
            println!("shift={shift:#?}, reduce={reduce:#?}, nonterminal={nonterminal:?}");
            assert_eq!(shift.symbols.len(), 5); // Ty -> Ty -> Ty
            assert_eq!(shift.cursor, 3); // Ty -> Ty -> Ty
            assert_eq!(shift.symbols, reduce.symbols);
            assert_eq!(shift.cursor, reduce.cursor);
            assert_eq!(nonterminal, nt("Ty"));
        }
        r => panic!("wrong classification {r:#?}"),
    }
}

#[test]
fn expr_braced_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(
        r#"
grammar;
pub Expr: () = {
    "Id" => (),
    "Id" "{" "}" => (),
    "Expr" "+" "Id" => (),
    "if" Expr "{" "}" => (),
};
"#,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt("Expr")).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    let conflict = &conflicts[0][0];

    println!("conflict={conflict:?}");

    match cx.classify(conflict) {
        ConflictClassification::InsufficientLookahead { .. } => {}
        r => panic!("wrong classification {r:#?}"),
    }
}

#[test]
fn suggest_question_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(
        r#"
        grammar;

        pub E: () = {
            "L",
            "&" OPT_L E
        };

        OPT_L: () = {
            (),
            "L"
        };
"#,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt("E")).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    let conflict = &conflicts[0][0];

    println!("conflict={conflict:?}");

    match cx.classify(conflict) {
        ConflictClassification::SuggestQuestion {
            shift: _,
            reduce: _,
            nonterminal,
            symbol,
        } => {
            assert_eq!(nonterminal, nt("OPT_L"));
            assert_eq!(
                symbol,
                Symbol::Terminal(TerminalString::quoted(Atom::from("L")))
            );
        }
        r => panic!("wrong classification {r:#?}"),
    }
}

#[test]
fn suggest_inline_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(
        r##"
grammar;

pub ImportDecl: () = {
    "import" <Path> ";" => (),
    "import" <Path> "." "*" ";" => (),
};

Path: () = {
    <head: Ident> <tail: ("." <Ident>)*> => ()
};

Ident = r#"[a-zA-Z][a-zA-Z0-9]*"#;
"##,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt("ImportDecl")).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    let conflict = &conflicts[0][0];

    println!("conflict={conflict:?}");

    match cx.classify(conflict) {
        ConflictClassification::SuggestInline {
            shift: _,
            reduce: _,
            nonterminal,
        } => {
            assert_eq!(nonterminal, nt("Path"));
        }
        r => panic!("wrong classification {r:#?}"),
    }
}

/// This example used to cause an out-of-bounds error.
#[test]
fn issue_249() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(
        r#"
grammar;

pub Func = StructDecl* VarDecl*;
StructDecl = "<" StructParameter* ">";
StructParameter = "may_dangle"?;
VarDecl = "let";
"#,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt("Func")).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    for conflict in conflicts.iter().flatten() {
        println!("conflict={conflict:?}");
        cx.classify(conflict);
    }
}

fn verify_errors(
    grammar_text: &str,
    pub_state: &str,
    unique_conflicts: usize,
    terminal_count: usize, // Must include Eof.  E.g. a grammar with just one real terminal has a terminal_count of 2
    text: &str,
) {
    use crate::message::{Content, Message};
    use ascii_canvas::AsciiCanvas;
    let _tls = Tls::test_string(grammar_text);
    let grammar = normalized_grammar(grammar_text);
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let err = build_states(&grammar, nt(pub_state)).unwrap_err();
    let mut cx = ErrorReportingCx::new(&grammar, &err.states, &err.conflicts);
    let conflicts = super::token_conflicts(&err.conflicts);
    assert_eq!(conflicts.len(), unique_conflicts); // One group of conflicts
    for conflict in conflicts {
        assert_eq!(conflict.len(), terminal_count); // terminal count
    }

    let mut calls = 0;
    let test_report = |message: Message| -> Result<(), ()> {
        let mut canvas = AsciiCanvas::new(0, message.min_width());
        message.emit(&mut canvas);
        assert!(canvas
            .to_strings()
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .contains(text));
        calls += 1;
        assert!(calls <= unique_conflicts);
        Ok(())
    };

    cx.report_errors(test_report).unwrap();
    assert_eq!(calls, unique_conflicts);
}

#[test]
fn compress_errors() {
    let grammar = r#"
grammar;

pub A: () = {
        "a" B "z",
        "a" C "z",
}

B: () = {
        "b",
        "q"
}

C: () = {
        "c",
        "q"
}
"#;
    verify_errors(grammar, "A", 1, 6, "Ambiguous grammar");
}

#[test]
fn ambiguous_reduction() {
    let grammar = r#"
grammar;

A: () = {
        "a" "c",
        "a" "b"? "c"
}

pub B: () = {
        "x" A "z"
}
"#;
    verify_errors(grammar, "B", 1, 6, "same reduction");
}
