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
fn unknown_nonterminal() {
    check_err(
        "no definition found for nonterminal `Y`",
        r#"grammar; X = X >>>Y<<<;"#);
}

#[test]
fn unknown_nonterminal_in_macro_arg() {
    check_err(
        "no definition found for nonterminal `Y`",
        r#"grammar; X = X Id<>>>Y<<<>; Id<T> = T;"#);
}

#[test]
fn unknown_nonterminal_in_repeat_question() {
    check_err(
        "no definition found for nonterminal `Y`",
        r#"grammar; X = >>>Y<<<?;"#);
}

#[test]
fn repeated_macro_arg() {
    check_err(
        "multiple macro arguments declared with the name `Y`",
        r#"grammar; >>>X<Y,Y><<< = "foo";"#);
}

#[test]
fn unknown_nonterminal_two() {
    check_err(
        "no definition found for nonterminal `Expr`",
        r#"grammar; Term = { <n:"Num"> => n.as_num(); "A" <>>>Expr<<<> "B"; };"#);
}

#[test]
fn named_symbols() {
    check_err(
        r#"named symbols \(like `"Num"`\) require a custom action"#,
        r#"grammar; Term = { <n:>>>"Num"<<<>; };"#);
}

#[test]
fn bad_assoc_type() {
    check_err(
        r#"associated type `Foo` not recognized"#,
        r#"grammar; extern token { type >>>Foo<<< = i32; enum Tok { } }"#);
}

#[test]
fn dup_assoc_type() {
    check_err(
        r#"associated type `Location` already specified"#,
        r#"grammar; extern token { type Location = i32;
                                   type >>>Location<<< = u32;
                                   enum Tok { } }"#);
}

#[test]
fn lookahead_without_loc_type() {
    check_err(
        r#"lookahead/lookbehind require you to declare the type of a location"#,
        r#"grammar; extern token { enum Tok { } } Foo = >>>@L<<<;"#);
}

#[test]
fn multiple_extern_token() {
    check_err(
        r#"multiple extern token definitions are not permitted"#,
        r#"grammar; extern token { enum Tok { } } >>>extern token<<< { enum Tok { } }"#);
}
