use parser;
use normalize::normalize;
use normalize::test_util::expect_debug;

#[test]
fn test_comma() {
    let grammar = parser::parse_grammar("
grammar Foo {
    token Tok where { };

    Comma<E>: Vec<E> =
       ~v:(~E \",\")* ~e:E? =>
           v.into_iter().chain(e.into_iter()).collect();

    Ids = Comma<\"Id\">;
}
").unwrap();
    let actual = normalize(grammar).unwrap();
    expect_debug(actual, "foo");
}
