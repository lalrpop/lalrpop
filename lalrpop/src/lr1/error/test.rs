use intern::intern;
use grammar::repr::*;
use lr1::backtrace::Tracer;
use lr1::build_states;
use lr1::{Item, StateIndex};
use session::Session;
use test_util::{expect_debug, normalized_grammar};

use super::{ConflictClassification, ErrorReportingCx};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

#[test]
fn priority_conflict() {
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
    let cx = ErrorReportingCx::new(&session, &grammar, &states);
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
        let grammar = normalized_grammar(r#"
grammar;
pub Expr: () = {
    "Id" => (),
    "Id" "{" "}" => (),
    "Expr" "+" "Id" => (),
    "if" Expr "{" "}" => (),
};
"#);
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("Expr")).unwrap_err().states;
    let cx = ErrorReportingCx::new(&session, &grammar, &states);
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
fn inline_conflict() {
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
    let session = Session::test();
    let states = build_states(&session, &grammar, nt("E")).unwrap_err().states;

    let cx = ErrorReportingCx::new(&session, &grammar, &states);
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
            shift,
            reduce,
            nonterminal,
            symbol,
        } => {
            assert_eq!(nonterminal, nt("OPT_L"));
            assert_eq!(symbol, Symbol::Terminal(TerminalString::Literal(TerminalLiteral::Quoted(intern("L")))));
        }
        r => panic!("wrong classification {:#?}", r)
    }
}
