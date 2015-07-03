use intern::intern;
use parser;
use normalize::macro_expand::expand_macros;
use normalize::tyinfer::infer_types;
use grammar::parse_tree::{NonterminalString, Span};
use grammar::repr::TypeRepr;
use regex::Regex;

fn check_err(expected_err: &str, grammar: &str) {
    let expected_err = Regex::new(expected_err).unwrap();

    // the string will have a `>>>` and `<<<` in it, which serve to
    // indicate the span where an error is expected.
    let start_index = grammar.find(">>>").unwrap();
    let grammar = grammar.replace(">>>", ""); // remove the `>>>` marker
    let end_index = grammar.find("<<<").unwrap();
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
fn unknown_nonterminal() {
    check_err(
        "no definition found for nonterminal `Y`",
        r#"grammar { X = X >>>Y<<<; }"#);
}

#[test]
fn repeated_macro_arg() {
    check_err(
        "multiple macro arguments declared with the name `Y`",
        r#"grammar { >>>X<Y,Y> <<<= "foo"; }"#);
}

#[test]
fn unknown_nonterminal_two() {
    check_err(
        "no definition found for nonterminal `Expr`",
        r#"grammar { Term = { n:"Num" => n.as_num(); "A" ~>>>Expr <<<"B"; }; }"#);
}
