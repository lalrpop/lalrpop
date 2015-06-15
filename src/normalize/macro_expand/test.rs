use parser;

use super::expand_macros;

#[test]
fn test_comma() {
    let grammar = parser::parse_grammar("
grammar Foo {
    Comma<E>: Vec<E> =
       ~v:(~E \",\")* ~e:E? =>
           v.into_iter().chain(e.into_iter()).collect();

    Ids = Comma<\"Id\">;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();

    let expected = parser::parse_grammar("
grammar Foo {
    Ids = `Comma<\"Id\">`;

    `Comma<\"Id\">`: Vec<`\"Id\"`> =
        ~v:(~\"Id\" \",\")* ~e:\"Id\"? =>
           v.into_iter().chain(e.into_iter()).collect();
}
").unwrap();

    println!("Actual:");
    println!("{:#?}", actual);
    println!("Expected:");
    println!("{:#?}", expected);

    assert_eq!(actual, expected);
}
