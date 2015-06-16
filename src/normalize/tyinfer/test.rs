use parser;
use normalize::macro_expand::expand_macros;
use normalize::tyinfer::infer_types;
use normalize::test_util::compare;

#[test]
fn test_pairs_and_tokens() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    X = Y Z;
    Y: Foo = \"Hi\";
    Z = \"Ho\";
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    let actual = infer_types(actual).unwrap();

    let expected = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    X: (Foo, Tok) = Y Z;
    Y: Foo = \"Hi\";
    Z: Tok = \"Ho\";
}
").unwrap();

    compare(actual, expected);
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
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    Two<X>: (X, X) = X X;
    Ids = Two<\"Id\">;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    let actual = infer_types(actual).unwrap();

    let expected = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    Ids: (Tok, Tok) = `Two<\"Id\">`;
    `Two<\"Id\">`: (Tok, Tok) = \"Id\" \"Id\";
}
").unwrap();

    compare(actual, expected);
}

#[test]
fn test_macro_expansion_infer() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    Two<X> = X X;
    Ids = Two<\"Id\">;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();
    let actual = infer_types(actual).unwrap();

    let expected = parser::parse_grammar("
grammar Foo {
    token Tok where { };
    Ids: (Tok, Tok) = `Two<\"Id\">`;
    `Two<\"Id\">`: (Tok, Tok) = \"Id\" \"Id\";
}
").unwrap();

    compare(actual, expected);
}
