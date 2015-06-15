use super::{parse_type_name, parse_grammar, parse_alternative, parse_symbol};

#[test]
fn type_name() {
    let x = parse_type_name("parser::Enum<'l,T>");
    assert_eq!(x.reference(), "::parser::Enum<'l, T>");
}

#[test]
fn empty_grammar() {
    parse_grammar(r#"grammar Foo { }"#).unwrap();
}

#[test]
fn alternative() {
    parse_alternative(r#"Alt => Bar;"#).unwrap();
}

#[test]
fn symbol() {
    parse_symbol(r#"Alt"#).unwrap();
}

#[test]
fn nonterminal0() {
    parse_grammar(r#"grammar Foo { Expr = Alt; }"#).unwrap();
}

#[test]
fn paren() {
   parse_grammar(r#"grammar Foo { Expr = (Alt); }"#).unwrap();
}

#[test]
fn paren_with_plus() {
    parse_grammar(r#"grammar Foo { Expr = (Alt)+; }"#).unwrap();
}

#[test]
fn paren_with_plus_and_anon() {
    parse_grammar(r#"grammar Foo { Expr = (~Alt)+; }"#).unwrap();
}

#[test]
fn named_choice() {
    parse_grammar(r#"grammar Foo { Expr = ~n:Alt; }"#).unwrap();
}

#[test]
fn named_choice_plus() {
    parse_grammar(r#"grammar Foo { Expr = ~Alt+; }"#).unwrap();
}

#[test]
fn token_expr() {
    parse_grammar(r#"grammar Foo { token Expr where { "foo" => "bar"; }; }"#).unwrap();
}

#[test]
fn map1() {
    parse_grammar(
        r#"grammar Foo { Expr = ~n:Alt+ => { { foo } }; }"#).unwrap();
}

#[test]
fn mapN() {
    parse_grammar(
        r#"grammar Foo { Expr = { Bar => { Baz }; X ~n:Bar => { Y }; }; }"#).unwrap();
}

