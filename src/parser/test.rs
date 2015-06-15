use grammar::parse_tree::TypeRef;

#[test]
fn type_ref() {
    super::parse_type_ref("parser::Enum<'l,T>").unwrap();
}

#[test]
fn type_ref_tuple() {
    super::parse_type_ref("(X,Y)").unwrap();
}

#[test]
fn type_ref_special_case_for_id() {
    let x = super::parse_type_ref("X").unwrap();
    assert!(match x {
        TypeRef::Id(_) => true,
        _ => false
    });
}

#[test]
fn empty_grammar() {
    super::parse_grammar(r#"grammar Foo { }"#).unwrap();
}

#[test]
fn alternative() {
    super::parse_alternative(r#"Alt => Bar;"#).unwrap();
}

#[test]
fn symbol() {
    super::parse_symbol(r#"Alt"#).unwrap();
}

#[test]
fn nonterminal0() {
    super::parse_grammar(r#"grammar Foo { Expr = Alt; }"#).unwrap();
}

#[test]
fn paren() {
   super::parse_grammar(r#"grammar Foo { Expr = (Alt); }"#).unwrap();
}

#[test]
fn paren_with_plus() {
    super::parse_grammar(r#"grammar Foo { Expr = (Alt)+; }"#).unwrap();
}

#[test]
fn paren_with_plus_and_anon() {
    super::parse_grammar(r#"grammar Foo { Expr = (~Alt)+; }"#).unwrap();
}

#[test]
fn named_choice() {
    super::parse_grammar(r#"grammar Foo { Expr = ~n:Alt; }"#).unwrap();
}

#[test]
fn named_choice_plus() {
    super::parse_grammar(r#"grammar Foo { Expr = ~Alt+; }"#).unwrap();
}

#[test]
fn token_expr() {
    super::parse_grammar(r#"grammar Foo { token Expr where { "foo" => "bar"; }; }"#).unwrap();
}

#[test]
fn map1() {
    super::parse_grammar(
        r#"grammar Foo { Expr = ~n:Alt+ => { { foo } }; }"#).unwrap();
}

#[test]
fn mapN() {
    super::parse_grammar(
        r#"grammar Foo { Expr = { Bar => { Baz }; X ~n:Bar => { Y }; }; }"#).unwrap();
}

#[test]
fn macro_symbols() {
    super::parse_symbol(r#"Foo<Baz>"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz">"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+>"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+, "Balooga">"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+, ("Balooga" Potato),>"#).unwrap();
}

#[test]
fn macro_nt() {
    super::parse_nonterminal(
        r#"Comma<E>: Vec<E> = ~v:(~E ",")* ~e:E? => { let mut v = v; v.extend(e.into_iter()); v};"#)
        .unwrap();
}

#[test]
fn cond_nt() {
    super::parse_nonterminal(
        "Foo<E> = {
           X if E == \"Bar\";
           X if E ~~ \"Bar\";
           X if E != \"Bar\";
           X if E !~ \"Bar\";
         };").unwrap();
}

