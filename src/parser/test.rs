use super::*;

#[test]
fn type_name() {
    let x = parse_type_name("parser::Enum<'l,T>");
    assert_eq!(x.reference(), "::parser::Enum<'l, T>");
}

#[test]
fn empty_grammar() {
    assert!(parse_grammar(r#"grammar Foo { }"#).is_ok());
}

#[test]
fn nonterminal0() {
    assert!(parse_grammar(r#"grammar Foo { Expr = Alt; }"#).is_ok());
}

#[test]
fn incorrect_paren() {
    assert!(parse_grammar(r#"grammar Foo { Expr = (Alt); }"#).is_err());
}

#[test]
fn paren_with_plus() {
    assert!(parse_grammar(r#"grammar Foo { Expr = (Alt)+; }"#).is_ok());
}

#[test]
fn paren_with_plus_and_anon() {
    assert!(parse_grammar(r#"grammar Foo { Expr = (<Alt>)+; }"#).is_ok());
}

#[test]
fn named_choice() {
    assert!(parse_grammar(r#"grammar Foo { Expr = <n:Alt>; }"#).is_ok());
}

#[test]
fn named_choice_plus() {
    assert!(parse_grammar(r#"grammar Foo { Expr = <n:Alt+>; }"#).is_ok());
}

