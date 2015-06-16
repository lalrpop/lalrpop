use regex::Regex;
use parser;
use std::fmt::Debug;

use super::expand_macros;

thread_local! {
    static SPAN: Regex =
        Regex::new(r"Span\([0-9 ,]*\)").unwrap()
}

fn compare<D:Debug>(actual: D, expected: D) {
    println!("Actual:");
    println!("{:#?}", actual);
    println!("Expected:");
    println!("{:#?}", expected);

    let actual = format!("{:?}", actual);
    let expected = format!("{:?}", expected);

    SPAN.with(|span| {
        let actual = span.replace_all(&actual, "Span(..)");
        let expected = span.replace_all(&expected, "Span(..)");
        assert_eq!(actual, expected);
    });
}

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
        ~v:`(~\"Id\" \",\")`* ~e:\"Id\"? =>
           v.into_iter().chain(e.into_iter()).collect();

    `(~\"Id\" \",\")` = ~\"Id\" \",\";
}
").unwrap();

    println!("Actual:");
    println!("{:#?}", actual);
    println!("Expected:");
    println!("{:#?}", expected);

    compare(actual, expected);
}

#[test]
fn test_if_match() {
    let grammar = parser::parse_grammar("
grammar Foo {
    Expr<E> = {
       A if E == \"A*C\";
       B if E ~~ \"^A*C$\";
       C if E != \"A*C\";
       D if E !~ \"^A*C$\";
    };

    Expr1 = Expr<\"A*C\">;
    Expr2 = Expr<\"AAC\">;
    Expr3 = Expr<\"ABC\">;
}
").unwrap();

    let actual = expand_macros(grammar).unwrap();

    let expected = parser::parse_grammar("
grammar Foo {
    Expr1 = `Expr<\"A*C\">`;
    Expr2 = `Expr<\"AAC\">`;
    Expr3 = `Expr<\"ABC\">`;

    `Expr<\"ABC\">` = { C; D; };
    `Expr<\"AAC\">` = { B; C; };
    `Expr<\"A*C\">` = { A; D; };
}
").unwrap();

    println!("Actual:");
    println!("{:#?}", actual);
    println!("Expected:");
    println!("{:#?}", expected);

    compare(actual, expected);
}
