use parser;
use test_util;

fn check_err(expected_err: &str, grammar: &str, span: &str) {
    let parsed_grammar = parser::parse_grammar(&grammar).unwrap();
    let err = super::validate(&parsed_grammar).unwrap_err();
    test_util::check_norm_err(expected_err, span, err);
}

#[test]
fn named_symbols() {
    check_err(
        r#"named symbols \(like `"Num"`\) require a custom action"#,
        r#"grammar; Term = { <n:"Num"> };"#,
        r#"                     ~~~~~    "#);
}

#[test]
fn bad_assoc_type() {
    check_err(
        r#"associated type `Foo` not recognized"#,
        r#"grammar; extern { type Foo = i32; enum Tok { } }"#,
        r#"                       ~~~                      "#);
}

#[test]
fn dup_assoc_type() {
    check_err(
        r#"associated type `Location` already specified"#,
        r#"grammar; extern { type Location = i32; type Location = u32; enum Tok { } }"#,
        r#"                                            ~~~~~~~~                      "#);
}

#[test]
fn lookahead_without_loc_type() {
    check_err(
        r#"lookahead/lookbehind require you to declare the type of a location"#,
        r#"grammar; extern { enum Tok { } } Foo = @L;"#,
        r#"                                       ~~ "#);
}

#[test]
fn multiple_extern_token() {
    check_err(
        r#"multiple extern definitions are not permitted"#,
        r#"grammar; extern { enum Tok { } } extern { enum Tok { } }"#,
        r#"                                 ~~~~~~                 "#);
}
