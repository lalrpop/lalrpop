use parser;
use normalize::macro_expand::expand_macros;
use normalize::tyinfer::infer_types;
use normalize::test_util;

fn compare(g1: &str, g2: &str) {
    let actual = parser::parse_grammar(g1).unwrap();
    let actual = expand_macros(actual).unwrap();
    let actual = infer_types(actual).unwrap();

    let expected = parser::parse_grammar(g2).unwrap();

    test_util::compare(actual, expected);
}

#[test]
fn test_pairs_and_tokens() {
    compare("
grammar Foo {
         token Tok where { };
    X = Y Z;
    Y: Foo = \"Hi\";
    Z = \"Ho\";
}
","
grammar Foo {
    token Tok where { };
    X: (Foo, Tok) = Y Z;
    Y: Foo = \"Hi\";
    Z: Tok = \"Ho\";
}
")
}

#[test]
fn test_cycle_direct() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    X = {
        X Y;
        Y;
    };
    Y = \"Hi\";
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    assert!(infer_types(actual).is_err());
}

#[test]
fn test_cycle_indirect() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    A = B;
    B = C;
    C = D;
    D = A;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    assert!(infer_types(actual).is_err());
}

#[test]
fn test_macro_expansion() {
    compare("
grammar Foo {
    token Tok where { };
    Two<X>: (X, X) = X X;
    Ids = Two<\"Id\">;
}
","
grammar Foo {
    token Tok where { };
    Ids: (Tok, Tok) = `Two<\"Id\">`;
    `Two<\"Id\">`: (Tok, Tok) = \"Id\" \"Id\";
}
")
}

#[test]
fn test_macro_expansion_infer() {
    compare("
grammar Foo {
    token Tok where { };
    Two<X> = X X;
    Ids = Two<\"Id\">;
}
","
grammar Foo {
    token Tok where { };
    Ids: (Tok, Tok) = `Two<\"Id\">`;
    `Two<\"Id\">`: (Tok, Tok) = \"Id\" \"Id\";
}
")
}

#[test]
fn test_type_question() {
    compare("
grammar Foo {
    token Tok where { };
    X = Y?;
    Y = \"Hi\";
}
","
grammar Foo {
    token Tok where { };
    X: std::option::Option<Tok> = Y?;
    Y: Tok = \"Hi\";
}
")
}
