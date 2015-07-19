use super::{is_identifier_start, is_identifier_continue, Tok, Tokenizer};
use super::Tok::*;

fn test(input: &str,
        expected: Vec<(&str, Tok)>)
{
    // use $ to signal EOL because it can be replaced with a single space
    // for spans, and because it applies also to r#XXX# style strings:
    let input = input.replace("$", "\n");

    let tokenizer = Tokenizer::new(&input);
    let len = expected.len();
    for (token, (expected_span, expected_tok)) in tokenizer.zip(expected.into_iter()) {
        println!("token: {:?}", token);
        let expected_start = expected_span.find("~").unwrap();
        let expected_end = expected_span.rfind("~").unwrap() + 1;
        assert_eq!(Ok((expected_start, expected_tok, expected_end)), token);
    }

    let tokenizer = Tokenizer::new(&input);
    assert_eq!(None, tokenizer.skip(len).next());
}

#[test]
fn identifier_start1() {
    assert!(is_identifier_start('f'));
}

#[test]
fn identifier_continue1() {
    assert!(is_identifier_continue('o'));
}

#[test]
fn basic() {
    test("extern foo", vec![
        ("~~~~~~    ", Extern),
        ("       ~~~", Id("foo")),
    ]);
}

#[test]
fn eol_comment() {
    test("extern // This is a comment$ foo", vec![
        ("~~~~~~                          ", Extern),
        ("                             ~~~", Id("foo")),
    ]);
}

#[test]
fn code1() {
    test("=> a(b, c),", vec![
        ("~~~~~~~~~~ ", EqualsGreaterThanCode(" a(b, c)")),
        ("          ~", Comma),
    ]);
}

#[test]
#[should_panic]
fn code_forgot_comma() {
    test("=> a(b, c),", vec![
        ("~~~~~~~~~~ ", EqualsGreaterThanCode(" a(b, c)")),
        // intentionally forget the comma token; this is more of a test of `test`
    ]);
}
