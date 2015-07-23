use parser;
use grammar::parse_tree::{Span};
use regex::Regex;

fn check_err(expected_err: &str, grammar: &str) {
    let expected_err = Regex::new(expected_err).unwrap();

    // the string will have a `>>>` and `<<<` in it, which serve to
    // indicate the span where an error is expected.
    let start_index = grammar.find(">>>").unwrap();
    let grammar = grammar.replace(">>>", ""); // remove the `>>>` marker
    let end_index = grammar.rfind("<<<").unwrap();
    let grammar = grammar.replace("<<<", "");

    assert!(start_index <= end_index);

    let parsed_grammar = parser::parse_grammar(&grammar).unwrap();
    match super::validate(&parsed_grammar) {
        Ok(()) => {
            panic!("expected error for grammar");
        }
        Err(err) => {
            assert_eq!(err.span, Span(start_index, end_index));

            assert!(expected_err.is_match(&err.message),
                    "unexpected error text `{}`, did not match `{}`", err.message, expected_err);
        }
    }
}

#[test]
fn unknown_terminal() {
    check_err(
        r#"terminal `"\+"` does not have a pattern defined for it"#,
        r#"grammar; X = X >>>"+"<<<;"#);
}
