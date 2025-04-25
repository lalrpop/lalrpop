use std::rc::Rc;

use crate::file_text::FileText;
use crate::lr1;
use crate::lr1::tls::Lr1Tls;
use crate::session::Session;
use crate::test_util::normalized_grammar;
use crate::tls::Tls;

use super::*;

const GRAMMAR_TEXT: &str = "grammar;
pub A: () = {
    B,
    C
}

B: () = \"b\";
C: () = \"c\";
";

#[test]
fn test_report_generation() {
    let mut output = Vec::<u8>::new();
    let grammar = normalized_grammar(GRAMMAR_TEXT);
    let _tls = Tls::install(
        Rc::new(Session::new()),
        Rc::new(FileText::new("".into(), "".into())),
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let start_nt = grammar.start_nonterminals.keys().next().unwrap().clone();
    generate_report(&mut output, &lr1::build_states(&grammar, start_nt)).unwrap();

    let report = String::from_utf8(output).unwrap();

    assert!(report.contains("Constructed 5 states"));
    for i in 0..5 {
        assert!(report.contains(&format!("State {i}")));
    }
}
