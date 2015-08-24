use parser;
use grammar::parse_tree::Span;
use normalize::resolve::resolve;
use regex::Regex;
use lexer::dfa::interpret;

fn check_err(expected_err: &str,
             grammar: &str) {
    let expected_err = Regex::new(expected_err).unwrap();

    // the string will have a `>>>` and `<<<` in it, which serve to
    // indicate the span where an error is expected.
    let start_index = grammar.find(">>>").unwrap();
    let grammar = grammar.replace(">>>", ""); // remove the `>>>` marker
    let end_index = grammar.rfind("<<<").unwrap();
    let grammar = grammar.replace("<<<", "");

    assert!(start_index <= end_index);

    let parsed_grammar = parser::parse_grammar(&grammar).unwrap();
    let mut parsed_grammar = resolve(parsed_grammar).unwrap();
    match super::validate(&mut parsed_grammar) {
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

fn check_intern_token(grammar: &str,
                      expected_tokens: Vec<(&'static str, &'static str)>)
{
    let parsed_grammar = parser::parse_grammar(&grammar).unwrap();
    let mut parsed_grammar = resolve(parsed_grammar).unwrap();
    super::validate(&mut parsed_grammar).unwrap();
    let intern_token = parsed_grammar.intern_token().unwrap();
    for (input, expected_literal) in expected_tokens {
        let actual_literal =
            interpret::interpret(&intern_token.dfa, input)
            .map(|(index, text)| {
                let literal = intern_token.literals[index.index()];
                (literal, text)
            });
        let actual_literal = format!("{:?}", actual_literal);
        if expected_literal != actual_literal {
            panic!("input `{}` matched `{}` but we expected `{}`",
                   input,
                   actual_literal,
                   expected_literal);
        }
    }
}

#[test]
fn unknown_terminal() {
    check_err(
        r#"terminal `"\+"` does not have a pattern defined for it"#,
        r#"grammar; extern { enum Term { } } X = X >>>"+"<<<;"#);
}

#[test]
fn unknown_id_terminal() {
    check_err(
        r#"terminal `"foo"` does not have a pattern defined for it"#,
        r#"grammar; extern { enum Term { } } X = X >>>"foo"<<<;"#);
}

#[test]
fn tick_input_lifetime_already_declared() {
    check_err(
        r#".*the `'input` lifetime is implicit and cannot be declared"#,
        r#">>>grammar<<< <'input>; X = X "foo";"#);
}

#[test]
fn input_parameter_already_declared() {
    check_err(
        r#".*the `input` parameter is implicit and cannot be declared"#,
        r#">>>grammar<<<(input:u32); X = X "foo";"#);
}

#[test]
fn quoted_literals() {
    check_intern_token(
        r#"grammar; X = X "+" "-" "foo" "(" ")";"#,
        vec![("+", r#"Some(("+", "+"))"#),
             ("-", r#"Some(("-", "-"))"#),
             ("(", r#"Some(("(", "("))"#),
             (")", r#"Some((")", ")"))"#),
             ("foo", r#"Some(("foo", "foo"))"#),
             ("<", r#"None"#)]);
}
