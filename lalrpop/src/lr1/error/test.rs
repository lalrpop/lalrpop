use intern::intern;
use grammar::repr::*;
use lr1::build_states;
use test_util::normalized_grammar;
use tls::Tls;

use super::{ConflictClassification, ErrorReportingCx};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

#[test]
fn priority_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(r#"
grammar;
pub Ty: () = {
    "int" => (),
    "bool" => (),
    <t1:Ty> "->" <t2:Ty> => (),
};
"#);
    let states = build_states(&grammar, nt("Ty")).unwrap_err().states;
    let mut cx = ErrorReportingCx::new(&grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|state| {
                  state.conflicts.iter().flat_map(move |(l, cs)| {
                      cs.iter().map(move |c| (l, c))
                  })
              })
              .next()
              .unwrap();

    println!("lookahead={:?} conflict={:?}",
             lookahead, conflict);

    match cx.classify(lookahead, conflict) {
        ConflictClassification::Precedence {
            shift,
            reduce,
            nonterminal,
        } => {
            println!("shift={:#?}, reduce={:#?}, nonterminal={:?}",
                     shift, reduce, nonterminal);
            assert_eq!(shift.symbols.len(), 5); // Ty -> Ty -> Ty
            assert_eq!(shift.cursor, 3); // Ty -> Ty -> Ty
            assert_eq!(shift.symbols, reduce.symbols);
            assert_eq!(shift.cursor, reduce.cursor);
            assert_eq!(nonterminal, nt("Ty"));
        }
        r => panic!("wrong classification {:#?}", r)
    }
}

#[test]
fn expr_braced_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(r#"
grammar;
pub Expr: () = {
    "Id" => (),
    "Id" "{" "}" => (),
    "Expr" "+" "Id" => (),
    "if" Expr "{" "}" => (),
};
"#);
    let states = build_states(&grammar, nt("Expr")).unwrap_err().states;
    let mut cx = ErrorReportingCx::new(&grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|state| {
                  state.conflicts.iter().flat_map(move |(l, cs)| {
                      cs.iter().map(move |c| (l, c))
                  })
              })
              .next()
              .unwrap();

    println!("lookahead={:?} conflict={:?}",
             lookahead, conflict);

    match cx.classify(lookahead, conflict) {
        ConflictClassification::InsufficientLookahead { .. } => { }
        r => panic!("wrong classification {:#?}", r)
    }
}

#[test]
fn suggest_question_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(r#"
        grammar;

        pub E: () = {
            "L",
            "&" OPT_L E
        };

        OPT_L: () = {
            (),
            "L"
        };
"#);
    let states = build_states(&grammar, nt("E")).unwrap_err().states;

    let mut cx = ErrorReportingCx::new(&grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|state| {
                  state.conflicts.iter().flat_map(move |(l, cs)| {
                      cs.iter().map(move |c| (l, c))
                  })
              })
              .next()
              .unwrap();

    println!("lookahead={:?} conflict={:?}",
             lookahead, conflict);

    match cx.classify(lookahead, conflict) {
        ConflictClassification::SuggestQuestion {
            shift: _,
            reduce: _,
            nonterminal,
            symbol,
        } => {
            assert_eq!(nonterminal, nt("OPT_L"));
            assert_eq!(symbol, Symbol::Terminal(TerminalString::Literal(TerminalLiteral::Quoted(intern("L")))));
        }
        r => panic!("wrong classification {:#?}", r)
    }
}

#[test]
fn suggest_inline_conflict() {
    let _tls = Tls::test();
    let grammar = normalized_grammar(r##"
grammar;

pub ImportDecl: () = {
    "import" <Path> ";" => (),
    "import" <Path> "." "*" ";" => (),
};

Path: () = {
    <head: Ident> <tail: ("." <Ident>)*> => ()
};

Ident = r#"[a-zA-Z][a-zA-Z0-9]*"#;
"##);
    let states = build_states(&grammar, nt("ImportDecl")).unwrap_err().states;

    let mut cx = ErrorReportingCx::new(&grammar, &states);
    let (&lookahead, conflict) =
        states.iter()
              .flat_map(|state| {
                  state.conflicts.iter().flat_map(move |(l, cs)| {
                      cs.iter().map(move |c| (l, c))
                  })
              })
              .next()
              .unwrap();

    println!("lookahead={:?} conflict={:?}",
             lookahead, conflict);

    match cx.classify(lookahead, conflict) {
        ConflictClassification::SuggestInline {
            shift: _,
            reduce: _,
            nonterminal,
        } => {
            assert_eq!(nonterminal, nt("Path"));
        }
        r => panic!("wrong classification {:#?}", r)
    }
}
